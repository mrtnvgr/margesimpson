# margesimpson

Handy tool for splitting up a config file.

The idea is to have modular configuration of apps that
generate its "default" user settings at first launch.

Small named config patches can be easily shared between users.

Can be used to ensure that some settings will be set,
even after unexpected updates or re-installs.

## Supported formats:

- [.ini](https://crates.io/crates/rust-ini)

## Usage:

```console
> margesimpson -t <target-config> [patch(es)]
```
