use super::errors::*;
use std::{
    fs::File,
    io::{BufReader, Read, Seek},
    path::{self, Path},
    process::ExitCode,
};

/// The expected XML file extension.
const XML_EXT: &str = "xml";
/// The expected property list file extension.
const PLIST_EXT: &str = "plist";

/// The `DOCTYPE` tag which is expected in `.plist` files.
const DOCTYPE_TAG: &str = "DOCTYPE";

/// Options for formatting error messages sent to the `stderr` stream on
/// failure.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum FailureFmt {
    // Only a provided error is printed.
    ErrorOnly,
    // The provided error and a usage hint is printed.
    WithUsage,
    // The provided error and an example is printed.
    WithExample,
    // The provided error and both a usage hint and example is printed.
    WithUsageAndExample,
}

/// Prints a formatted error message to the `stderr` stream and returns
/// [`ExitCode::FAILURE`].
pub fn failure(error: &str, format_type: FailureFmt) -> ExitCode {
    let mut s = error.to_string();

    match format_type {
        FailureFmt::ErrorOnly => {}
        FailureFmt::WithUsage => s = format!("{s}\n\n{USAGE}"),
        FailureFmt::WithExample => s = format!("{s}\n\n{EXAMPLES}"),
        FailureFmt::WithUsageAndExample => {
            s = format!("{s}\n\n{USAGE}\n\n{EXAMPLES}");
        }
    }

    eprintln!("{s}");

    ExitCode::FAILURE
}

/// Attempts to find a line containing the [`DOCTYPE_TAG`] within the provided
/// `File`, or returns `None` if not found.
pub fn get_doctype_str(file: &File) -> Option<String> {
    let mut s = String::new();
    let mut reader = BufReader::new(file);

    if reader.read_to_string(&mut s).is_err() {
        return None;
    }

    assert!(
        reader.rewind().is_ok(),
        "Fatal: failed to rewind reader while finding DOCTYPE tag"
    );

    s.lines()
        .find(|&x| x.contains(DOCTYPE_TAG))
        .map(String::from)
}

/// Attempts to format a `Path` into an absolute path as a `String`. If this
/// can't be achieved, `"[unknown path]"` is returned.
pub fn absolutize(path: &Path) -> String {
    if !path.exists() {
        return String::from(path.to_str().unwrap_or("[unknown path]"));
    }

    let abs = path::absolute(path);

    if abs.is_err() {
        return String::from("[unknown path]");
    }

    String::from(abs.unwrap().as_path().to_str().unwrap_or("[unknown_path]"))
}

/// Tries to convert the provided `Path` into a `File` which is guaranteed to be
/// a `.plist` file, or returns an error message if unsuccessful.
pub fn into_plist(path: &Path) -> Result<File, String> {
    into_file(path, PLIST_EXT, ERR_NOT_PLIST_FILE)
}

/// Tries to convert the provided `Path` into a `File` which is guaranteed to be
/// a `.xml` file, or returns an error message if unsuccessful.
pub fn into_xml(path: &Path) -> Result<File, String> {
    into_file(path, XML_EXT, ERR_NOT_XML_FILE)
}

/// Inner implementation for [`into_plist()`] and [`into_xml()`].
fn into_file(
    path: &Path,
    file_ext: &str,
    ext_err: &str,
) -> Result<File, String> {
    if !path.exists() {
        return Err(format!("{ERR_FILE_NOT_FOUND} \"{}\"", absolutize(path)));
    }

    if !path.is_file() {
        return Err(format!("{ERR_UNKNOWN_FILE}: \"{}\"", absolutize(path)));
    }

    let ext = path.extension();

    if ext.is_none() {
        return Err(String::from(ext_err));
    }

    if ext.unwrap() != file_ext {
        return Err(String::from(ext_err));
    }

    File::open(path).map_err(|e| e.to_string())
}
