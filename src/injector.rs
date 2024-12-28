use crate::{errors::DOCTYPE_INFO, helpers::get_doctype_str};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};
use xml::{reader::XmlEvent, EmitterConfig, EventReader, EventWriter};

/// The XML key to check for before injecting properties.
// TODO: this should be configurable.
const XML_DICT_KEY: &str = "dict";

/// Type for injecting XML properties into an XML source, writing to an internal
/// buffer.
pub struct XMLInjector<W: Write> {
    writer: EventWriter<BufWriter<W>>,
}

impl<W: Write> XMLInjector<W> {
    /// Creates a new `XMLInjector` which will write to the
    /// `output_destination`.
    pub fn new(write_destination: W) -> Self {
        Self {
            writer: EmitterConfig::new()
                .perform_indent(true)
                .indent_string("  ")
                .line_separator("\n")
                .create_writer(BufWriter::new(write_destination)),
        }
    }

    /// Copies the XML contents of `source` into an internal buffer, and injects
    /// the XML contents of `injection` where appropriate.
    pub fn inject(
        &mut self,
        source: &File,
        injection: &File,
    ) -> Result<(), String> {
        // we need to get the doctype string first as it involves reading
        // through the whole source file, which is done incrementally in the
        // loop below.
        let doctype_str = get_doctype_str(source);
        let source_reader = EventReader::new(BufReader::new(source));

        for line in source_reader.into_iter().flatten() {
            // if we've found the end element for the target key we should
            // inject here.
            if let XmlEvent::EndElement { name } = &line
                && name.local_name == XML_DICT_KEY
            {
                self.inject_here(injection)?;
            }

            // if we fail to write this event, we should return an error.
            if let Some(writer_event) = line.as_writer_event()
                && let Err(e) = self.writer.write(writer_event)
            {
                return Err(format!(
                    "Error: failed to copy XML from source file\nDetails: \"{e}\""
                ));
            }

            // if we've just copied the start of the XML document and a DOCTYPE
            // tag was found, we should write it directly to the output buffer.
            if let XmlEvent::StartDocument { .. } = line
                && let Some(tag) = &doctype_str
            {
                match write!(self.writer.inner_mut(), "{tag}") {
                    Ok(()) => {
                        println!("{DOCTYPE_INFO}");
                    }
                    Err(e) => {
                        return Err(format!(
                            "Error: failed to inject DOCTYPE property\nDetails: \"{e:?}\""
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    /// Returns a reference to the internal buffer.
    pub fn buffer(&self) -> &[u8] {
        self.writer.inner_ref().buffer()
    }

    /// Method which copies from the injection source to the internal buffer.
    fn inject_here(&mut self, injection: &File) -> Result<(), String> {
        let injection = EventReader::new(BufReader::new(injection));

        for line in injection.into_iter().flatten() {
            if let XmlEvent::StartDocument { .. } = line {
                continue;
            }

            if let Some(writer_event) = line.as_writer_event()
                && let Err(e) = self.writer.write(writer_event)
            {
                return Err(format!(
                    "Error: failed to copy from injection file\nDetails: \"{e}\""
                ));
            }
        }

        Ok(())
    }
}
