# audion

Ping a host continuously and write results to a file. The tool
send ICMP Type 8 (echo) requests to check if the target is "on".

`sudo` is required to execute the tool.

## Installation

Clone the repository and use `cargo` to build the package.

For Nix or NixOS users is a [package](https://search.nixos.org/packages?channel=unstable&from=0&size=50&sort=relevance&type=packages&query=audion)
available in Nixpkgs. Keep in mind that the lastest releases might only
be present in the ``unstable`` channel.

```bash
$ nix-env -iA nixos.audion
```

## Usage

The tool requires elevated permissions. Thus, run it with `sudo` or `doas`.

```bash
$ sudo audion -h
Name:
	audion

Author:
	Fabian Affolter <fabian.affolter@audius.de>

Description:
	Ping the host continuously and write results to a file

Usage:
	audion [args]

Flags:
	-t, --target <string> : Target host to ping
	-o, --output <string> : Output file to write results
	-to, --timeout <int>  : Timeout in milliseconds
	-i, --interval <int>  : Interval between pings in seconds
	-h, --help            : Show help

Version:
	0.1.0
```

## Development

Run tests:

```bash
$ nix-shell --run "cargo test"
``` 

Build everything:

```bash
$ nix-shell --run "cargo clean && cargo build"
```

Run the app:

```bash
$ nix-shell --run "cargo run -- --host 127.0.0.1 --port 8080 --wt-port 4433"
```

## License

`audion` is licensed under MIT, for more details check the LICENSE file.
