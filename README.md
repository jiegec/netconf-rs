# netconf-rs

A Rust library to configure devices with NETCONF(RFC 6241) protocol.

## Features

netconf-rs supports multiple SSH backends through Cargo features:

- **ssh2** (default): Uses the `ssh2` crate for SSH connections
  - Module: `netconf_rs::transport::ssh::SSHTransport`
- **russh**: Uses the `russh` crate (pure Rust SSH implementation)
  - Module: `netconf_rs::transport::russh::RusshTransport`

Enable the russh feature instead of ssh2:

```toml
[dependencies]
netconf-rs = { version = "0.2", default-features = false, features = ["russh"] }
```

Or enable both features:

```toml
[dependencies]
netconf-rs = { version = "0.2", features = ["russh"] }
```

## Transports

Currently, netconf-rs only supports NETCONF over SSH.

## Vendors

Currently, netconf-rs supports the following vendors:

- H3C: tested with S5024E-PWR-X

## License

See LICENSE file.
