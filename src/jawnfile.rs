use anyhow::{anyhow, Result};
use indicatif::ProgressBar;

use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Jawnfile {
    path: String,
    contents: String,
}

impl Jawnfile {
    pub fn from_path(pth: &str) -> Result<Self> {
        let path = Path::new(pth);
        if path.is_file() {
            let contents = fs::read_to_string(&path)?;

            Ok(Jawnfile {
                path: String::from(pth),
                contents: contents,
            })
        } else {
            Err(anyhow!("Invalid Jawnfile path: {}", pth))
        }
    }

    pub fn execute(self, pb: ProgressBar) {
        let jawns_count = self.contents.lines().filter(|line| &"jawn" == line).count();
        for _i in 1..jawns_count {
            thread::sleep(Duration::from_millis(5));
            pb.inc(1);
        }
        pb.finish_with_message("jawn complete");
    }
}
