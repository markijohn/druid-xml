use std::borrow::Cow;

use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;
use quick_xml::events::{Event, BytesStart, BytesText};
use quick_xml::name::QName;
use simplecss::{StyleSheet};

pub mod writer;
mod named_color;
use writer::{SourceGenerator, DruidGenerator};

#[derive(Debug)]
pub enum Error {
	///Flex child length at least 1
	InvalidFlexChildNum( usize ),

	///Split child length must be 2
	InvalidSplitChildNum( usize ),

	///Container child must be 1
	InvalidContainerChildNum( usize ),

	///Container child must be 1
	InvalidScrollChildNum( usize ),

	///detected close tag without start tag
	CloseWithoutStart( usize ),

	///start and close tag not matche
	InvalidCloseTag( usize ),

	///Required attribute not exist
	AttributeRequired( (usize, &'static str)),

	///Invalid attribute value
	InvalidAttributeValue( (usize, &'static str) ),

	///That element can't have child
	ChildlessElement( usize ),

	///Unknown attribute
	UnknownAttribute( usize ),

	///Not available as top elment
	InvalidTopElement( usize ),

	///CSS syntax error
	CSSSyntaxError( (usize,simplecss::Error) ),

	///XML syntax error
	XMLSyntaxError( (usize,quick_xml::Error) )
}

impl Error {
	fn error_at(&self) -> usize {
		match self {
			Error::InvalidFlexChildNum( s ) => *s,
			Error::InvalidSplitChildNum( s ) => *s,
			Error::InvalidContainerChildNum( s ) => *s,
			Error::InvalidScrollChildNum( s ) => *s,
			Error::CloseWithoutStart( s ) => *s,
			Error::InvalidCloseTag(s) => *s,
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
	src_pos_end : usize,
	bs : BytesStart<'a>, //Originally, I tried to save the QName, but since BytesStart has a Cow, it took a lifecycle problem. (Cow type is always do that)
	text : Option<quick_xml::events::BytesText<'a>>,
	childs : Vec<Element<'a>>
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

	fn get_as<T: std::str::FromStr>(&self, name:&'static str, pos:usize) -> Result<T, Error> {
		let e = self.get_result(name, pos)?;
		match String::from_utf8_lossy(&e).parse::<T>() {
			Ok(e) => Ok(e),
			Err(e) => Err(Error::InvalidAttributeValue( (pos,name) ))
		}
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
						if let Some(elem) = parse_element(pos, Some(Event::Start(e)), &mut reader )? {
							let attrs = elem.attributes();
							let fn_name = attrs.get_result("fn", elem.src_pos)?;
							let fn_name = String::from_utf8_lossy( fn_name.as_ref() );
							let lens = attrs.get_result("lens", elem.src_pos)?;
							let lens = String::from_utf8_lossy( lens.as_ref() );
							writer.write_raw(&format!("fn {fn_name}() -> impl Widget<{lens}> {{\n") ).unwrap();
							writer.write(&elem, &style).unwrap();
							writer.write_raw("}\n").unwrap();
						} else {
							break
						}
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
			Ok(Event::End(_)) => (), //return Err(Error::CloseWithoutStart(reader.buffer_position())),
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


fn parse_element<'a>(mut src_pos:usize, mut backward:Option<Event<'a>>, reader:&mut Reader<&'a [u8]>) 
-> Result< Option<Element<'a>> , Error> {
	let mut elem:Option<Element> = None;
	let mut last_text = None;
	
	loop {
		let event = if let Some(e) = backward.take() {
			Ok(e)
		} else {
			src_pos = reader.buffer_position();
			reader.read_event()
		};
		//println!("{:?}", event);
		match event {
			Ok(Event::Start(e)) => {
				//check parentable element
				if let Some( el) = elem.as_mut() {
					let tag = el.tag();
					let tag = tag.as_ref();
					if tag == b"flex" {
						//just ok
					} else if tag == b"split" {
						//must be two
						if el.childs.len() == 2 {
							return Err(Error::InvalidSplitChildNum(el.src_pos))
						}
					} else if tag == b"container" || tag == b"scroll" {
						//must be one
						if el.childs.len() == 1 {
							return Err(Error::InvalidContainerChildNum(el.src_pos))
						}
					} else {
						return Err(Error::ChildlessElement(el.src_pos))
					}

					if let Some(child) = parse_element(src_pos, Some(Event::Start(e)),reader)? {
						el.childs.push( child );
					}
				} else {
					let el = Element {
						src_pos,
						src_pos_end : reader.buffer_position(),
						bs : e,
						text : last_text.take(),
						childs : vec![]
					};
					elem = Some(el);
				}
			}
			Ok(Event::End(e)) => {
				if let Some(mut el) = elem.as_mut() {
					//println!("############ {} {}", String::from_utf8_lossy(e.name().as_ref()), String::from_utf8_lossy(el.tag().as_ref()));
					//check matching tag start and end
					if e.name().as_ref() != el.tag().as_ref() {
						return Err(Error::InvalidCloseTag(src_pos))
					} 					
					el.src_pos_end = reader.buffer_position();
					el.text = last_text.take();			
				} else {
					return Err(Error::InvalidCloseTag(src_pos))
				}
				break
			}
			Ok(Event::Text(t)) => {
				last_text = Some(t);				
			}
			Ok(Event::Comment(_)) => (), //ignore
			Ok(Event::Empty(e)) => {
				//like <tag name=value .. />

				//check parentable element
				if let Some( el) = elem.as_mut() {
					let tag = el.tag();
					let tag = tag.as_ref();
					if tag == b"flex" {
						//just ok
					} else if tag == b"split" {
						//must be two
						if el.childs.len() == 2 {
							return Err(Error::InvalidSplitChildNum(el.src_pos))
						}
					} else if tag == b"container" || tag == b"scroll" {
						//must be one
						if el.childs.len() == 1 {
							return Err(Error::InvalidContainerChildNum(el.src_pos))
						}
					} else {
						return Err(Error::ChildlessElement(el.src_pos))
					}

					if let Some(child) = parse_element(src_pos, Some(Event::Empty(e)),reader)? {
						el.childs.push( child );
					}
				} else {
					let el = Element {
						src_pos,
						src_pos_end : reader.buffer_position(),
						bs : e,
						text : last_text.take(),
						childs : vec![]
					};
					elem = Some(el);

					//different to `Event::Start`
					break
				}
			}, //ignore
			Ok(Event::Eof) => {
				if elem.is_some() {
					return Err(Error::InvalidCloseTag(src_pos))
				} else {
					break
				}
			},
			Err(e) => return Err(Error::XMLSyntaxError( (src_pos,e) )),
			_etc@ _ => {
				unimplemented!()
			}
		}
	}
	
	Ok( elem )
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

	#[test]
	fn test_basic() {
		let src = r#"
        <style>
        flex { background-color:black; }
        </style>

        <flex fn="build_main" lens="()">
            <label>HI</label>
            <button>MyButton</button>
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