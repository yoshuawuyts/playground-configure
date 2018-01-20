#[macro_use]
extern crate configure;
extern crate playground_configure;

use playground_configure::Config;

fn main() {
  use_default_config!();
  let foo = Config::default();
  println!("{}", foo.debug);
}
