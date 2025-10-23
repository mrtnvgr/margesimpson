# margesimpson

margesimpson is a handy tool to apply config "patches".

The idea is to have modular configuration of the app that
generates its "default" config at first launch.

Supported formats:
- [ini](https://crates.io/crates/rust-ini)

Usage:

```console
> margesimpson -t <target-config> [patch(es)]
```
