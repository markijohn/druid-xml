

use std::borrow::Cow;
use quick_xml::events::BytesStart;
use quick_xml::name::{self, QName};
use simplecss::{Declaration, DeclarationTokenizer, StyleSheet};
use std::fmt::Write;

use crate::named_color;
use crate::{AttributeGetter, Element, Error};

/// stack[parent .. elem]
struct ElementQueryWrap<'a> {
	parent_stack : &'a [&'a Element<'a>],
    elem : &'a Element<'a>
}

impl <'a> simplecss::Element for ElementQueryWrap<'a> {
    fn parent_element(&self) -> Option<Self> {
        let len = self.parent_stack.len();
		if len > 0 {
			Some( ElementQueryWrap { 
                parent_stack:&self.parent_stack[..len-1], 
                elem:&self.parent_stack[len-1]
            } )
		} else {
			None
		}
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        let len = self.parent_stack.len();
        if len > 1 {
            let parent = &self.parent_stack[len-1];
            if let Some( (idx,_finded)) = parent.childs.iter().enumerate().find( |(_idx,e)| e.src_pos == self.elem.src_pos ) {
                if idx > 0 {
                    return Some( ElementQueryWrap { 
                        parent_stack:self.parent_stack,
                        elem:&parent.childs[idx-1] } )
                }
            }
		}
        None
    }

    fn has_local_name(&self, name: &str) -> bool {
        &self.elem.tag().0 == &name.as_bytes()
    }

    fn attribute_matches(&self, local_name: &str, operator: simplecss::AttributeOperator) -> bool {
		if let Some(v) = self.elem.attributes().get(local_name.as_bytes()) {
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
    fn write(&mut self, elem:&Element, css:&StyleSheet) -> Result<(),Error>;
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

impl DruidGenerator {
    fn impl_write(&mut self, parent_stack:&[&Element], elem:&Element, css:&StyleSheet) -> Result<(),Error> {
        let depth = parent_stack.len();
        let elem_query = ElementQueryWrap { parent_stack, elem };

        //just simplify ordered iteration without vec allocation (#id query first)
        //Reference : https://www.w3.org/TR/selectors/#specificity
        let css_iter = 
        css.rules.iter()
        .filter( |e| e.selector.specificity()[0] == 1 && e.selector.matches(&elem_query) )
        .chain(
            css.rules.iter()
            .filter( |e| e.selector.specificity()[0] != 1 && e.selector.matches(&elem_query) ) )
        .map( |e| &e.declarations );

        let tag_qname = elem.tag();
        let tag = String::from_utf8_lossy(tag_qname.as_ref());
        let mut tag_wrap:&str = &tag;

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

        macro_rules! _src {
            ( $tab_add:expr, $($tts:tt)* ) => {
                write!(self.writer, "{}", std::iter::repeat('\t').take($tab_add).collect::<String>() ).unwrap();
                write!(self.writer, $($tts)* ).unwrap();
            }
        }

        macro_rules! src {
            ( $($tts:tt)* ) => { {
                _src!( depth+1, $($tts)* );
            } }
        }

        macro_rules! style {
            ( $start:literal, $name:literal, $end:literal ) => {
                if $name == "width-height" {
                    if let (Some(width), Some(height)) = (get_style!("width") , get_style!("height")) {
                        CSSAttribute::size(&mut self.writer, width).unwrap();
                        _src!(0,",");
                        CSSAttribute::size(&mut self.writer, height).unwrap();
                    }
                } else if let Some(value) = get_style!($name) {
                    src!($start);
                    match $name {
                        "background-color" => CSSAttribute::color(&mut self.writer, value).unwrap(),
                        "color" => CSSAttribute::color(&mut self.writer, value).unwrap(),
                        "font-size" => CSSAttribute::font_size(&mut self.writer, value).unwrap(),
                        "border" => CSSAttribute::border_color_and_width(&mut self.writer, value).unwrap(),
                        "text-align" => CSSAttribute::text_align(&mut self.writer, value).unwrap(),
                        "placeholder" => { write!(self.writer, "{}", value ).unwrap() },
                        "object-fit" => CSSAttribute::object_fit(&mut self.writer, value).unwrap(),
                        "width" | "height" => CSSAttribute::size(&mut self.writer, value).unwrap(),
                        "image-rendering" => CSSAttribute::image_rendering(&mut self.writer, value).unwrap(),
                        _ => unimplemented!("unknown css attribute : {}", $name)
                    }
                    _src!( 0, $end );
                }
            }
        }

        macro_rules! new_parent_stack {
            () => {
                new_parent_stack!( elem )
            };
            ( $new:ident ) => { {
                let mut new_stack = parent_stack.to_owned();
                new_stack.push( $new );
                new_stack
            } }
        }
        
        let input_type_holder = &attrs.get(b"type").unwrap_or(Cow::Borrowed(b""));
        let input_type = String::from_utf8_lossy( input_type_holder );
        //TODO : Wrap EnvSetup
        //TODO : Bind event
        if tag == "flex" {
            if let Some( Cow::Borrowed(b"column") ) = attrs.get(b"direction") {
                src!("let mut flex = Flex::column();\n");
            } else {
                src!("let mut flex = Flex::row();\n");
            }
            if elem.childs.len() < 1 {
                return Err(Error::InvalidFlexChildNum((elem.src_pos)))
            }
            if let Some(v) = attrs.get(b"cross_axis_alignment") {
                let v = match v.as_ref() {
                    b"start" => "CrossAxisAlignment::Start",
                    b"center" => "CrossAxisAlignment::Center",
                    b"end" => "CrossAxisAlignment::End",
                    b"baseline" => "CrossAxisAlignment::Baseline",
                    _ => return Err(Error::InvalidAttributeValue((elem.src_pos, "cross_axis_alignment")))
                };
                src!("flex.set_cross_axis_alignment({});\n", v);
            }

            if let Some(v) = attrs.get(b"axis_alignment") {
                let v = match v.as_ref() {
                    b"start" => "MainAxisAlignment::Start",
                    b"center" => "MainAxisAlignment::Center",
                    b"end" => "MainAxisAlignment::End",
                    b"spacebetween" => "MainAxisAlignment::SpaceBetween",
                    b"spaceevenly" => "MainAxisAlignment::SpaceEvenly",
                    b"spacearound" => "MainAxisAlignment::SpaceAround",
                    _ => return Err(Error::InvalidAttributeValue((elem.src_pos, "axis_alignment")))
                };
                src!("flex.set_main_axis_alignment({});\n", v);
            }
            

            let new_stack = new_parent_stack!();
            for child in elem.childs.iter() {
                if child.tag().as_ref() == b"spacer" {
                    if let Some(flex) = child.attributes().get(b"flex") {
                        src!("flex.add_flex_spacer({});\n", String::from_utf8_lossy(&flex));
                    } else {
                        src!("flex.add_default_spacer( );\n");
                    }
                } else {
                    src!("let child = {{\n");
                    self.impl_write(&new_stack, child, css)?;
                    src!("}};\n");
                    if let Some(flex) =child.attributes().get(b"flex") {
                        src!("flex.add_flex_child(child, {});\n", String::from_utf8_lossy(&flex));
                    } else {
                        src!("flex.add_child( child );\n");
                    }
                }
            }
        }

        //WARN : checkbox is none-standard
        else if tag == "label" || tag == "button" || tag == "checkbox" || (tag == "input" && input_type == "checkbox") {
            let name = elem.text.as_ref().map( |e| String::from_utf8_lossy(&e) ).unwrap_or( std::borrow::Cow::Borrowed("Label") );
            src!("let mut label = Label::new(\"{}\");\n", name );
            style!("label.set_text_color(", "color", ");\n");
            style!("label.set_text_size(", "font-size", ");\n");
            style!("label.set_text_alignment(\"", "text-align", "\");\n");

            if tag == "button" {
                src!("let button = Button::from_label(label);\n");
            } else if tag == "checkbox" || (tag == "input" && input_type == "checkbox") {
                tag_wrap = "checkbox";
                src!("let checkbox = Checkbox::new(label);\n");
            }
        }

        //TODO : password type?
        else if tag == "textbox" || (tag == "input" && input_type == "text") {
            tag_wrap = "textbox";
            src!("let mut textbox = TextBox::new();\n" );
            style!("textbox.set_text_color(", "color", ");\n");
            style!("textbox.set_text_size(", "font-size", ");\n");
            style!("textbox.set_text_alignment(\"", "text-align", "\");\n");
            style!("textbox.set_place_holder(\"", "placeholder", "\");\n");
        }

        //WARN : "image" is none-standard
        else if tag == "image" || tag == "img" {
            tag_wrap = "image";
            let file_src_holder = &attrs.get_result("src", 0)?;
            let file_src = String::from_utf8_lossy( file_src_holder );
            //TODO : more speedup as raw binary data
            src!( "let image_buf = ImageBuf::from_bytes( inclue_bytes!(\"{}\") ).unwrap();\n", &file_src);
            src!( "let mut image = Image::new(image_buf);\n");
            style!( "image.set_fill_mode(\"", "object-fit" ,");\n");
            style!( "image.set_interpolation_mode(\"", "image-rendering" ,");\n");
        }

        //WARN : list is none-standard
        else if tag == "list" {
            style!( "let mut list = List::new(", "fn" ,");\n");
            if let Some( Cow::Borrowed(b"horizontal") ) = attrs.get(b"direction") {
                src!( "list = list.horizontal();\n");
            }
            style!( "list.set_spacing(", "spacing", ");\n");
        }

        else if tag == "scroll" {
            src!("let mut scroll = Scroll::new()")
        }

        else if tag == "slider" {

        }

        else if tag == "spinner" {

        }

        //TODO : child must be two item
        else if tag == "split" {
            if elem.childs.len() != 2 {
                return Err(Error::InvalidSplitChildNum(elem.src_pos))
            }
            let new_stack = new_parent_stack!();
            src!("let one = {{\n");
            self.impl_write(&new_stack, &elem.childs[0], css)?;
            src!("}};");

            src!("let two = {{\n");
            self.impl_write(&new_stack, &elem.childs[1], css)?;
            src!("}};");

            if let Some( Cow::Borrowed(b"column") ) = attrs.get(b"direction") {
                src!("let mut split = Split::columns(one, two);\n");
            } else {
                src!("let mut split = Split::rows(one, two);\n");
            }
            
        }

        else if tag == "stepper" {

        }

        else if tag == "switch" {

        }

        //TODO
        else if tag == "painter" || tag == "canvas" {
            
        }

        //WARN : container is none-standard
        else if tag == "container" {
            if elem.childs.len() != 1 {
                return Err(Error::InvalidContainerChildNum(elem.src_pos))
            }
            let new_stack = new_parent_stack!();
            src!("let child = {{\n");
            self.impl_write(&new_stack, &elem.childs[0], css)?;
            src!("}};\n");

            src!( "let mut container = Container::new(child);\n");
            style!("container.set_background(", "background-color", ");\n");
            style!("container.set_border(", "border", ");\n");
        }
        else {
            unimplemented!("Unknown tag : {}",tag);
        }


        //all component
        //background, padding, 
        {
            if attrs.get(b"width").is_some() && attrs.get(b"height").is_some() {
                style!("{tag_wrap} = {tag_wrap}.fix_size(" , "width-height", ");\n" );
            } else {
                style!("{tag_wrap} = {tag_wrap}.fix_width(" , "width", ");\n" );
                style!("{tag_wrap} = {tag_wrap}.fix_height(" , "height", ");\n" );    
            }
            
            style!("{tag_wrap} = {tag_wrap}.background(" , "background-color", ");\n" );
            style!("{tag_wrap} = {tag_wrap}.border(" , "border", ");\n" );
        }

        src!("{}\n", tag_wrap ); //return element

        Ok(())
    }
}


impl SourceGenerator for DruidGenerator {
    fn write(&mut self, elem:&Element, css:&StyleSheet) -> Result<(),Error> {
        self.impl_write(&mut vec![], elem, css)
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
        if tv.starts_with('#') {
            write!(w,"Color::from_hex_str({})", &tv[1..]).unwrap();
        } else if tv.starts_with("rgb") && tv.ends_with(')') {
            write!(w,"Color::rgba8({})", &tv[tv.find('(').unwrap() .. tv.rfind(')').unwrap()]).unwrap();
        } else if tv.starts_with("rgba") && tv.ends_with(')') {
            write!(w,"Color::rgba({})", &tv[tv.find('(').unwrap() .. tv.rfind(')').unwrap()]).unwrap();
        } else {
            if let Some(rgba) = named_color::named_color(v) {
                write!(w,"{}", rgba).unwrap();
            } else {
                return Err(Error::InvalidAttributeValue((0, "invalid color value")))
            }
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