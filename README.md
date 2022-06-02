# Ignite One-time Signer Shim
version 1.0

## Short Description
The Ignite One Time Signer shim acts as translational layer between your Tendermint validators and the Ignite One Time Signer Plugin. It handles RPC communication from the validator node and relays it to plugin using REST API's.

This is a fork of [Tendermint KMS](https://github.com/iqlusioninc/tmkms), Key Management System for [Tendermint] applications such as
[Cosmos Validators].

## Production Fortanix DSM setup
`shim` can be compiled directly from the git repository source code using the
following method.

```
$ git clone <GITHUB_REPO_URL> && cd <CLONED FOLDER>
[...]
$ cargo build --release
```

If successful, this will produce a `shim` executable located at
`./target/release/shim`


### Shim setup

In order to perform setup, `shim` needs a  configuration file which
contains the authentication details needed to authenticate to the DSM with an API key.

This configuration should be placed in a file called: `shim.toml`.
You can specifty the path to the config with either `-c /path/to/shim.toml` or else shim will look in the current working directory for the same file.




The `shim init` command can be used to generate a directory containing
the configuration files needed to run the KMS. Run the following:

```
$ shim init /path/to/kms/home
```

This will output a `shim.toml` file, a `kms-identity.key` (used to authenticate
the KMS to the validator), and create `secrets` and `state` subdirectories.

Please look through `shim.toml` after it's generated, as various sections
will require some customization.


example Toml file : 

```
# shim configuration file

## Chain Configuration

### Cosmos Hub Network

[[chain]]
id = "cosmoshub-3"
key_format = { type = "cosmos-json" }

## Signing Provider Configuration

### Fortanix DSM Signer Configuration

[[providers.fortanixdsm]]
api_endpoint = "https://sdkms.fortanix.com"
api_key = "Nzk5MDQ3ZGUtN2Q2NS00OTRjLTgzMDMtNjQwMTlhYzdmOGUzOlF1SU93ZXJsOFU4VUdEWEdQMmx1dFJOVjlvMTRSd3lhNnVABCNhVkpZOVhzYVgyc0pOVGRQVGJ0RjZJdmVLMy00X05iTEhxMkowamF3UGVPaXJEWEd3"
signing_keys = [
    { chain_ids = ["cosmoshub-3"], type = "consensus", key_id = "72e9ed9e-9eb4-46bd-a135-e78ed9bfd611" },
]
plugin_id="81ce0bfb-3bd8-4214-bed6-b4257f2b9938"

## Validator Configuration

[[validator]]
chain_id = "cosmoshub-3"
addr = "tcp://127.0.0.1:26659"
secret_key = <path to secrete key> # Autogen from shim init
protocol_version = "legacy"# Autogen from shim init
reconnect = true # Autogen from shim init
```
## Running Shim

Validator must be configured to use remote signing with address as in shim.toml. After creading the configuration, start `shim` with the following:

```
$ shim start
```

This will read the configuration from the `shim.toml` file in the current
working directory.

To explicitly specify the path to the configuration, use the `-c` flag:

```
$ shim start -c /path/to/shim.toml
```

## License

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

[//]: # (general links)

[Tendermint]: https://tendermint.com/
[Cosmos Validators]: https://cosmos.network/docs/gaia/