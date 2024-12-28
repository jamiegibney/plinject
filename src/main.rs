#![feature(let_chains)]
#![allow(clippy::wildcard_imports)]

use arguments::Arguments;
use helpers::{
    failure,
    FailureFmt::{ErrorOnly, WithUsageAndExample},
};
use injector::XMLInjector;
use std::{io::BufWriter, process::ExitCode};

mod arguments;
mod errors;
mod helpers;
mod injector;

fn main() -> Result<ExitCode, ExitCode> {
    // Get arguments from the command line
    let args =
        Arguments::from_args().map_err(|e| failure(&e, WithUsageAndExample))?;

    // Try to open a File of the .plist file
    let plist = args.get_plist_file().map_err(|e| failure(&e, ErrorOnly))?;

    // Try to open a File of the .xml file
    let xml = args.get_xml_file().map_err(|e| failure(&e, ErrorOnly))?;

    // Create an output buffer and injector
    let output = BufWriter::new(Vec::new());
    let mut injector = XMLInjector::new(output);

    // Read the source file into the output buffer, injecting the target
    // properties where appropriate
    injector
        .inject(&plist, &xml)
        .map_err(|e| failure(&e, ErrorOnly))?;

    // Write the output file
    args.write(injector.buffer())
        .map_err(|e| failure(&e, ErrorOnly))?;

    // Success!
    args.print_success();
    Ok(ExitCode::SUCCESS)
}
