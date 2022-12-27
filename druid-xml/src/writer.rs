

use std::borrow::Cow;
use simplecss::{Declaration, DeclarationTokenizer, StyleSheet};
use std::fmt::Write;

use crate::{AttributeGetter, Element, Error};


struct ElementQueryWrap<'a> {
	stack : &'a [Element<'a>],
}

impl <'a> simplecss::Element for ElementQueryWrap<'a> {
    fn parent_element(&self) -> Option<Self> {
		if self.stack.len() > 1 {
			Some( ElementQueryWrap { stack:&self.stack[..self.stack.len()-1] } )
		} else {
			None
		}
    }

	// TODO
	/// NOT SUPPORT AdjacentSibling 
    fn prev_sibling_element(&self) -> Option<Self> {
        None
    }

    fn has_local_name(&self, name: &str) -> bool {
        &self.stack[self.stack.len()-1].tag().0 == &name.as_bytes()
    }

    fn attribute_matches(&self, local_name: &str, operator: simplecss::AttributeOperator) -> bool {
		let elem = &self.stack[self.stack.len()-1];
		if let Some(v) = elem.attributes().get(local_name.as_bytes()) {
			return operator.matches( &String::from_utf8_lossy(&v) )
		}

		false
    }

	// TODO
	// NOT SUPPORT
    fn pseudo_class_matches(&self, _class: simplecss::PseudoClass) -> bool {
        //TODO : 
		//https://docs.rs/simplecss/latest/simplecss/enum.PseudoClass.html
		//https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-classes
		false
    }
}


pub(crate) trait SourceGenerator {
    fn begin(&mut self, elem_stack:&[Element], css:&StyleSheet) -> Result<(),Error> ;
    fn end(&mut self, elem_stack:&[Element], css:&StyleSheet) -> Result<(),Error> ;
}

pub struct DruidGenerator {
    writer : String
}

impl DruidGenerator {
    pub fn new() -> Self {
        Self {
            writer : String::new()
        }
    }

    pub fn get_parsed(&self) -> &str {
        self.writer.as_str()
    }

    pub fn into(self) -> String {
        self.writer
    }
}


impl SourceGenerator for DruidGenerator {
    fn begin(&mut self, elem_stack:&[Element], _css:&StyleSheet) -> Result<(),Error> {
        assert!( elem_stack.len() != 0 );
        let depth = elem_stack.len() - 1;
        let elem = &elem_stack[elem_stack.len()-1];
        let attrs = elem.attributes();
        macro_rules! writeln {
            ( $tab_add:tt , $($tts:tt)* ) => {
                write!(self.writer, "{}", std::iter::repeat('\t').take(depth+$tab_add).collect::<String>() ).unwrap();
                write!(self.writer, $($tts)* ).unwrap();
                write!(self.writer, "\n").unwrap();
            };
        }

        let tag = elem.tag();
        if tag.as_ref() == b"flex" && depth == 0 {
            let fnname = attrs.get_result("fn", elem.src_pos)?;
            writeln!(0 , "fn {}() -> impl Widget", String::from_utf8_lossy(&fnname));
        }

        writeln!(0,"{{\n");

        match tag.as_ref() {
            b"flex" => {
                //if elem.attrs.find( |e| if let Ok(e) = e { e.key.as_ref() == b"column" } else { false }).is_some() {
                if let Some( Cow::Borrowed(b"column") ) = attrs.get(b"direction") {
                    writeln!(1,"let flex = Flex::column();");
                } else {
                    writeln!(1,"let flex = Flex::row();");
                }
            }
            _ => ()
        }
        Ok(())
    }

    fn end(&mut self, elem_stack:&[Element], css:&StyleSheet) -> Result<(),Error> {
        let depth = elem_stack.len() - 1;
        let elem = &elem_stack[elem_stack.len()-1];

        let elem_query = ElementQueryWrap { stack : elem_stack };

        //just simplify ordered iteration without vec allocation (#id query first)
        //Reference : https://www.w3.org/TR/selectors/#specificity
        let css_iter = 
        css.rules.iter()
        .filter( |e| e.selector.specificity()[0] == 1 && e.selector.matches(&elem_query) )
        .chain(
            css.rules.iter()
            .filter( |e| e.selector.specificity()[0] != 1 && e.selector.matches(&elem_query) ) )
        .map( |e| &e.declarations );

        macro_rules! writeln {
            ( $tab_add:tt , $($tts:tt)* ) => {
                write!(self.writer, "{}", std::iter::repeat('\t').take(depth+$tab_add).collect::<String>() ).unwrap();
                write!(self.writer, $($tts)* ).unwrap();
                write!(self.writer, "\n").unwrap();
            };
        }

        let attrs = elem.attributes();
        let elem_style = attrs.get(b"style").unwrap_or( Cow::Borrowed(b"") );
        let elem_style_str = &String::from_utf8_lossy(&elem_style) as &str;
        let specific_style:Vec<Declaration> = DeclarationTokenizer::from( elem_style_str ).collect();

        macro_rules! get_style {
            ($name:tt) => {
                specific_style.iter().find( |e| e.name == $name ).map( |e| e.value ).or_else( || {
                    for global_style in css_iter.clone() {
                        let find = global_style.iter().find( |e| {println!("Find CSS : {} {}",e.name,$name); e.name == $name} ).map( |e| e.value );
                        if find.is_some() {
                            return find
                        }
                    }
                    None
                })
            }
        }

        let tag = elem.tag();

        //TODO : Wrap EnvSetup
        //TODO : Bind event
        match tag.as_ref() {
            b"flex" => {
                
            }
            b"label" => {
                let name = elem.text.as_ref().map( |e| String::from_utf8_lossy(&e) ).unwrap_or( std::borrow::Cow::Borrowed("Label") );
                writeln!(1,"let label = Label::new(\"{}\");", name );
                if let Some(color) = get_style!("color") {
                    writeln!(1,"label.set_text_color({});", CSSParse::color_attribute(color)? );
                }
            }
            b"button" => {
                let name = elem.text.as_ref().map( |e| String::from_utf8_lossy(&e) ).unwrap_or( std::borrow::Cow::Borrowed("Button") );
                writeln!(1,"let label_for_button = Label::new(\"{}\");", name );
                writeln!(1,"let button = Button::from_label(btn_label);");
            }
            _ => ()
        }

        writeln!(1,"{}", String::from_utf8_lossy(tag.as_ref()) ); //return element
        writeln!(0,"}}\n"); //close with return

        
        Ok(())
    }
}


struct CSSParse;

impl CSSParse {
    //TODO : Error check
    /// [O] : rgb(0,255,0)
    /// [O] : rgba(0,255,0,88)
    /// [O] : #FF33FF
    /// [O] : #FF33FF22
    /// [X] : rgb(100%, 0, 25%, 2)
    fn color_attribute(v:&str) -> Result<String,Error> {
        let tv = v.trim();    
        if tv.starts_with("rgba") && tv.ends_with(")") {
            Ok( format!("Color::rgba8({})", &tv[tv.find('(').unwrap() .. tv.rfind(')').unwrap()]) )
        } else if tv.starts_with("rgb") && tv.ends_with(")") {
            Ok( format!("Color::rgba({})", &tv[tv.find('(').unwrap() .. tv.rfind(')').unwrap()]) )
        } else {
            Ok( format!("Color::from_hex_str({})",v) )
        }
    }
}