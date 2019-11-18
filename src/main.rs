use clap::{App, Arg};
use std::thread;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

mod jawnfile;

fn main() {
    let matches = App::new("jawn")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("jawnfile")
                .short("j")
                .long("jawnfile")
                .value_name("PATH")
                .help("specify an alternative Jawnfile")
                .takes_value(true),
        )
        .get_matches();

    let jawnfile_path = match matches.value_of("jawnfile").unwrap_or("") {
        "" => "Jawnfile",
        path => path,
    };

    println!("Jawnfile path: {}", jawnfile_path);

    let pb = ProgressBar::new(1024);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:50.cyan/blue}] ({pos}/{len})")
            .progress_chars("#>-"),
    );

    for _i in 0..1000 {
        thread::sleep(Duration::from_millis(5));
        pb.inc(1);
    }

    pb.finish_with_message("jawn complete");
}
