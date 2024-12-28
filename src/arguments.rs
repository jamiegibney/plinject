use super::{errors::*, helpers::*};
use std::{fs::File, path::Path};

/// A type for storing and acting upon command line arguments for the `plinject`
/// command.
pub struct Arguments {
    pub plist: String,
    pub xml: String,
    pub output: Option<String>,
}

impl Arguments {
    /// Tries to create a new `Arguments` from [`std::env::args()`], or returns
    /// an error message if the arguments are invalid.
    pub fn from_args() -> Result<Self, String> {
        let mut args = std::env::args();
        _ = args.next();
        let first_arg = args.next();

        if first_arg.is_none() {
            return Err(String::from(ERR_NO_ARGS));
        }

        let second_arg = args.next();

        if second_arg.is_none() {
            return Err(String::from(ERR_ONE_ARG));
        }

        Ok(Self {
            plist: first_arg.unwrap(),
            xml: second_arg.unwrap(),
            output: args.next(),
        })
    }

    /// Writes the data from the provide buffer to the appropriate output path,
    /// or returns an error message if the write operation failed.
    pub fn write(&self, buffer: &[u8]) -> Result<(), String> {
        std::fs::write(self.get_output_path(), buffer).map_err(|e| {
            format!("Error: failed to write file\nDetails: \"{e:?}\"")
        })
    }

    /// Tries to create a `File` object for the expected `.plist` file, or
    /// returns an error message if the `File` could not be created.
    pub fn get_plist_file(&self) -> Result<File, String> {
        let path = Path::new(&self.plist);
        into_plist(path)
    }

    /// Tries to create a `File` object for the expected `.xml` file, or
    /// returns an error message if the `File` could not be created.
    pub fn get_xml_file(&self) -> Result<File, String> {
        let path = Path::new(&self.xml);
        into_xml(path)
    }

    /// Prints a success message to the `stdout` stream with the appropriate
    /// paths.
    pub fn print_success(&self) {
        println!(
            "Done: injected contents of \"{}\" into \"{}\"",
            absolutize(Path::new(&self.xml)),
            absolutize(self.get_output_path())
        );
    }

    /// Returns the appropriate output path from the current arguments.
    fn get_output_path(&self) -> &Path {
        self.output
            .as_ref()
            .map_or_else(|| Path::new(&self.plist), |s| Path::new(s))
    }
}
