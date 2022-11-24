
use std::io::Write;

use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;
use quick_xml::events::Event;

pub mod basic_elem;

pub enum DruidXMLError {
	ChildlessElement,
	XMLError( usize )
}

pub trait SourceWriter {
	
}


pub fn parse_xml(ext_writer:Option<ElementSourceProvider>, xml_src:&str) -> Result<Box<dyn ElementSource>, DruidXMLError> {
	let reader = Reader::from_str(xml_src);
	let mut buf = Vec::new();
	// The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
	loop {
		// NOTE: this is the generic case when we don't know about the input BufRead.
		// when the input is a &str or a &[u8], we don't actually need to use another
		// buffer, we could directly call `reader.read_event()`
		match reader.read_event_into(&mut buf) {
			Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
			// exits the loop when reaching end of file
			Ok(Event::Eof) => break,

			Ok(Event::Start(e)) => {


				match e.name().as_ref() {
					b"tag1" => println!("attributes values: {:?}",
										e.attributes().map(|a| a.unwrap().value)
										.collect::<Vec<_>>()),
					b"tag2" => count += 1,
					_ => (),
				}
			}
			Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),

			// There are several other `Event`s we do not consider here
			_ => (),
		}
		// if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
		buf.clear();
	}
}