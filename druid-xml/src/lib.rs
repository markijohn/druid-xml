use std::borrow::Cow;

use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::name::QName;
use simplecss::{StyleSheet};

pub mod writer;
use writer::{SourceGenerator, DruidGenerator};

#[derive(Debug)]
pub enum Error {
	InvalidChild( usize ),
	CloseWithoutStart( usize ),
	InvalidCloseTag( usize ),
	MustBeTopElement(usize),
	AttributeRequired( (usize, &'static str)),
	InvalidAttributeValue( (usize, &'static str) ),
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
			Error::CloseWithoutStart( s ) => *s,
			Error::InvalidCloseTag(s) => *s,
			Error::MustBeTopElement(s) => *s,
			Error::AttributeRequired( (s, _)) => *s,
			Error::InvalidAttributeValue( (s, _) ) => *s,
			Error::ChildlessElement(s) => *s,
			Error::UnknownAttribute(s) => *s,
			Error::InvalidTopElement(s) => *s,
			Error::CSSSyntaxError( (s,_) ) => *s,
			Error::XMLSyntaxError( (s, _) ) => *s,
		}
	}
}


#[derive(Debug,Clone)]
pub(crate) struct Element<'a> {
	src_pos : usize,
	bs : BytesStart<'a>,
	text : Option<quick_xml::events::BytesText<'a>>,
}


impl <'a> Element<'a> {
	pub fn tag(&'a self) -> QName<'a> {
		self.bs.name()
	}

	pub fn attributes(&'a self) -> Attributes<'a> {
		self.bs.attributes()
	}
}



pub(crate) trait AttributeGetter {
	fn get(&self, name:&[u8]) -> Option<Cow<[u8]>>;
	
	fn get_result(&self, name:&'static str, pos:usize) -> Result<Cow<[u8]>,Error> {
		let nameb = name.as_bytes();
		self.get(nameb)
		.ok_or( Error::AttributeRequired((pos, name)) )
	}
}

impl <'a> AttributeGetter for Attributes<'a> {
    fn get(&self, name:&[u8]) -> Option<Cow<'a, [u8]>> {
        self.clone()
		.find( |e| 
			e.is_ok() && e.as_ref().unwrap().key.as_ref() == name
		).map( |e|  {
			e.unwrap().value
		})
    }
}


pub fn compile(xml:&str) -> Result<String,Error> {
	let mut writer = DruidGenerator::new();
	let mut style = StyleSheet::new();
	let mut reader = Reader::from_str(xml);
	loop {
		let pos = reader.buffer_position();
		match reader.read_event() {
			Ok(Event::Start(e)) => {
				match e.name().as_ref() {
					b"style" => {
						let start_pos = reader.buffer_position();
						match reader.read_to_end(e.name()) {
							Ok(span) => {
								let css_impl = &xml[span];
								style.parse_more( css_impl );
							},
							_ => return Err(Error::InvalidCloseTag(start_pos))
						}
					},
					_ => {
						let elem = Element {
							src_pos : pos,
							bs : e,
							text : None
						};
						parse_recurrsive(&mut vec![elem], &style, &mut reader, &mut writer)?;
					}
				}
			},

			Err(e) => return Err(Error::XMLSyntaxError( (reader.buffer_position(),e) )),
			// exits the loop when reaching end of file
			Ok(Event::Eof) => break,
			Ok(Event::Comment(_)) => (),
			Ok(Event::CData(_)) => (),
			Ok(Event::Empty(_)) => (),
			Ok(Event::Decl(_)) => (),
			Ok(Event::PI(_)) => (),
			Ok(Event::DocType(_)) => (),
			Ok(Event::Text(_)) => (), //ignore text from root node
			Ok(Event::End(_)) => return Err(Error::CloseWithoutStart(reader.buffer_position())),
		}
	}
	Ok( writer.into() )
}

#[allow(unused)]
fn custom_ui<'a, IA, IS>(text:&'a str, attrs:IA, styles:IA)
where IA:Iterator<Item=(&'a [u8],Cow<'a,[u8]>)>, IS:Iterator<Item=(&'a [u8], IA)> {

}

struct AttributeIter<'a> {
	attrs : Attributes<'a>
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


fn parse_recurrsive<'a,W:SourceGenerator>(stack:&mut Vec<Element<'a>>, global_style:&StyleSheet<'a>,reader:&mut Reader<&'a [u8]>,w:&mut W) 
-> Result< (), Error> {
	w.begin(stack, global_style)?;
	
	let mut last_text = None;
	
	loop {
		let pos = reader.buffer_position();
		match reader.read_event() {
			Ok(Event::Start(e)) => {
				stack.push( Element {
					src_pos : pos,
					bs : e,
					text : None
				} );
				parse_recurrsive(stack, global_style, reader, w)?;
			}
			Ok(Event::End(e)) => {
				if e.name() == stack.last().unwrap().tag() {
					stack.last_mut().unwrap().text = last_text.take();
					w.end( stack, global_style)?;
					stack.pop();
				} else {
					return Err(Error::InvalidCloseTag(pos))
				}
				break
			}
			Ok(Event::Text(t)) => {
				last_text = Some(t);				
			}
			Ok(Event::Comment(_)) => (), //ignore
			Ok(Event::Empty(_)) => (), //ignore
			Ok(Event::Eof) => {
				return Err(Error::InvalidCloseTag(pos))
			},
			Err(e) => return Err(Error::XMLSyntaxError( (pos,e) )),
			_etc@ _ => {
				unimplemented!()
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
		label { color:#333333 }
		button {color:black; background-color:white}
		textbox {color:black; background-color:gray}
		#pwd {color:white, background-color:black}
		</style>

		<!-- you can remove direction="column" attribute because that default value is "row" -->
		<flex fn="build_icon" direction="row">
			<label style="font-size:25px">${icon_text}</label>
			<label style="font-size:10px">${title}</label>
		</flex>

		<flex fn="build_main">
			<label style="color:black; font-size:12em">Login..</label>

			<widget name=native_custom_widget title="GO"/>
			<widget name=native_custom_widget title="MAIN"/>
			<widget name=native_custom_widget title="NO"/>
			<icon title="Exit" icon="★" onclick="exit"/>
	
			<flex direction="column" cross_alignment="" main_alignment="" fill_major_axis="true">
				<label>ID</label><textbox class="normal" lens="id" value="Default Value" placeholder="Input here"/>
				<label>PWD</label><textbox lens="pwd" placeholder="Your password"/>
			</flex>

			<flex>
				<button onclick="login">OK</button>
				<button style="background-color:red; color:white">CANCEL</button>
			</flex>
		</flex>
		"#;
		let result = super::compile( src );
		match result {
			Ok(compiled) => println!("{}", compiled),
			Err(e) => { println!("Error : {:?} : {}", e, &src[e.error_at() .. ])}
		}
	}

	//#[test]
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