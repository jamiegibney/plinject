use super::{errors::*, helpers::*};
use std::{fs::File, path::Path};

const PLINJECT_EXECUTABLE_NAME: &str = "plinject";

/// A type for storing and acting upon command line arguments for the `plinject`
/// command.
pub struct Arguments {
    plist: String,
    xml: String,
    target: Option<String>,
}

impl Arguments {
    /// Tries to create a new `Arguments` from [`std::env::args()`], or returns
    /// an error message if the arguments are invalid.
    pub fn from_env() -> Result<Self, String> {
        let mut args = std::env::args();

        // the very first argument is usually the executable path, but this
        // isn't guaranteed. here we panic if it isn't, as any remaining
        // arguments may not follow an expected format.
        // TODO: this should be handled more gracefully
        if let Some(executable_path) = args.next() {
            assert!(
                executable_path.contains(PLINJECT_EXECUTABLE_NAME),
                "{ERR_INVALID_EXECUTABLE_ARG}, but was \"{executable_path}\""
            );
        }

        let first_arg = args.next();

        if first_arg.is_none() {
            return Err(String::from(ERR_NO_ARGS));
        }

        let second_arg = args.next();

        if second_arg.is_none() {
            return Err(String::from(ERR_ONE_ARG));
        }

        unsafe {
            Ok(Self {
                plist: first_arg.unwrap_unchecked(),
                xml: second_arg.unwrap_unchecked(),
                target: args.next(),
            })
        }
    }

    /// Writes the data from the provide buffer to the appropriate target path,
    /// or returns an error message if the write operation failed.
    pub fn write_to_target(&self, buffer: &[u8]) -> Result<(), String> {
        std::fs::write(self.get_target_path(), buffer).map_err(|e| {
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
            fmt_rel_path(Path::new(&self.xml)),
            fmt_rel_path(self.get_target_path())
        );
    }

    /// Returns the appropriate target path from the current arguments.
    pub fn get_target_path(&self) -> &Path {
        self.target
            .as_ref()
            .map_or_else(|| Path::new(&self.plist), |s| Path::new(s))
    }
}
