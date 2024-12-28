pub const USAGE: &str = "Usage:
  $ plinject [destination .plist file] [source .xml file to inject]";
pub const EXAMPLE: &str = "Example:
  $ plinject App.app/Contents/Info.plist injection.xml";

pub const NO_ARGS: &str = "Error: received 0 arguments, expected 2
Help: a destination and source path are required";
pub const ONE_ARG: &str = "Error: received 1 argument, expected 2
Help: both a destination and source path are required";

pub const FILE_MISSING: &str = "Error: failed to locate file";
pub const NOT_A_FILE: &str = "Error: invalid file provided";

pub const NOT_PLIST: &str = "Error: first argument is not a .plist file";
pub const NOT_XML: &str = "Error: second argument is not a .xml file";
