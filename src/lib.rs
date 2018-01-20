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
  addr: SocketAddr,
  tls_cert: Option<PathBuf>,
}

impl Default for Config {
  fn default() -> Config {
    Config {
      addr: "127.0.0.1:7878".parse().unwrap(),
      tls_cert: None,
    }
  }
}
