//! This module contains the Extractor implementation, which is the core logic
//! of the program.
//!
use clap::Parser;

use std::convert::TryFrom;
use std::path::PathBuf;

use crate::input::Input;
use crate::local_error::Error;

/// Extracts the information contained in an UPDATE.APP file from Huawei smartphone
/// firmwares.
#[derive(Parser)]
pub struct Extractor {
    /// The name of the file to extract the img files from.
    ///
    /// Defaults to UPDATE.APP.
    #[clap(value_parser, default_value_os_t = PathBuf::from("UPDATE.APP"))]
    input: PathBuf,
    /// Show content of file instead of extracting.
    #[clap(short = 'C', long, group = "action")]
    show_content: bool,
    /// Show header summary instead of extracting.
    #[clap(short = 'H', long, group = "action")]
    show_headers: bool,
    /// Dump header table into a parseable file.
    #[clap(short, long, group = "action")]
    dump_headers: bool,
}

impl Extractor {
    pub fn run(self) -> Result<(), Error> {
        if !self.input.exists() {
            Err(Error::from(format!(
                "File {} does not exist",
                self.input.display()
            )))
        } else {
            let mut input = Input::try_from(self.input.as_path())?;

            input.validate()?;

            // Parse the input to get img headers
            input.parse()?;

            if self.show_content {
                eprintln!("{input}");
            } else if self.show_headers {
                eprintln!("{}", input.full_table());
            } else if self.dump_headers {
                eprintln!("{}", input.export_csv());
            } else {
                input.extract()?
            }

            Ok(())
        }
    }
}
