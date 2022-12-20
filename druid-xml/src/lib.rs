
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
	MustBeTopElement(usize),
	FnMapRequired(usize),
	ChildlessElement( usize ),
	UnknownAttribute( usize ),
	InvalidTopElement( usize ),
	CSSSyntaxError( (usize,simplecss::Error) ),
	XMLSyntaxError( (usize,quick_xml::Error) )
}

impl Error {
	fn error_at(&self) -> usize {
		match self {
			Error::InvalidChild(s) => *s,
			Error::InvalidCloseTag(s) => *s,
			Error::MustBeTopElement(s) => *s,
			Error::FnMapRequired(s) => *s,
			Error::ChildlessElement(s) => *s,
			Error::UnknownAttribute(s) => *s,
			Error::InvalidTopElement(s) => *s,
			Error::CSSSyntaxError( (s,_) ) => *s,
			Error::XMLSyntaxError( (s, _) ) => *s,
		}
	}
}

#[derive(Debug)]
struct Element<'a> {
	tag : &'a [u8],
	attrs : Attributes<'a>,
	style : StyleSheet<'a>,
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
						parse_child_content(0, &mut reader, &mut elem, &mut res)?;
						res.push_str("}}")
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

fn parse_child_content<'a:'b, 'b>(depth:usize, reader:&mut Reader<&'a [u8]>, elem:&'b mut Element, writer:&mut String) -> Result<(), Error> {
	let mut text:Option<quick_xml::events::BytesText<'a>> = None;
	let mut child_count = 0;

	macro_rules! writeln {
		( $($tts:tt)* ) => { {
			write!(writer, "{}", std::iter::repeat('\t').take(depth).collect::<String>() ).unwrap();
			write!(writer, $($tts)* ).unwrap();
			write!(writer, "\n").unwrap();
		} }
	}

	match elem.tag {
		b"flex" => {
			if elem.attrs.find( |e| if let Ok(e) = e { e.key.as_ref() == b"column" } else { false }).is_some() {
				writeln!("let flex = Flex::column()");
			} else {
				writeln!("let flex = Flex::row()");
			}
		}
		b"custom" => {
			if depth != 0 {
				return Err(Error::MustBeTopElement(reader.buffer_position()))
			}
		}
		_ => ()
	}

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
				child_count += 1;
				writeln!("let child_{}_{} = {{", depth, child_count);
				parse_child_content(depth+1, reader, &mut child_elem, writer)?;
				writeln!("}}");

				//TODO elem attribute check
				writeln!("flex.with_child(child_{}_{});", depth, child_count);
			}
			Ok(Event::End(e)) => {
				if e.name().as_ref() == elem.tag {
					match elem.tag {
						b"flex" => {
							writeln!("flex");
						}
						b"label" => {
							let name = text.as_ref().map( |e| String::from_utf8_lossy(&e) ).unwrap_or( std::borrow::Cow::Borrowed("Label") );
							writeln!("let label = Label::new(\"{}\");", name );
							writeln!("label");
						}
						b"button" => {
							let name = text.as_ref().map( |e| String::from_utf8_lossy(&e) ).unwrap_or( std::borrow::Cow::Borrowed("Button") );
							writeln!("let label_for_button = Label::new(\"{}\");", name );
							writeln!("let button = Button::from_label(btn_label);");
							writeln!("button");
						},
						_ => () //ignore all text like CRLF
					};
					return Ok(())
				}
			}
			Ok(Event::Text(t)) => {
				// let text:&[u8] = text.as_ref();
				// elem.text = Some(text);
				text = Some(t);
			}
			Ok(Event::Comment(_)) => (), //ignore
			Ok(Event::Empty(_)) => (), //ignore
			Ok(Event::Eof) => {
				return Err(Error::InvalidCloseTag(pos))
			},
			Err(e) => return Err(Error::XMLSyntaxError( (pos,e) )),
			etc@ _ => {
				todo!( "{:?}",etc )
			}
		}
	}
	
}


#[cfg(test)]
mod test {
	#[test]
	fn test() {
		let src = r#"
		<style>
		label:hover { color:#333333 }
		button {color:black, background-color:white}
		textbox {color:black, background-color:gray}
		#pwd {color:white, background-color:black}
		</style>

		<custom fn=build_icon>
			<flex direction="row">
				<label style="font-size:25px">${icon_text}</label>
				<label style="font-size:10px">${title}</label>
			</flex>
		</custom>

		<flex fn=build_main>
			<label>Login..</label>

			<widget name=native_custom_widget title="GO"/>
			<widget name=native_custom_widget title="MAIN"/>
			<widget name=native_custom_widget title="NO"/>
			<icon title="Exit" icon="★" onclick="exit"/>

			<!-- you can remove direction="col" attribute because that default value is "col" and also other all default value is ignorable -->
			<flex column cross_alignment="" main_alignment="" fill_major_axis="true">
				<label>ID</label><textbox class="normal" lens="id" value="Default Value" placeholder="Input here"/>
				<label>PWD</label><textbox lens="pwd" placeholder="Your password"/>
			</flex>

			<flex>
				<button onclick="login">OK</button>
				<button style="background-color:red; color:white">CANCEL</button>
			</flex>
		</flex>
		"#;
		let result = super::parse_xml( src );
		println!("Result : {:?}", result);
		match result {
			Ok(compiled) => println!("{}", compiled),
			Err(e) => { println!("Error : {}", &src[e.error_at() .. ])}
		}
	}
}