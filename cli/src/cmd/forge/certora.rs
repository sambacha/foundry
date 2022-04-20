use crate::cmd::Cmd;
use clap::{AppSettings, Parser};
use color_eyre::eyre::{bail, eyre, Context, Result};
use ethers::solc::Solc;
use foundry_config::{Config, SolcReq};
use regex::Regex;
use semver::Version;
use std::{borrow::Cow, collections::HashSet, env, fs};

//TODO: consider ditching this and just use std::process::Command
use xshell::{cmd, Shell};

const MIN_PYTHON_VERSION: Version = Version::new(3, 8, 0);

#[derive(Debug, Clone, Parser)]
#[clap(global_setting = AppSettings::DeriveDisplayOrder)]
pub struct CertoraArgs {
    #[clap(
        help = "Arguments to be passed to Certora command-line interface",
        short = 'a',
        long = "args"
    )]
    cli_args: Option<String>,
    #[clap(help = "Attempt to install the Certora command-line interface", short, long)]
    install_cli: bool,
    #[clap(
        help = "Auto-detect required solc version from project, and add it to args",
        long,
        requires = "cli-args"
    )]
    add_solc_version: bool,
    #[clap(
        help = "Attempt to install the auto-detected solc version",
        long,
        requires = "add-solc-version"
    )]
    install_solc: bool,
}

fn python3_bin(sh: &mut Shell) -> Option<String> {
    let re =
        Regex::new(r"^Python (?P<sem_ver>[2|3]\.[0-9]{1,3}\.[0-9]{1,2})\s*$").expect("regex failure");
    let get_ver_str = |bin: &str| cmd!(sh, "{bin} --version").read().ok();

    let bin_names = ["python3", "python"].map(String::from);

    for bin_name in bin_names {
        if let Some(cap) = get_ver_str(&bin_name).as_deref().and_then(|s| re.captures(s)) {
            let sem_ver = Version::parse(&cap["sem_ver"]).unwrap();
            if sem_ver >= MIN_PYTHON_VERSION {
                return Some(bin_name);
            }
        }
    }

    None
}

fn is_pip3_installed(sh: &mut Shell, python3: &str) -> bool {
    //expects string of form "pip x.y.z from {path_to_pip} (python 3.x)"
    let re = Regex::new(r"^pip.+\(python 3\.[0-9]+\)\s*$").expect("regex failure");
    let matches = |s: String| re.is_match(&s);

    cmd!(sh, "{python3} -m pip --version").read().map(matches).unwrap_or(false)
}

/// based on workaround from https://askubuntu.com/a/588392
fn is_certora_cli_installed(sh: &mut Shell, python3: &str) -> bool {
    let imp = "import certora_cli";

    //since run() returns an Err value on a non-zero exit code,
    //we can simply test the variant of the Result value
    cmd!(sh, "{python3} -c {imp}").quiet().ignore_stderr().run().is_ok()
}

fn add_dirs_to_path(dirs: HashSet<&str>) -> Result<()> {
    const DELIMITER: char = if cfg!(target_os = "windows") { ';' } else { ':' };

    for dir in dirs {
        let cur_path = env::var("PATH")?;
        let new_path = format!("{dir}{DELIMITER}{cur_path}");

        env::set_var("PATH", &new_path);
    }

    Ok(())
}

fn dirs_not_in_path(stderr_output: &str) -> HashSet<&str> {
    let re = Regex::new(r"The script (?P<script_name>[^\s]+) is installed in '(?P<path>[^']+)' which is not on PATH").expect("regex failure");

    re.captures_iter(&stderr_output)
        .map(|warning_line| warning_line.name("path").unwrap().as_str())
        .collect()
}

fn try_install_certora(sh: &mut Shell, python3: &str, cli_args_given: bool) -> Result<()> {
    match (is_pip3_installed(sh, python3), is_certora_cli_installed(sh, python3)) {
        (true, true) => {
            println!("certora-cli is already installed.");
            Ok(())
        }
        (true, false) => {
            println!("Found pip3 installation, installing certora-cli...");

            let output = cmd!(sh, "{python3} -m pip install --user certora-cli")
                .output()
                .wrap_err("certora-cli installation failed.")?;

            let stderr = String::from_utf8(output.stderr)?;
            let dirs = dirs_not_in_path(&stderr);
            if !dirs.is_empty() {
                println!("Some scripts were installed to directories which are not in PATH:");
                for &dir in &dirs {
                    println!("\t{dir}")
                }

                if cli_args_given {
                    println!("These directories will be temporarily added to PATH for this run, but must be manually added for future runs.");
                    add_dirs_to_path(dirs)?;
                } else {
                    println!(
                        "For future runs, make sure you manually add these directories to PATH."
                    );
                }
            }
            Ok(())
        }
        (false, _) => {
            bail!("Python 3 is installed (as {python3}), but pip3 is not installed. Please manually install pip3 and run this tool again.")
        }
    }
}

fn args_with_solc_ver_param(mut certora_run_args: String, required_ver: &Version) -> String {
    let re = Regex::new(r"\s+--solc\s+[^\s]+").expect("regex failed");

    let solc_arg = format!(" --solc {required_ver}");

    match re.replace_all(&certora_run_args, &solc_arg) {
        Cow::Borrowed(_unchanged) => {
            certora_run_args.push_str(&solc_arg);
            certora_run_args
        }
        Cow::Owned(modified) => modified,
    }
}

fn required_solc_ver_from_config(config: &Config) -> Option<Version> {
    //if the project has a required version of solc, try to detect which version it uses
    let solc_req = config.solc.as_ref()?.clone();

    match solc_req {
        SolcReq::Version(ver) => Some(ver),
        SolcReq::Local(solc_path) => Solc::new(solc_path).version().ok(),
    }
}

fn solc_ver_from_project_config() -> Option<Version> {
    let project_config = Config::try_from(Config::figment()).ok()?;

    required_solc_ver_from_config(&project_config)
}

fn solc_ver_from_foundry_toml() -> Option<Version> {
    let toml_path = Config::find_config_file()?;
    let toml_data = fs::read_to_string(toml_path).unwrap();
    let config: Config = toml::from_str(&toml_data).ok()?;

    required_solc_ver_from_config(&config)
}

impl Cmd for CertoraArgs {
    type Output = ();

    fn run(self) -> eyre::Result<Self::Output> {
        let CertoraArgs { install_cli, cli_args, add_solc_version, install_solc } = self;

        let mut sh = Shell::new()?;

        let python3 = python3_bin(&mut sh).ok_or_else(|| {
            eyre!(
                "certora-cli requires Python {MIN_PYTHON_VERSION} or newer. \
                  Install Python {MIN_PYTHON_VERSION} and run this tool again."
            )
        })?;

        if install_cli {
            try_install_certora(&mut sh, &python3, cli_args.is_some())?;
        }

        if let Some(mut cli_args) = cli_args {
            //no need to check installation if install_cli was passed as arg,
            //because we already verified it earlier.
            if !install_cli && !is_certora_cli_installed(&mut sh, &python3) {
                println!("certora-cli not found, attempting to install it automatically...");
                try_install_certora(&mut sh, &python3, true)?;
            }

            if add_solc_version {
                let required_solc_ver = solc_ver_from_project_config()
                    .or_else(solc_ver_from_foundry_toml)
                    .or_else(|| {
                        //as a last-ditch effort, try to detect the most recent solc version.

                        //TODO: I think this is unnecessary because certoraRun already selects the most recent version.
                        None
                    });

                if let Some(ver) = required_solc_ver {
                    println!(
                        "Project seems to require solc version {ver}. Adding this requirement to cli args."
                    );
                    cli_args = args_with_solc_ver_param(cli_args, &ver);

                    if install_solc {
                        println!("Attempting to install solc version {ver}...");
                        let ver_string = ver.to_string();
                        Solc::version_req(&ver_string)
                            .and_then(|req| Solc::ensure_installed(&req))
                            .wrap_err("Failure when attempting to install solc version {ver}.")?;
                    }
                } else {
                    println!(
                        "Unable to detect required solc version. The cli args were not modified."
                    )
                }
            }

            cmd!(sh, "certoraRun {cli_args}").run()?;
        }

        Ok(())
    }
}
