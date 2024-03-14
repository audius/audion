# audion

Ping the host continuously and write results to a file. The tool
send ICMP echo requests to check if the target is "on".

## Usage

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
	-ms, --timeout <int>  : Timeout in milliseconds
	-i, --interval <int>  : Interval between pings in seconds
	-h, --help            : Show help

Version:
	0.1.0
```

## License

`audion` is licensed under MIT, for more details check the LICENSE file.
