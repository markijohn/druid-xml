
use std::io::Write;
use std::fmt::Write as FmtWrite;
use std::borrow::Cow;

use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;
use quick_xml::events::Event;
use simplecss::StyleSheet;

#[derive(Debug)]
pub enum Error {
	InvalidChild( usize ),
	InvalidCloseTag( usize ),
	MustBeTopElement(usize),
	AttributeRequired( (usize, &'static str)),
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
			Error::AttributeRequired( (s, _)) => *s,
			Error::ChildlessElement(s) => *s,
			Error::UnknownAttribute(s) => *s,
			Error::InvalidTopElement(s) => *s,
			Error::CSSSyntaxError( (s,_) ) => *s,
			Error::XMLSyntaxError( (s, _) ) => *s,
		}
	}
}

#[derive(Debug,Clone)]
struct Element<'a> {
	parent : std::rc::Rc<Element<'a>>,
	tag : &'a [u8],
	attrs : Attributes<'a>,
	style : Option<StyleSheet<'a>>,
}

impl <'a> simplecss::Element for Element<'a> {
    fn parent_element(&self) -> Option<Self> {
        todo!()
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        todo!()
    }

    fn has_local_name(&self, name: &str) -> bool {
        todo!()
    }

    fn attribute_matches(&self, local_name: &str, operator: simplecss::AttributeOperator) -> bool {
        todo!()
    }

    fn pseudo_class_matches(&self, class: simplecss::PseudoClass) -> bool {
        todo!()
    }
}

trait AttributeGetter {
	fn get(&self, name:&[u8]) -> Option<Cow<[u8]>>;
	
	fn get_result(&self, name:&'static str, pos:usize) -> Result<Cow<[u8]>,Error> {
		let nameb = name.as_bytes();
		self.get(nameb)
		.ok_or( Error::AttributeRequired((pos, name)) )
	}
}

impl <'a> AttributeGetter for Attributes<'a> {
    fn get(&self, name:&[u8]) -> Option<Cow<[u8]>> {
        self.clone()
		.find( |e| 
			e.is_ok() && e.as_ref().unwrap().key.as_ref() == name
		).map( |e|  {
			e.unwrap().value
		})
    }
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
							style: None,
						};
						parse_content_recurrsive(0, &mut reader, &mut elem, &style, &mut res)?;
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

fn custom_ui<'a, IA, IS>(text:&'a str, attrs:IA, styles:IA)
where IA:Iterator<Item=(&'a [u8],Cow<'a,[u8]>)>, IS:Iterator<Item=(&'a [u8], IA)> {

}

struct AttributeIter<'a> {
	attrs : Attributes<'a>
}

struct StyleIter<'a> {
	style : StyleSheet<'a>
}

impl <'a> Iterator for AttributeIter<'a> {
	type Item = (&'a [u8], Cow<'a,[u8]>);

	fn next(&mut self) -> Option<Self::Item> {
		while let Some(e) = self.attrs.next() {
			if let Ok(attr) = e {
				return Some( (
					attr.key.into_inner()
					, attr.value
				) )
			}
		}
		None
    }
}

fn parse_content_recurrsive<'a:'b, 'b>(depth:usize, reader:&mut Reader<&'a [u8]>, elem:&'b mut Element, style:&StyleSheet, writer:&mut String) -> Result<(), Error> {
	//custom_ui();
	let mut text:Option<quick_xml::events::BytesText<'a>> = None;
	let mut child_count = 0;

	macro_rules! writeln {
		( $($tts:tt)* ) => { {
			write!(writer, "{}", std::iter::repeat('\t').take(depth+1).collect::<String>() ).unwrap();
			write!(writer, $($tts)* ).unwrap();
			write!(writer, "\n").unwrap();
		} }
	}

	match elem.tag {
		b"flex" => {
			if depth == 0 {
				let fnname = elem.attrs.get_result("fn", reader.buffer_position())?;
				write!(writer,"fn {}() {{\n", String::from_utf8_lossy(&fnname)).unwrap();
			}

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
			let fnname = elem.attrs.get_result("fn", reader.buffer_position())?;
			write!(writer,"fn {}() {{\n", String::from_utf8_lossy(&fnname)).unwrap();
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
				parse_content_recurrsive(depth+1, reader, &mut child_elem, &style, writer)?;
				writeln!("}}");

				if elem.tag == b"flex" {
					//TODO elem attribute check
					writeln!("flex.with_child(child_{}_{});", depth, child_count);
				}
			}
			Ok(Event::End(e)) => {
				if e.name().as_ref() == elem.tag {
					match elem.tag {
						b"flex" => {
							writeln!("flex");
							if depth == 0 {
								write!(writer,"}}\n").unwrap();
							}
						}
						b"custom" => {
							if depth == 0 {
								writeln!("child_{}_{}", depth, child_count);
								write!(writer,"}}\n").unwrap();
							} else {
								unreachable!()
							}
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
				}

				//TODO : make EnvSetup
				//TODO : bind events

				break
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
	
	Ok( () )
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

		<custom fn="build_icon">
			<flex direction="row">
				<label style="font-size:25px">${icon_text}</label>
				<label style="font-size:10px">${title}</label>
			</flex>
		</custom>

		<flex fn="build_main">
			<label>Login..</label>

			<widget name=native_custom_widget title="GO"/>
			<widget name=native_custom_widget title="MAIN"/>
			<widget name=native_custom_widget title="NO"/>
			<icon title="Exit" icon="★" onclick="exit"/>

			<!-- you can remove direction="col" attribute because that default value is "col" and also other all default value is ignorable -->
			<flex cross_alignment="" main_alignment="" fill_major_axis="true">
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

	#[test]
	fn stylesheet() {
		//css order
		//1. !important
		//2. explicit define in html style tag attribute
		//3. #id
		//4. .class , abstract class(link,visited,hover,active,focus,first,last,first-child,last-child,nth-child())
		//5. tag name
		//6. inherite attribute
		let mut css = simplecss::StyleSheet::new();
		let css1 = "
		body {background-color:blue; margin:2px}
		flex {padding:2px;}
		";

		let css2 = "
		body {background-color:yellow}
		flex .inside .myflex {padding:5px}
		";
		css.parse_more(css1);
		println!("one : {:#?}", css);

		css.parse_more(css2);
		println!("\n\ntwo : {:#?}", css);
	}
}