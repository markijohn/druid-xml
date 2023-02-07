use std::borrow::Cow;
use std::collections::HashMap;
use std::marker::PhantomData;

use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;
use quick_xml::events::{Event, BytesStart, BytesText};
use quick_xml::name::QName;
use simplecss::{StyleSheet};

pub mod writer;
mod named_color;
use writer::{SourceGenerator, DruidGenerator};

pub mod qwidget;
pub mod widget;
pub mod simple_style;
mod curve;


#[derive(Default)]
struct DummyLens<T,A> {
    o : PhantomData<T>,
    a : A
}

impl <T,A> DummyLens<T,A> {
    fn new(a : A) -> Self {
        Self { o : PhantomData, a : a }
    }
}

impl <T:druid::Data,U> druid::Lens<T, U> for DummyLens<T,U> {
    fn with<V, F: FnOnce(&U) -> V>(&self, data: &T, f: F) -> V {
        f(&self.a)
    }

    fn with_mut<V, F: FnOnce(&mut U) -> V>(&self, data: &mut T, f: F) -> V {
        #[allow(mutable_transmutes)]
        f( unsafe { std::mem::transmute::<&U,&mut U>(&self.a) } )
    }
}

//#[cfg(feature="dynamic")]
pub mod dynamic;

#[cfg_attr(taret_arch="wasm32", wasm_bindgen)]
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

	//invalid attribute value
	InvalidAttributeValue( (usize, &'static str) ),

	///Invalid size(width,height) value
	InvalidSizeAttributeValue( usize ),

	///Invalid border value
	InvalidBorderAttributeValue( usize ),

	///That element can't have child
	ChildlessElement( usize ),

	///Unknown attribute
	UnknownAttribute( usize ),

	//Unknwon localname
	UnknownTag( (usize,String) ),

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
			Error::InvalidAttributeValue( (s,_) ) => *s,
			Error::InvalidSizeAttributeValue( s ) => *s,
			Error::InvalidBorderAttributeValue( s ) => *s,
			Error::ChildlessElement(s) => *s,
			Error::UnknownAttribute(s) => *s,
			Error::UnknownTag( (s,_) ) => *s,
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

	pub fn attributes(&'a self, rel_param:Option<&'a AttributesWrapper>) -> AttributesWrapper<'a> {
		AttributesWrapper { pos:self.src_pos, attrs:self.bs.attributes(), rel_attrs:rel_param }
	}
}

#[derive(Clone)]
pub struct AttributesWrapper<'a> {
	pos : usize,
	attrs : Attributes<'a>,
	rel_attrs : Option<&'a AttributesWrapper<'a>>
}

impl <'a> AttributesWrapper<'a> {
	fn tuples(&self) -> String {
		use std::fmt::Write;
		let mut r = "&[".to_owned();
		self.attrs.clone().map( |e| {
			if let Ok(e) = e {
				write!(&mut r,"({},{}),", String::from_utf8_lossy(e.key.as_ref()), String::from_utf8_lossy(e.value.as_ref())).unwrap();
			}
		});
		r.push(']');
		r
	}
}

pub(crate) trait AttributeGetter {
	fn get(&self, name:&[u8]) -> Option<Cow<[u8]>>;

	fn pos(&self) -> usize;
	
	fn get_result(&self, name:&'static str) -> Result<Cow<[u8]>,Error> {
		let nameb = name.as_bytes();
		self.get(nameb)
		.ok_or( Error::AttributeRequired((self.pos(), name)) )
	}

	fn get_as<T: std::str::FromStr>(&self, name:&[u8]) -> Option<T> {
		if let Some(e) = self.get(name) {
			if let Ok(v) = String::from_utf8_lossy(&e).parse::<T>() {
				return Some(v)
			}
		}
		None
	}

	fn get_as_result<T: std::str::FromStr>(&self, name:&'static str) -> Result<T, Error> {
		let e = self.get_result(name)?;
		match String::from_utf8_lossy(&e).parse::<T>() {
			Ok(e) => Ok(e),
			Err(e) => Err(Error::InvalidAttributeValue( (self.pos(),name) ))
		}
	}

	fn get_size(&self, name:&'static str) -> Result<f64, Error> {
		let e = self.get_result(name)?;
		let se = String::from_utf8_lossy(&e);
		if se.as_ref().ends_with("px") {
			se[..se.len()-2].parse::<f64>().map_err( |_| Error::InvalidAttributeValue( (self.pos(),name) ) )
		} else {
			se.parse::<f64>().map_err( |_| Error::InvalidAttributeValue( (self.pos(),name) ) )
		}
	}
}

impl <'a> AttributeGetter for AttributesWrapper<'a> {
    fn get(&self, name:&[u8]) -> Option<Cow<'a, [u8]>> {
        self.attrs.clone()
		.find( |e| 
			e.is_ok() && e.as_ref().unwrap().key.as_ref() == name
		).map( |e|  {
			let value = e.unwrap().value;
			let ck_value = String::from_utf8_lossy(&value);
			if ck_value.starts_with("${") && ck_value.ends_with("}") {
				if let Some(rel) = self.rel_attrs {
					let key = &ck_value[2..ck_value.len()-1];
					if let Some(alter_value) = rel.get(key.as_bytes() ) {
						return alter_value
					}
				}
				// let key = &ck_value[2..ck_value.len()-1];
				// if let Some(alter) = self.rel_attrs.clone().unwrap().find( |e|
				// 	e.is_ok() && e.as_ref().unwrap().key.as_ref() == key.as_bytes()
				// ) {
				// 	return alter.unwrap().value
				// }
			}
			value
		})
    }

	fn pos(&self) -> usize {
		self.pos
	}


}


pub fn compile(xml:&str, wrappers:&HashMap<String,String>) -> Result<String,Error> {
	let mut writer = DruidGenerator::new();
	let mut style = StyleSheet::new();
	let mut reader = Reader::from_str(xml);
	let mut elem_map = HashMap::new();
	let mut expected_main_widget = None;
	let mut last_widget = None;
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
							let fnname = elem.attributes( None ).get_as_result::<String>("fn")?;
							last_widget = Some(fnname.clone());
                            if fnname.find("main").is_some() {
                                expected_main_widget = Some(fnname);
                            }
							elem_map.insert( elem.attributes( None ).get_as_result::<String>("fn")?, elem);
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


	if let Some(main) = expected_main_widget.and( last_widget ) {
        if let Some(elem ) = elem_map.get(&main) {
			let attrs = elem.attributes(None);
			let fn_name = attrs.get_result("fn")?;
			let fn_name = String::from_utf8_lossy( fn_name.as_ref() );
			let lens = attrs.get_result("lens")?;
			let lens = String::from_utf8_lossy( lens.as_ref() );
			writer.write_raw(&format!("fn {fn_name}() -> impl druid::Widget<{lens}> {{\n") ).unwrap();
			writer.write(&elem_map, &elem, &style, wrappers).unwrap();
			writer.write_raw("}\n").unwrap();
        } else {
            panic!();
        }
    } else {
        panic!();
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

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch="wasm32")]
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
extern {
	pub fn get_xml_src() -> String;

	pub fn xml_error(cause:&str, codeat:usize, ext:&str);

	#[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(target_arch="wasm32")]
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub fn show_preview() {
	use druid::{WindowDesc, LocalizedString, TimerToken, AppLauncher, Event, Widget, Data, Lens, EventCtx, LayoutCtx, UpdateCtx, LifeCycle, LifeCycleCtx, PaintCtx, Env, Size, BoxConstraints};

	std::panic::set_hook(Box::new(console_error_panic_hook::hook));

	struct DynWidget {
		timer_id: TimerToken,
		child : Option< Box<dyn Widget<()>> >
	}

	impl Widget<()> for DynWidget {
		fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut (), env: &Env) {
			match event {
				Event::WindowConnected => {
					self.timer_id = ctx.request_timer( std::time::Duration::from_millis(200) );
				},
				Event::Timer(id) => {
					if *id == self.timer_id {
						let src = get_xml_src();
						if !src.is_empty() {
							match dynamic::generate_widget( &src ) {
								Ok(widget) => {
									self.child = Some( widget );
									xml_error("",0,"");
									ctx.children_changed();
								},
								Err(e) => {
									match e {
										Error::InvalidFlexChildNum( s ) => xml_error("InvalidFlexChildNum", s, ""),
										Error::InvalidSplitChildNum( s ) => xml_error("InvalidSplitChildNum", s, ""),
										Error::InvalidContainerChildNum( s ) => xml_error("InvalidContainerChildNum", s, ""),
										Error::InvalidScrollChildNum( s ) => xml_error("InvalidScrollChildNum", s, ""),
										Error::CloseWithoutStart( s ) => xml_error("CloseWithoutStart", s, ""),
										Error::InvalidCloseTag(s) => xml_error("InvalidCloseTag", s, ""),
										Error::AttributeRequired( (s, n)) => xml_error("AttributeRequired", s, n),
										Error::InvalidAttributeValue( (s, n) ) => xml_error("InvalidAttributeValue", s, n),
										Error::InvalidSizeAttributeValue( s ) => xml_error("InvalidSizeAttributeValue", s, ""),
										Error::InvalidBorderAttributeValue( s ) => xml_error("InvalidBorderAttributeValue", s, ""),
										Error::ChildlessElement(s) => xml_error("ChildlessElement", s, ""),
										Error::UnknownAttribute(s) => xml_error("UnknownAttribute", s, ""),
										Error::UnknownTag( (s,e) ) => xml_error("UnknownTag", s, &e),
										Error::InvalidTopElement(s) => xml_error("InvalidTopElement", s, ""),
										Error::CSSSyntaxError( (s, e) ) => xml_error("CSSSyntaxError", s, &format!("{:?}",e) ),
										Error::XMLSyntaxError( (s, e) ) => xml_error("XMLSyntaxError", s, &format!("{:?}",e) ),
									}
								}
							}
						}
						self.timer_id = ctx.request_timer( std::time::Duration::from_millis(200) );
					}
				}
				_ => (),
			}

			if let Some(child) = self.child.as_mut() {
				child.event(ctx, event, data, env);
			}
		}
	
		fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &(), env: &Env) {
			if let Some(child) = self.child.as_mut() {
				child.lifecycle(ctx, event, data, env)
			}
		}
	
		fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &(), data: &(), env: &Env) {
			if let Some(child) = self.child.as_mut() {
				child.update(ctx, _old_data, data, env);
			}
		}
	
		fn layout(
			&mut self,
			ctx: &mut LayoutCtx,
			bc: &BoxConstraints,
			data: &(),
			env: &Env,
		) -> Size {
			if let Some(child) = self.child.as_mut() {
				child.layout(ctx, bc, data, env)
			} else {
				bc.constrain((100.0, 100.0))
			}
		}
	
		fn paint(&mut self, ctx: &mut PaintCtx, data: &(), env: &Env) {
			if let Some(child) = self.child.as_mut() {
				child.paint(ctx, data, env);
			}
		}
	}

	fn build_main() -> Box<dyn Widget<()>> {
		Box::new( DynWidget {timer_id:TimerToken::INVALID, child:None} )
	}

	let window = WindowDesc::new( build_main() )
	.window_size((223., 300.))
	.resizable(false)
	.title(
		LocalizedString::new("basic-demo").with_placeholder("Basic Demo"),
	);
	AppLauncher::with_window(window)
	.launch( () )
	.expect("launch failed");
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;

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
		let result = super::compile( src, &HashMap::new() );
		match result {
			Ok(compiled) => println!("{}", compiled),
			Err(e) => { println!("Error : {:?} : {}", e, &src[e.error_at() .. ])}
		}
	}

	#[test]
	fn test_basic() {
		let src = r#"
		<flex fn="build_main" lens="()">
			<label>Hello Druid!</label>
			<button>OK</button>
		</flex>
        "#;
		let result = super::compile( src, &HashMap::new() );
		match result {
			Ok(compiled) => println!("{}", compiled),
			Err(e) => { println!("Error : {:?} : {}", e, &src[e.error_at() .. ])}
		}
	}

	#[test]
	fn custom_widget() {
		let src = r#"
        <style>
        .wrap_border { padding:10; border:5px solid cyan; width:200px; height:50px; }
        </style>
        
        <flex fn="my_custom">
          <label>Label</label>
          <textbox lens="MyAppState:name"/>
        </flex>
        
        <flex fn="my_custom_param">
          <label>${name}</label>
          <textbox placeholder="${placeholder}" lens="MyAppState:name"/>
        </flex>
        
        <flex direction="column" fn="build_main" lens="MyAppState" must_fill_main_axis="true" axis_alignment="spaceevenly">
          <!-- map custom widget -->
          <my_custom/>
        
          <!-- custom widget with style -->
          <my_custom class="wrap_border"/>
          
          <!-- custom widget with parameter -->
          <my_custom_param name="MyName" placeholder="Input here..."/>
        </flex>
        "#;
		let result = super::compile( src, &HashMap::new() );
		match result {
			Ok(compiled) => println!("{}", compiled),
			Err(e) => { println!("Error : {:?} : {}", e, &src[e.error_at() .. ])}
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

		let mut css = simplecss::StyleSheet::parse(r#"
		label { color:black; font-size:16px }
		.my_label { color:yellow; font-size:1.6em }
		#my_special_label { color:cyan; font-soze:24px }
		"#);
		for rule in css.rules {
			println!("{} : {:?}", rule.selector.to_string(), rule.selector.specificity());
		}
	}

	#[test]
	fn basic_mapping() {
		let src = r#"
			r#"
			<!-- The top-level element must have a `fn` `lens` element. -->
			<!-- `fn` is generated function name. -->
			<!-- `lens` is druid `Lens` type. -->
			<flex direction="column" fn="build_main" lens="()">
			  <flex>
				  <label flex="1">Hello Druid!</label>
				  <button id="my_btn" flex="1">OK</button>
			  </flex>
			  <label>Second</label>
			</flex>
			"#;
		let mut map = HashMap::new();
		map.insert("#my_btn".to_string(), r#"|btn| {
				btn.on_click( |_,_,_| {
					println!("On clicked");
				})
			}"#.to_string() );
		
		let result = super::compile(src, &map);
		println!("{}",result.unwrap());
	}
}