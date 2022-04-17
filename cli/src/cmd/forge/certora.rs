use crate::cmd::Cmd;
use clap::Parser;
use color_eyre::eyre::{bail, eyre, Context, Result};
use regex::Regex;
use std::{collections::HashSet, env};
use xshell::{cmd, Shell};

const MIN_MAJOR_PYTHON_VER: u32 = 3;
const MIN_MINOR_PYTHON_VER: u32 = 8;

#[derive(Debug, Clone, Parser)]
pub struct CertoraArgs {
    #[clap(help = "Attempt to install the Certora command-line interface", short, long)]
    install: bool,
    #[clap(
        help = "Arguments to be passed to certoraRun, see Certora documentation for more info",
        short,
        long
    )]
    args: Option<String>,
}

fn python3_bin(sh: &mut Shell) -> Option<String> {
    let re = Regex::new(r"^Python (?P<major_ver>2|3)\.(?P<minor_ver>\d{1,2}).+$")
        .expect("regex failure");

    for bin_name in ["python3", "python"] {
        if let Ok(ver_str) = cmd!(sh, "{bin_name} --version").read() {
            if let Some(captures) = re.captures(&ver_str) {
                //these unwraps are safe, since we already validated the formatting by matching the regex.
                let major_ver =
                    captures.name("major_ver").unwrap().as_str().parse::<u32>().unwrap();
                let minor_ver =
                    captures.name("minor_ver").unwrap().as_str().parse::<u32>().unwrap();

                if major_ver > MIN_MAJOR_PYTHON_VER
                    || (major_ver == MIN_MAJOR_PYTHON_VER && minor_ver >= MIN_MINOR_PYTHON_VER)
                {
                    return Some(String::from(bin_name));
                }
            }
        }
    }

    None
}

fn is_pip3_installed(sh: &mut Shell, python3: &str) -> bool {
    //expects string of form "pip x.y.z from {path_to_pip} (python 3.x)"
    let re = Regex::new(r"^pip.+\(python 3\.\d+\)\s*$").expect("regex failure");
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

fn try_install_certora(sh: &mut Shell, python3: &str, run_args_given: bool) -> Result<()> {
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

                if run_args_given {
                    println!("These directories will be temporarily added to PATH for this run, but must be manually added for future runs.");
                    add_dirs_to_path(dirs)?;
                } else {
                    println!("Make sure to add these directories to PATH before attempting to run this script.");
                }
            }
            Ok(())
        }
        (false, _) => {
            bail!("Python 3 is installed (as {python3}), but pip3 is not installed. Please manually install pip3 and run this tool again.")
        }
    }
}

impl Cmd for CertoraArgs {
    type Output = ();

    fn run(self) -> eyre::Result<Self::Output> {
        let mut sh = Shell::new()?;

        let python3 = python3_bin(&mut sh).ok_or_else(|| {
            let req = format!("Python {MIN_MAJOR_PYTHON_VER}.{MIN_MINOR_PYTHON_VER}");
            eyre!("certora-cli requires {req} or newer. Install {req} and run this tool again.")
        })?;

        if self.install {
            try_install_certora(&mut sh, &python3, self.args.is_some())?;
        }

        if let Some(run_arg) = self.args {
            //no need to check installation if install was passed as arg,
            //because we already verified it earlier.
            if !self.install && !is_certora_cli_installed(&mut sh, &python3) {
                println!("certora-cli not found, attempting to install it automatically...");
                try_install_certora(&mut sh, &python3, true)?;
            }

            cmd!(sh, "certoraRun {run_arg}").run()?;
        }

        Ok(())
    }
}
