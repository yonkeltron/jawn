use clap::{App, Arg};

use indicatif::{ProgressBar, ProgressStyle};

mod jawnfile;

use jawnfile::Jawnfile;

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

    let provided_jawnfile_path = matches.value_of("jawnfile").unwrap_or("");
    let jawnfile_path = if provided_jawnfile_path.is_empty() {
        "Jawnfile"
    } else {
        provided_jawnfile_path
    };

    let res = Jawnfile::from_path(jawnfile_path).and_then(|jawnfile| {
        let pb = ProgressBar::new(1024);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:50.cyan/blue}] ({pos}/{len})")
                .progress_chars("#>-"),
        );

        jawnfile.execute(pb);

        Ok("jawn complete")
    });

    match res {
        Ok(msg) => println!("{}", msg),
        Err(err) => eprintln!("{}", err),
    }
}
