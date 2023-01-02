

use std::borrow::Cow;
use quick_xml::name;
use simplecss::{Declaration, DeclarationTokenizer, StyleSheet};
use std::fmt::Write;

use crate::named_color;
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
        macro_rules! write_src {
            ( $tab_add:tt , $($tts:tt)* ) => {
                write!(self.writer, "{}", std::iter::repeat('\t').take(depth+$tab_add).collect::<String>() ).unwrap();
                write!(self.writer, $($tts)* ).unwrap();
            };
        }

        let tag_qname = elem.tag();
        let tag = String::from_utf8_lossy(tag_qname.as_ref());
        if depth == 0 {
            let fnname = attrs.get_result("fn", elem.src_pos)?;
            write_src!(0 , "fn {}() -> impl Widget\n", String::from_utf8_lossy(&fnname));
        } else {
            write_src!(0,"let child = {{\n");
        }

        match tag.as_ref() {
            "flex" => {
                //if elem.attrs.find( |e| if let Ok(e) = e { e.key.as_ref() == b"column" } else { false }).is_some() {
                if let Some( Cow::Borrowed(b"column") ) = attrs.get(b"direction") {
                    write_src!(1,"let flex = Flex::column();\n");
                } else {
                    write_src!(1,"let flex = Flex::row();\n");
                }
            }
            _ => ()
        }
        Ok(())
    }

    fn end(&mut self, elem_stack:&[Element], css:&StyleSheet) -> Result<(),Error> {
        assert!( elem_stack.len() != 0 );
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

        macro_rules! write_src {
            ( $tab_add:tt , $($tts:tt)* ) => {
                write!(self.writer, "{}", std::iter::repeat('\t').take(depth+$tab_add).collect::<String>() ).unwrap();
                write!(self.writer, $($tts)* ).unwrap();
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
                        let find = global_style.iter().find( |e| e.name == $name ).map( |e| e.value );
                        if find.is_some() {
                            return find
                        }
                    }
                    None
                })
            }
        }
        
        macro_rules! write_attr {
            ( $tab_add:tt , $start:literal, $attr:literal, $end:literal ) => {
                if let Some(value) = get_style!($attr) {
                    write!(self.writer, "{}", std::iter::repeat('\t').take(depth+$tab_add).collect::<String>() ).unwrap();
                    write!(self.writer, $start ).unwrap();
                    match $attr {
                        "color" => CSSAttribute::color(&mut self.writer, value)?,
                        "font-size" => CSSAttribute::font_size(&mut self.writer, value)?,
                        "border" => CSSAttribute::border_color_and_width(&mut self.writer, value)?,
                        "text-align" => CSSAttribute::text_align(&mut self.writer, value)?,
                        "placeholder" => { write!(self.writer, "{}", value ).unwrap() },
                        "object-fit" => CSSAttribute::object_fit(&mut self.writer, value)?,
                        _ => unimplemented!("unknown css attribute : {}", $attr)
                    }
                    write!(self.writer, $end ).unwrap();
                }
            };
        }

        let parent_tag_holder = if depth > 0 {
            Some(elem_stack[depth-1].tag())
        } else {
            None
        };
        let parent_tag = if let Some(parent_tag) = parent_tag_holder.as_ref() {
            Some( String::from_utf8_lossy(parent_tag.as_ref()) )
        } else {
            None
        };
        let tag_qname = elem.tag();
        let tag = String::from_utf8_lossy(tag_qname.as_ref());

        let input_type_holder = &attrs.get(b"type").unwrap_or(Cow::Borrowed(b""));
        let input_type = String::from_utf8_lossy( input_type_holder );
        //TODO : Wrap EnvSetup
        //TODO : Bind event
        if tag == "flex" {
            //None
        }

        else if tag == "label" || tag == "button" || tag == "checkbox" || (tag == "input" && input_type == "checkbox") {
            let name = elem.text.as_ref().map( |e| String::from_utf8_lossy(&e) ).unwrap_or( std::borrow::Cow::Borrowed("Label") );               
            write_src!(1,"let mut label = Label::new(\"{}\");\n", name );
            write_attr!(1,"label.set_text_color(", "color", ");\n");
            write_attr!(1,"label.set_text_size(", "font-size", ");\n");
            write_attr!(1,"label.set_text_alignment(\"", "text-align", "\");\n");
            if tag == "button" {
                write_src!(1,"let button = Button::from_label(label);\n");
            } else if tag == "checkbox" || (tag == "input" && input_type == "checkbox") {
                write_src!(1,"let checkbox = Checkbox::new(label);\n");
            }
        }

        //TODO : password type?
        else if tag == "textbox" || (tag == "input" && input_type == "text") {
            write_src!(1,"let mut textbox = TextBox::new();\n" );
            write_attr!(1,"textbox.set_text_color(", "color", ");\n");
            write_attr!(1,"textbox.set_text_size(", "font-size", ");\n");
            write_attr!(1,"textbox.set_place_holder(\"", "placeholder", "\");\n");
            write_attr!(1,"textbox.set_text_alignment(\"", "text-align", "\");\n");
        }

        else if tag == "image" || tag == "img" {
            let file_src = String::from_utf8_lossy( &attrs.get_result("src", 0)? );
            write_src!(1, "let image_buf = druid::ImageBuf::from_file(v).unwrap();\n");
            write_src!(1, "let mut image = druid::Image::new(image_buf);\n");
            write_attr!(1, "image.set_fill_mode(\"", "object-fit" ,");\n");
            write_attr!(1, "image.set_interpolation_mode(\"", "image-rendering" ,");\n");
        }

        else if tag == "container" {
            write_src!(1, "let mut container = Container::new(child);\n");
            write_attr!(1,"container.set_background(", "background-color", ");\n");
            write_attr!(1,"container.set_border(", "border", ");\n");
        }
        else {
            unimplemented!("Unknown tag : {}",tag);
        }
        write_src!(1,"{}\n", tag ); //return element
        write_src!(0,"}};\n"); //close with return

        //add to parent
        if depth > 0 {
            if let Some(parent_tag) = parent_tag {
                match parent_tag.as_ref() {
                    "flex" => { write_src!(0,"flex.with_child(child);\n"); }
                    "container" => { write_src!(0,"let container = Container::new(child);\n"); } //border-color,border-width,border-round,background
                    _ => ()
                }
            }
        }
        
        Ok(())
    }
}


struct CSSAttribute;

impl CSSAttribute {
    //TODO : Error check
    /// [O] : rgb(0,255,0)
    /// [O] : rgba(0,255,0,88)
    /// [O] : #FF33FF
    /// [O] : #FF33FF22
    /// [X] : rgb(100%, 0, 25%, 2)
    fn color(w:&mut String, v:&str) -> Result<(),Error> {
        let tv = v.trim();
        if tv.starts_with("#") {
            write!(w,"Color::from_hex_str({})", &tv[1..]).unwrap();
        } else if tv.ends_with("pt") {
            write!(w,"Color::rgba8({})", &tv[tv.find('(').unwrap() .. tv.rfind(')').unwrap()]).unwrap();
        } else if tv.starts_with("rgb") && tv.ends_with(")") {
            write!(w,"Color::rgba({})", &tv[tv.find('(').unwrap() .. tv.rfind(')').unwrap()]).unwrap();
        } else {
            return Err(Error::InvalidAttributeValue((0, "invalid color value")))
        }
        Ok(())
    }

    fn size(w:&mut String, v:&str) -> Result<(), Error> {
        let tv = v.trim();
        match tv.as_bytes() {
            [val @ .. , b'p', b'x'] => write!(w,"{}", String::from_utf8_lossy(val) ).unwrap(),
            [val @ .. , b'e', b'm'] => write!(w, "{}", String::from_utf8_lossy(val).parse::<f64>().map( |v| v / 0.0625).unwrap() ).unwrap(),
            val @ _ => write!(w, "{}", String::from_utf8_lossy(val).parse::<f64>().unwrap() ).unwrap()
        }
        Ok(())
    }

    //Reference : https://simplecss.eu/pxtoems.html or https://websemantics.uk/tools/font-size-conversion-pixel-point-em-rem-percent/
    fn font_size(w:&mut String, v:&str) -> Result<(),Error> {
        let tv = v.trim();
        match tv.as_bytes() {
            b"xx-small" => write!(w,"9" ).unwrap(),
            b"x-small" => write!(w,"10" ).unwrap(),
            b"small" => write!(w,"13.333" ).unwrap(),
            b"medium" => write!(w,"16" ).unwrap(),
            b"large" => write!(w,"18" ).unwrap(),
            b"x-large" => write!(w,"24" ).unwrap(),
            b"xx-large" => write!(w,"32" ).unwrap(),
            [val @ .. , b'p', b'x'] => write!(w,"{}", String::from_utf8_lossy(val) ).unwrap(),
            [val @ .. , b'e', b'm'] => write!(w, "{}", String::from_utf8_lossy(val).parse::<f64>().map( |v| v / 0.0625).unwrap() ).unwrap(),
            [val @ .. , b'p', b't'] => write!(w, "{}", String::from_utf8_lossy(val).parse::<f64>().map( |v| v * 1.333).unwrap() ).unwrap() ,
            [val @ .. , b'%'] => write!(w, "{}", String::from_utf8_lossy(val).parse::<f64>().map( |v| v / 100f64 / 0.0625 ).unwrap() ).unwrap(),
            val @ _ => write!(w, "{}", String::from_utf8_lossy(val).parse::<f64>().unwrap() ).unwrap()
        }
        Ok(())
    }

    fn text_align(w:&mut String, v:&str) -> Result<(), Error> {
        match v {
            "left" => write!(w, "druid::TextAlignment::Start").unwrap(),
            "right" => write!(w, "druid::TextAlignment::End").unwrap(),
            "center" => write!(w, "druid::TextAlignment::Center").unwrap(),
            "justify" => write!(w, "druid::TextAlignment::Justify").unwrap(),
            _ => return Err( Error::InvalidAttributeValue((0,"text-align")) )
        }
        Ok(())
    }

    fn border_color_and_width(w:&mut String, v:&str) -> Result<(), Error> {
        let mut splited = v.split_whitespace();
        let width = splited.next().map( |v| v[..v.find("px").unwrap_or(v.len())].parse::<f64>().unwrap() ).unwrap_or(1f64);
        //TODO : support other border style?
        let _border_style = splited.next().unwrap_or("solid");
        if _border_style != "solid" {
            Err(Error::InvalidAttributeValue((0,"border")))
        } else {
            let color = splited.next().unwrap_or("black");
            write!(w,"{},", width).unwrap();
            Self::color(w,color)
        }
    }

    //https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit
    fn object_fit(w:&mut String, v:&str) -> Result<(), Error> {
        match v.trim() {
            "none" => write!(w,"FillStart::None").unwrap(), //Do not scale
            "fill" | "" => write!(w,"FillStart::Fill").unwrap(), //(default) Fill the widget with no dead space, aspect ratio of widget is used
            "contain" => write!(w,"FillStart::Contain").unwrap(), //As large as posible without changing aspect ratio of image and all of image shown
            "cover" => write!(w,"FillStart::Cover").unwrap(), //As large as posible with no dead space so that some of the image may be clipped
            "scale-down" => write!(w,"FillStart::ScaleDown").unwrap(), //Scale down to fit but do not scale up

            //WARN : None-standard css attribute
            "fit-width" => write!(w,"FillStart::FitWidth").unwrap(), //Fill the width with the images aspect ratio, some of the image may be clipped
            "fit-height" => write!(w,"FillStart::FitHeight").unwrap(), //Fill the hight with the images aspect ratio, some of the image may be clipped
            _ => return Err(Error::InvalidAttributeValue((0,"object-fit")))
        }
        Ok(())
    }

    //https://developer.mozilla.org/en-US/docs/Web/CSS/image-rendering
    fn image_rendering(w:&mut String, v:&str) -> Result<(), Error> {
        match v.trim() {
            //TODO 
            "auto" | "smooth" | "high-quality" | "crisp-edges" => write!(w,"InterpolationMode::Bilinear").unwrap(),

            "pixelated" => write!(w,"InterpolationMode::NearestNeighbor").unwrap(),

            _ => return Err(Error::InvalidAttributeValue((0,"image_rendering")))
        }
        Ok(())
    }
}