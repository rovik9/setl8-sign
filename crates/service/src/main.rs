//! The signer daemon: loads configuration and runs the request loop, delegating every
//! decision to `setl8-policy` and every signature to `setl8-keystore`.
//!
//! Listens on a local address only; makes no outbound network connections. Placeholders only.
#![forbid(unsafe_code)]

use std::net::SocketAddr;

/// Daemon configuration.
///
/// TODO(interfaces): config file format and fields are not defined yet.
#[derive(Debug)]
struct Config {
    /// Local listen address. Must be a loopback address.
    #[allow(dead_code)]
    listen: SocketAddr,
}

/// Placeholder: load configuration. Not implemented.
fn load_config() -> Result<Config, &'static str> {
    Err("config loading not implemented")
}

/// Placeholder: accept requests, evaluate policy, sign or refuse. Not implemented.
fn run(_config: Config) -> Result<(), &'static str> {
    Err("request loop not implemented")
}

fn main() {
    match load_config().and_then(run) {
        Ok(()) => {}
        Err(msg) => {
            eprintln!("setl8-signer: {msg}");
            std::process::exit(1);
        }
    }
}
