//! This module defines the representation of the input file based on its expected layout:
//!
//! ```
//! |---------------------------------------------------------|
//! | 0x00 * 92 | Img header + data | ... | Img header + data |
//! |---------------------------------------------------------|
//! ```
//!
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};
use std::path::Path;

use crate::img::Img;
use crate::img_header::{ImgHeader, MIN_DATA_LEN};
use crate::local_error::Error;

mod display;

pub struct Input {
    /// Buffer containing the input data
    data: BufReader<File>,
    /// Vector containing the different headers and their offset
    img_parts: Vec<Img>,
    /// Size of the input file
    pub size: u64,
    /// File name we got the data from
    filename: String,
    // /// File containing the input data
    // file: File,
    // /// Pointer to the data
    // data: &'a [u8],
}

impl std::convert::TryFrom<&Path> for Input {
    type Error = String;
    /// Create an instance of Input from a Path
    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file = File::open(path).map_err(|e| format!("{e}"))?;
        let size = file.metadata().map_err(|e| format!("{e}"))?.len();
        let data = BufReader::new(file);
        Ok(Input {
            data,
            size,
            img_parts: Vec::new(),
            filename: format!("{}", path.display()),
        })
    }
}

impl Input {
    /// Validate the input data: returns true if the data is valid, false elsewise.
    ///
    /// The input data must start with 92 bytes containing 0x00.
    pub fn validate(&mut self) -> Result<(), Error> {
        self.data.rewind()?;
        let mut head_content = [0; 92];
        self.data.read_exact(&mut head_content)?;
        if head_content != [0; 92] {
            Err(Error::from("File doesn't contain a valid data header"))
        } else {
            Ok(())
        }
    }

    /// Get the headers of the packed img files.
    ///
    /// Returns a Vec<Img>.
    pub fn parse(&mut self) -> Result<(), Error> {
        let end = self.data.seek(SeekFrom::End(0))?;
        self.data.seek(SeekFrom::Start(92))?;
        let mut offset = self.data.stream_position()?;
        while (offset + MIN_DATA_LEN as u64) < end {
            let mut buf = [0; MIN_DATA_LEN as usize];
            self.data.read_exact(&mut buf)?;
            match ImgHeader::try_from(buf.as_slice()) {
                Ok(header) => {
                    self.img_parts.push(Img::new(header.to_owned(), offset));
                    offset += header.offset();
                }
                Err(_) => {
                    offset += 1;
                }
            }
            self.data.seek(SeekFrom::Start(offset))?;
        }
        let mut remaining: i128 = (end - offset) as i128;
        while remaining > 0 {
            let mut byte = [0; 1];
            self.data.read_exact(&mut byte)?;
            offset += 1;
            remaining -= 1;
            self.data.seek(SeekFrom::Start(offset))?;
        }
        Ok(())
    }

    /// Extract the content of the img files to disk
    pub fn extract(&mut self) -> Result<(), Error> {
        for part in &self.img_parts {}
        unimplemented!()
    }
}
