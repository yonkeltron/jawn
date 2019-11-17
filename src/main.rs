use clap::{App, Arg};

mod jawnfile;

fn main() {
    let matches = App::new("jawn")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();
}
