use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};
use xml::{reader::XmlEvent, EmitterConfig, EventReader, EventWriter};

const DOCTYPE: &str = r#"<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">"#;
const XML_DICT_KEY: &str = "dict";

pub struct XMLInjector<W: Write> {
    writer: EventWriter<BufWriter<W>>,
}

impl<W: Write> XMLInjector<W> {
    pub fn new(destination: W) -> Self {
        Self {
            writer: EmitterConfig::new()
                .perform_indent(true)
                .indent_string("  ")
                .line_separator("\n")
                .create_writer(BufWriter::new(destination)),
        }
    }

    pub fn inject(
        &mut self,
        source: &File,
        injection: &File,
    ) -> Result<(), String> {
        let source = EventReader::new(BufReader::new(source));

        for line in source.into_iter().flatten() {
            if let XmlEvent::EndElement { name } = &line
                && name.local_name == XML_DICT_KEY
            {
                self.inject_here(injection)?;
            }

            if let Some(writer_event) = line.as_writer_event() {
                if let Err(e) = self.writer.write(writer_event) {
                    return Err(format!(
                        "Error: failed to copy\nDetails: \"{e}\""
                    ));
                }
            }

            if let XmlEvent::StartDocument { .. } = line
                && let Err(e) = write!(self.writer.inner_mut(), "{DOCTYPE}")
            {
                return Err(format!("Error: failed to inject DOCTYPE property\nDetails: \"{e:?}\""));
            }
        }

        Ok(())
    }

    pub fn buffer(&self) -> &[u8] {
        self.writer.inner_ref().buffer()
    }

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
                    "Error: failed to inject\nDetails: \"{e}\""
                ));
            }
        }

        Ok(())
    }
}
