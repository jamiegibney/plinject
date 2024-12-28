#![feature(let_chains)]
#![allow(clippy::wildcard_imports)]

use helpers::{
    absolutize, get_args, into_plist, into_xml, quit,
    QuitFormat::{ErrorOnly, WithUsageAndExample},
};
use injector::XMLInjector;
use std::{io::BufWriter, path::Path, process::ExitCode};

mod errors;
mod helpers;
mod injector;

fn main() -> ExitCode {
    // Get arguments from the command line
    let args = get_args();
    if let Err(e) = &args {
        return quit(e, WithUsageAndExample);
    }

    let (first, second) = args.unwrap();

    // Try to open a File of the .plist file
    let plist_path = Path::new(&first);
    let plist = into_plist(plist_path);
    if let Err(e) = &plist {
        return quit(e, ErrorOnly);
    }
    let plist = plist.unwrap();

    // Try to open a File of the .xml file
    let xml_path = Path::new(&second);
    let xml = into_xml(xml_path);
    if let Err(e) = &xml {
        return quit(e, ErrorOnly);
    }
    let xml = xml.unwrap();

    // Create an output buffer and injector
    let output = BufWriter::new(Vec::new());
    let mut injector = XMLInjector::new(output);

    // Try to inject the XML into the .plist, writing to the output buffer
    if let Err(e) = &injector.inject(&plist, &xml) {
        return quit(e, ErrorOnly);
    }

    // Try to overwrite the .plist file with the output buffer
    if let Err(e) = std::fs::write(plist_path, injector.buffer()) {
        let err = format!("Error: failed to write file\nDetails: \"{e:?}\"");
        return quit(&err, ErrorOnly);
    }

    println!(
        "Done: injected contents of \"{}\" into \"{}\"",
        absolutize(xml_path),
        absolutize(plist_path)
    );

    ExitCode::SUCCESS
}
