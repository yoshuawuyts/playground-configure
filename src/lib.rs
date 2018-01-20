#[macro_use]
extern crate configure;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Deserialize, Configure)]
#[serde(default)]
pub struct Config {
  pub addr: SocketAddr,
  pub debug: bool,
  tls_cert: Option<PathBuf>,
}

impl Default for Config {
  fn default() -> Config {
    Config {
      debug: false,
      addr: "127.0.0.1:7878".parse().unwrap(),
      tls_cert: None,
    }
  }
}
