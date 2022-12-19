
use std::io::Write;
use std::fmt::Write as FmtWrite;

use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;
use quick_xml::events::Event;
use simplecss::StyleSheet;

#[derive(Debug)]
pub enum Error {
	InvalidChild( usize ),
	InvalidCloseTag( usize ),
	ChildlessElement( usize ),
	UnknownAttribute( usize ),
	InvalidTopElement( usize ),
	CSSSyntaxError( (usize,simplecss::Error) ),
	XMLSyntaxError( (usize,quick_xml::Error) )
}

impl Error {
	fn error_at(&self) -> Option<usize> {
		match self {
			Error::InvalidChild(s) => Some(*s),
			Error::InvalidCloseTag(s) => Some(*s),
			Error::ChildlessElement(s) => Some(*s),
			Error::UnknownAttribute(s) => Some(*s),
			Error::InvalidTopElement(s) => Some(*s),
			Error::CSSSyntaxError( (s,_) ) => Some(*s),
			Error::XMLSyntaxError( (s, _) ) => Some(*s),
		}
	}
}

#[derive(Debug)]
struct Element<'a> {
	tag : &'a [u8],
	attrs : Attributes<'a>,
	style : StyleSheet<'a>,
}

struct LayerInfo {
	naming : String,
	layer_group : HashMap<&str, &str>,
	child : usize
}

fn write_rust_source<'a, W:std::io::Write>(w:W, parent:Option<&Element<'a>>, target:&Element<'a>) -> Result<usize, Error> {
	// w.write_all(  )
	// if let Some(w) = parent {
	// 	w.write()
	// } else {
	// 	write!(w, "let parent = ")
	// 	w.write_all( format!("") );
	// }
	// Ok( )
	todo!()
}

pub fn parse_xml(xml:&str) -> Result<String,Error> {
	let mut reader = Reader::from_str( xml );

	let mut res = String::new();

	let mut style = StyleSheet::new();
	
	loop {
		match reader.read_event() {
			Ok(Event::Start(e)) => {
				let ename = e.name();
				let tag = ename.as_ref();
				match tag {
					b"style" => {
						let start_pos = reader.buffer_position();
						match reader.read_to_end(e.name()) {
							Ok(span) => {
								style.parse_more( &xml[span] );
							},
							_ => return Err(Error::InvalidCloseTag(start_pos))
						}
					},
					_ => {
						let mut elem = Element { 
							tag,
							attrs: e.attributes(),
							style: StyleSheet::new(),
						};
						parse_child_content(&mut reader, &mut elem, &mut res)?;
					}
				}

			},

			Err(e) => return Err(Error::XMLSyntaxError( (reader.buffer_position(),e) )),
			// exits the loop when reaching end of file
			Ok(Event::Eof) => return Ok(res),
			// Ok(Event::Comment(_)) => (),
			// Ok(Event::CData(_)) => (),
			// Ok(Event::Empty(_)) => (),
			// Ok(Event::Decl(_)) => (),
			// Ok(Event::PI(_)) => (),
			// Ok(Event::DocType(_)) => (),
			Ok(Event::Text(e)) => (), //ignore text from root node
			// Ok(Event::End(e)) => (),

			el @ _ => {
				println!("{:?}", el);
				return Err(Error::InvalidTopElement(reader.buffer_position()))
			}
		}
	}
	
}

fn parse_child_content<'a:'b, 'b>(reader:&mut Reader<&'a [u8]>, elem:&'b mut Element, writer:&mut String) -> Result<(), Error> {
	loop {
		let pos = reader.buffer_position();
		match reader.read_event() {
			Ok(Event::Start(e)) => {
				let name = e.name();
				let mut child_elem = Element { 
					tag : name.as_ref(),
					attrs: e.attributes(),
					style: StyleSheet::new(),
				};
				parse_child_content(reader, &mut child_elem, writer)?;
			}
			Ok(Event::End(e)) => {
				if e.name().as_ref() == elem.tag {
					match elem.tag {
						b"flex"  => {
							
						}
						_ => todo!()
					}
				}
			}
			Ok(Event::Text(text)) => {
				// let text:&[u8] = text.as_ref();
				// elem.text = Some(text);
				
				if let Ok(Event::End(e)) = reader.read_event() {
					match elem.tag {
						b"label" => {
							write!(writer, r#"let btn_label = Label::new("{}");"#, String::from_utf8_lossy(&text) ).unwrap();
						}
						b"button" => {
							write!(writer, r#"let btn_label = Label::new("{}");"#, String::from_utf8_lossy(&text) ).unwrap();
							write!(writer, r#"let child{} = Button::from_label(btn_label);"#).unwrap();
						},
						_ => todo!()
					}
				} else {
					return Err( Error::InvalidCloseTag(pos) )
				}
			}
			_ => todo!(),
			Err(_) => todo!(),
		}
	}
	
}


#[cfg(test)]
mod test {
	#[test]
	fn test() {
		let result = super::parse_xml(r#"
		<style>
		label:hover { color:#333333 }
		button {color:black, background-color:white}
		textbox {color:black, background-color:gray}
		#pwd {color:white, background-color:black}
		</style>

		<widget name=icon>
			<flex direction="row">
				<label style="font-size:25px">${icon_text}</label>
				<label style="font-size:10px">${title}</label>
			</flex>
		</widget>

		<flex direction="row">
			<label>Login..</label>

			<widget name=native_custom_widget title="GO"/>
			<widget name=native_custom_widget title="MAIN"/>
			<widget name=native_custom_widget title="NO"/>
			<icon title="Exit" icon="★" onclick="exit"/>

			<!-- you can remove direction="col" attribute because that default value is "col" and also other all default value is ignorable -->
			<flex direction="col" cross_alignment="" main_alignment="" fill_major_axis="true">
				<label>ID</label><textbox class="normal" lens="id" value="Default Value" placeholder="Input here"/>
				<label>PWD</label><textbox lens="pwd" placeholder="Your password"/>
			</flex>

			<flex>
				<button onclick="login">OK</button>
				<button style="background-color:red; color:white">CANCEL</button>
			</flex>
		</flex>
		"#).unwrap();
	}
}