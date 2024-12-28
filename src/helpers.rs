use std::{
    fs::File,
    path::{self, Path},
    process::ExitCode,
};

use super::errors::*;

const XML_EXT: &str = "xml";
const PLIST_EXT: &str = "plist";

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum QuitFormat {
    ErrorOnly,
    WithUsage,
    WithExample,
    WithUsageAndExample,
}

pub fn quit(error: &str, format_type: QuitFormat) -> ExitCode {
    let mut s = error.to_string();

    match format_type {
        QuitFormat::ErrorOnly => {}
        QuitFormat::WithUsage => s = format!("{s}\n\n{USAGE}"),
        QuitFormat::WithExample => s = format!("{s}\n\n{EXAMPLE}"),
        QuitFormat::WithUsageAndExample => {
            s = format!("{s}\n\n{USAGE}\n{EXAMPLE}");
        }
    }

    eprintln!("{s}");

    ExitCode::FAILURE
}

pub fn get_args() -> Result<(String, String), String> {
    let mut args = std::env::args();
    _ = args.next();
    let first_arg = args.next();

    if first_arg.is_none() {
        return Err(String::from(NO_ARGS));
    }

    let second_arg = args.next();

    if second_arg.is_none() {
        return Err(String::from(ONE_ARG));
    }

    Ok((first_arg.unwrap(), second_arg.unwrap()))
}

pub fn into_plist(path: &Path) -> Result<File, String> {
    into_file(path, PLIST_EXT, NOT_PLIST)
}

pub fn into_xml(path: &Path) -> Result<File, String> {
    into_file(path, XML_EXT, NOT_XML)
}

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

fn into_file(
    path: &Path,
    file_ext: &str,
    ext_err: &str,
) -> Result<File, String> {
    if !path.exists() {
        return Err(format!("{FILE_MISSING} \"{}\"", absolutize(path)));
    }

    if !path.is_file() {
        return Err(format!("{NOT_A_FILE}: \"{}\"", absolutize(path)));
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
