
use std::io::Write;

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

struct ElementSep<'a> {
	tag : &'a str,
	class : Vec<&'a str>,
	id : Option<&'a str>
}

struct Element<'a> {
	parents : Vec<&'a ElementSep<'a>>,
	style : Vec<&'a simplecss::Rule<'a>>,
	elem_style_rule : &'a simplecss::Rule<'a>
}


pub trait SourceWriter {
	
}

fn write_rust_source<'a, W:std::io::Write>(w:W, parent:Option<&ElementSep<'a>>, parent_attrs:Option<Attributes>, 
et_attrs:Attributes, style:StyleSheet<'a>, text:&str) -> Result<usize, Error> {
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
	let mut reader = Reader::from_str(xml);

	let mut res = String::new();

	let mut styles:Vec<StyleSheet> = vec![];
	
	loop {
		match reader.read_event() {
			Ok(Event::Start(e)) => {
				let ename = e.name();
				let tag = ename.as_ref();
				match tag {
					b"widget" | b"flex" | b"style" => (),
					_ => {
						return Err(Error::InvalidTopElement(reader.buffer_position()))
					}
				}

				let start_pos = reader.buffer_position();
				match reader.read_to_end(e.name()) {
					Ok(span) => {
						match tag {
							b"style" => {
								styles.push( StyleSheet::parse(&xml[span]) );
							}
							_ => {

							}
						}
					},
					_ => return Err(Error::InvalidCloseTag(start_pos))
				}
			},

			Err(e) => return Err(Error::XMLSyntaxError( (reader.buffer_position(),e) )),
			// exits the loop when reaching end of file
			Ok(Event::Eof) => return Ok(res),
			//Ok(Event::Start(e)) => (),
			// Ok(Event::Comment(_)) => (),
			// Ok(Event::CData(_)) => (),
			// Ok(Event::Empty(_)) => (),
			// Ok(Event::Decl(_)) => (),
			// Ok(Event::PI(_)) => (),
			// Ok(Event::DocType(_)) => (),
			Ok(Event::Text(e)) => (),
			// Ok(Event::End(e)) => (),

			el @ _ => {
				println!("{:?}", el);
				return Err(Error::InvalidTopElement(reader.buffer_position()))
			}
		}
}
	
}

fn parse_child_content<R, W:Write>(reader:&mut Reader<R>, wirter:W) -> Result<bool, Error> {
	todo!()
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