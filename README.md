# `plinject`

`plinject` is a tool designed to inject properties into an `Info.plist` file: a
file found in many macOS applications which stores key properties about the
application such as its name, version, or icon.

An example usage for `plinject` would be to [inject properties to assocate a
certain file type with an
application](#example%3A-associate-file-type-with-application).

Typically setting .plist files is achieved with Xcode, but this tool provides
an alternative option if you know which properties need to be set. This can be
helpful for project that use build tools such as CMake or Cargo.

`plinject` only depends on the [`xml-rs`](https://crates.io/crates/xml-rs)
crate.

## Usage

Run `plinject` without arguments to see this message:

```
Error: received 0 arguments, expected at least 2
Help: a destination and source path are required

Usage:
  $ plinject [destination .plist file] [source .xml file to inject] [optional output .plist file]

Note: the destination .plist file is overwritten if no output file is provided

Examples:
  $ plinject Example.app/Contents/Info.plist injection.xml
  $ plinject source.plist injection.xml output.plist
```

## Building from source

### Prerequisites

- `git` must be installed (`brew install git` via Homebrew)
- [Rust](https://www.rust-lang.org/tools/install) must be installed
- (macOS) Apple Command Line Tools (`xcode-select --install`)

### 1. Clone the repo

```bash
git clone https://github.com/jamiegibney/plinject && cd plinject
```

### 2. Build with `cargo`

```bash
cargo build --release
```

### 3. Run the tool

See [this section](#usage) for details on using `plinject`.

#### Run directly with `cargo`

```bash
cargo run --release -- [arguments]
```

#### Run after building

```bash
./target/release/plinject [arguments]
```

#### Note

`plinject` is statically linked, thus you can move the built `plinject`
executable wherever is needed and it should run. However, it is nonetheless
recommended to create a symlink, e.g.:

```bash
ln -sF ./target/release/plinject /path/to/project/plinject
```

## Example: associate file type with application

Assuming a target application named `Example.app`.

#### Properties to inject: `file_type.xml`

```xml
<key>CFBundleDocumentTypes</key>
<array>
  <dict>
    <key>CFBundleTypeIconSystemGenerated</key>
    <integer>1</integer>
    <key>CFBundleTypeExtensions</key>
    <array>
      <string>extension_name</string>
    </array>
    <key>NSDocumentClass</key>
    <string>NSDocument</string>
    <key>CFBundleTypeName</key>
    <string>File type name</string>
    <key>LSHandlerRank</key>
    <string>Owner</string>
  </dict>
</array>
````

#### Command

```bash
cargo run --release -- Example.app/Contents/Info.plist file_type.xml
```

or

```bash
./target/release/plinject Example.app/Contents/Info.plist file_type.xml
```

## Other tools

- [`xmlformat`](https://github.com/pamoller/xmlformatter) is a great tool to use
if you want to clean up .plist or .xml files.
