/// A hint suggesting how to use the command.
pub const USAGE: &str = "Usage:
  $ plinject [destination .plist file] [source .xml file to inject] [optional output .plist file]

Note: the destination .plist file is overwritten if no output file is provided";

/// Two example usages of the command.
pub const EXAMPLES: &str = "Examples:
  $ plinject Example.app/Contents/Info.plist injection.xml
  $ plinject source.plist injection.xml output.plist";

/// Info message if a `DOCTYPE` tag is located.
pub const DOCTYPE_INFO: &str =
    "Info: located `DOCTYPE` tag in source .plist; copying it to the output";

/// Error message to show if no arguments are provided.
pub const ERR_NO_ARGS: &str = "Error: received 0 arguments, expected at least 2
Help: a destination and source path are required";

/// Error message to show if only one argument is provided.
pub const ERR_ONE_ARG: &str = "Error: received 1 argument, expected at least 2
Help: both a destination and source path are required";

/// Error message to show if a file was not able to be located.
pub const ERR_FILE_NOT_FOUND: &str = "Error: failed to locate file";

/// Error message to show if a file is invalid (e.g. a directory).
pub const ERR_UNKNOWN_FILE: &str = "Error: invalid file provided";

/// Error mesage to show if the expected `.plist` file has an incorrect
/// extension.
pub const ERR_NOT_PLIST_FILE: &str =
    "Error: first argument is not a .plist file";

/// Error mesage to show if the expected `.xml` file has an incorrect extension.
pub const ERR_NOT_XML_FILE: &str = "Error: second argument is not a .xml file";
