# `cast`

## Features

- [x] `--abi-decode`
- [x] `--calldata-decode`
- [x] `--from-ascii` (with `--from-utf8` alias)
- [ ] `--from-bin`
- [ ] `--from-fix`
- [x] `--from-wei`
- [x] `--max-int`
- [x] `--max-uint`
- [x] `--min-int`
- [x] `--to-checksum-address` (`--to-address` in dapptools)
- [x] `--to-ascii`
- [x] `--to-bytes32`
- [x] `--to-dec`
- [x] `--to-fix`
- [x] `--to-hex`
- [x] `--to-hexdata`
- [ ] `--to-int256`
- [x] `--to-uint256`
- [x] `--to-wei`
- [x] `4byte`
- [x] `4byte-decode`
- [ ] `4byte-event`
- [ ] `abi-encode`
- [x] `age`
- [x] `balance`
- [x] `basefee`
- [x] `block`
- [x] `block-number`
- [ ] `bundle-source`
- [x] `call` (partial)
- [x] `calldata`
- [x] `chain`
- [x] `chain-id`
- [x] `code`
- [ ] `debug`
- [ ] `estimate`
- [ ] `etherscan-source`
- [ ] `events`
- [x] `gas-price`
- [ ] `index`
- [x] `keccak`
- [ ] `logs`
- [x] `lookup-address`
- [ ] `ls`
- [ ] `mktx`
- [x] `namehash`
- [x] `nonce`
- [ ] `publish`
- [ ] `receipt`
- [x] `resolve-name`
- [ ] `run-tx`
- [x] `send` (partial)
- [ ] `sign`
- [x] `storage`
- [x] `tx`

## `cast wallet `

```sh
Set of wallet management utilities

USAGE:
    cast wallet <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    address    Convert a private key to an address
    help       Prints this message or the help of the given subcommand(s)
    new        Create and output a new random keypair
    sign       Sign the message with provided private key
    vanity     Generate a vanity address
    verify     Verify the signature on the message
``` 
