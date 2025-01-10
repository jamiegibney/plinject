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

// we return this Result to allow the ? operator to be used
fn main() -> Result<ExitCode, ExitCode> {
    // get arguments from the command line
    let args =
        Arguments::from_env().map_err(|e| failure(&e, WithUsageAndExample))?;

    // try to open Files from the plist/xml arguments
    let plist = args.get_plist_file().map_err(|e| failure(&e, ErrorOnly))?;
    let xml = args.get_xml_file().map_err(|e| failure(&e, ErrorOnly))?;

    // create an output buffer and injector
    // we could write directly to the output file, but using a buffer protects
    // the output file in case an error occurs before injection has finished
    let output = BufWriter::new(Vec::new());
    let mut injector = XMLInjector::new(output);

    // read the source file into the output buffer, injecting the target
    // properties where appropriate
    injector
        .inject(&plist, &xml)
        .map_err(|e| failure(&e, ErrorOnly))?;

    // write to the target file
    args.write_to_target(injector.buffer())
        .map_err(|e| failure(&e, ErrorOnly))?;

    // success!
    args.print_success();
    Ok(ExitCode::SUCCESS)
}
