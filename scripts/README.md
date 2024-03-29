## How to use the scripts

This is a simple document that outlines the necessary steps to execute the included scripts


### Run a 5ireChain local 

For running a 5ireChain local, Here are the steps that you need to follow:

Execute the `run-localnet` script:
```sh
./scripts/run-localnet.sh
```

### Generate session keys

Before you run this script, make sure that you installed `subkey`

https://docs.substrate.io/reference/command-line-tools/subkey/

Install by `cargo`:

```bash
cargo install --force subkey --git https://github.com/paritytech/substrate --version 3.0.0 --locked
```

Then:

```sh
./scripts/generate-session-keys.sh
```