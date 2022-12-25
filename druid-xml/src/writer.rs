use std::borrow::Cow;
use simplecss::{Declaration, DeclarationTokenizer, Rule};
use std::fmt::Write;

use crate::{AttributeGetter, Element, Error};


pub(crate) trait SourceGenerator {
    fn begin<'a,I:Iterator<Item=&'a Vec<Declaration<'a>>>>(&mut self, elem_stack:&[Element], styles:I) -> Result<(),Error> ;
    fn end<'a,I:Iterator<Item=&'a Vec<Declaration<'a>>>>(&mut self, elem_stack:&[Element], styles:I) -> Result<(),Error> ;
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
    fn begin<'a,I:Iterator<Item=&'a Vec<Declaration<'a>>>>(&mut self, elem_stack:&[Element], styles:I) -> Result<(),Error> {
        let mut depth = elem_stack.len();
        let elem = &elem_stack[depth-1];
        macro_rules! writeln {
            ( $($tts:tt)* ) => { {
                write!(self.writer, "{}", std::iter::repeat('\t').take(depth+1).collect::<String>() ).unwrap();
                write!(self.writer, $($tts)* ).unwrap();
                write!(self.writer, "\n").unwrap();
            } }
        }  
        match elem.tag().as_ref() {
            b"flex" => {
                let attrs = elem.attributes();
                if depth == 0 {
                    let fnname = attrs.get_result("fn", elem.src_pos)?;
                    write!(self.writer,"fn {}() {{\n", String::from_utf8_lossy(&fnname)).unwrap();
                }

                //if elem.attrs.find( |e| if let Ok(e) = e { e.key.as_ref() == b"column" } else { false }).is_some() {
                if let Some( Cow::Borrowed(b"column") ) = attrs.get(b"direction") {
                    writeln!("let flex = Flex::column()");
                } else {
                    writeln!("let flex = Flex::row()");
                }
            }
            _ => ()
        }
        Ok(())
    }

    fn end<'a,I:Iterator<Item=&'a Vec<Declaration<'a>>>>(&mut self, elem_stack:&[Element], styles:I) -> Result<(),Error> {
        let depth = elem_stack.len();
        let elem = &elem_stack[depth-1];
        macro_rules! writeln {
            ( $($tts:tt)* ) => { {
                write!(self.writer, "{}", std::iter::repeat('\t').take(depth+1).collect::<String>() ).unwrap();
                write!(self.writer, $($tts)* ).unwrap();
                write!(self.writer, "\n").unwrap();
            } }
        }

        let attrs = elem.attributes();
        let elem_style = attrs.get(b"style").unwrap_or( Cow::Borrowed(b"") );
        let style_decl = DeclarationTokenizer::from( &String::from_utf8_lossy(&elem_style) as &str );
        

        macro_rules! get_style {
            ($name:ident) => {
                style_decl.find( |e| e.name == name ).or_else( || {
                    for global_style in styles {
                        let find = global_style.iter().find( |e| e.name );
                        if find.is_some() {
                            return find
                        }
                    }
                    None
                });
            }
        }
        Ok(())
    }
}