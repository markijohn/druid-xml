

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
    fn write_raw(&mut self,code:&str) -> Result<(),Error>;
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
                        "padding" => CSSAttribute::padding(&mut self.writer, value).unwrap(),
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

        macro_rules! attr {
            ($start:literal, $attr:literal, $end:literal) => {
                attr!(attrs, $start, $attr, $end);
            };
            ($target:ident, $start:literal, $attr:literal, $end:literal) => {
                if let Some(attr) = $target.get($attr) {
                    src!( $start );
                    _src!( 0, "{}", String::from_utf8_lossy( &attr ) );
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
                src!("let mut flex = druid::widget::Flex::column();\n");
            } else {
                src!("let mut flex = druid::widget::Flex::row();\n");
            }
            if elem.childs.len() < 1 {
                return Err(Error::InvalidFlexChildNum((elem.src_pos)))
            }

            attr!("flex = flex.must_fill_main_axis(", b"must_fill_main_axis", ");\n");

            if let Some(v) = attrs.get(b"cross_axis_alignment") {
                let v = match v.as_ref() {
                    b"start" => "druid::widget::CrossAxisAlignment::Start",
                    b"center" => "druid::widget::CrossAxisAlignment::Center",
                    b"end" => "druid::widget::CrossAxisAlignment::End",
                    b"baseline" => "druid::widget::CrossAxisAlignment::Baseline",
                    _ => return Err(Error::InvalidAttributeValue((elem.src_pos, "cross_axis_alignment")))
                };
                src!("flex.set_cross_axis_alignment({});\n", v);
            }

            if let Some(v) = attrs.get(b"axis_alignment") {
                let v = match v.as_ref() {
                    b"start" => "druid::widget::MainAxisAlignment::Start",
                    b"center" => "druid::widget::MainAxisAlignment::Center",
                    b"end" => "druid::widget::MainAxisAlignment::End",
                    b"spacebetween" => "druid::widget::MainAxisAlignment::SpaceBetween",
                    b"spaceevenly" => "druid::widget::MainAxisAlignment::SpaceEvenly",
                    b"spacearound" => "druid::widget::MainAxisAlignment::SpaceAround",
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
                        src!("flex.add_flex_child(child, {}f64);\n", String::from_utf8_lossy(&flex));
                    } else {
                        src!("flex.add_child( child );\n");
                    }
                }
            }
        }

        //WARN : checkbox is none-standard
        else if tag == "label" || tag == "button" || tag == "checkbox" || (tag == "input" && input_type == "checkbox") {
            let name = elem.text.as_ref().map( |e| String::from_utf8_lossy(&e) ).unwrap_or( std::borrow::Cow::Borrowed("Label") );
            src!("let mut label = druid::widget::Label::new(\"{}\");\n", name );
            style!("label.set_text_color(", "color", ");\n");
            style!("label.set_text_size(", "font-size", ");\n");
            style!("label.set_text_alignment(\"", "text-align", "\");\n");

            if tag == "button" {
                src!("let button = druid::widget::Button::from_label(label);\n");
            } else if tag == "checkbox" || (tag == "input" && input_type == "checkbox") {
                tag_wrap = "checkbox";
                src!("let checkbox = Checkbox::new(label);\n");
            }
        }

        //TODO : password type?
        else if tag == "textbox" || (tag == "input" && input_type == "text") {
            tag_wrap = "textbox";
            src!("let mut textbox = druid::widget::TextBox::new();\n" );
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
            src!( "let image_buf = druid::ImageBuf::from_bytes( inclue_bytes!(\"{}\") ).unwrap();\n", &file_src);
            src!( "let mut image = druid::widget::Image::new(image_buf);\n");
            style!( "image.set_fill_mode(\"", "object-fit" ,");\n");
            style!( "image.set_interpolation_mode(\"", "image-rendering" ,");\n");
        }

        //WARN : list is none-standard
        else if tag == "list" {
            style!( "let mut list = druid::widget::List::new(", "fn" ,");\n");
            if let Some( Cow::Borrowed(b"horizontal") ) = attrs.get(b"direction") {
                src!( "list = list.horizontal();\n");
            }
            style!( "list.set_spacing(", "spacing", ");\n");
        }

        else if tag == "scroll" {
            if elem.childs.len() != 1 {
                return Err(Error::InvalidScrollChildNum(elem.src_pos))
            }
            let new_stack = new_parent_stack!();
            src!("let child = {{\n");
            self.impl_write(&new_stack, &elem.childs[0], css)?;
            src!("}};\n");
            src!("let mut scroll = druid::widget::Scroll::new(child);\n");
        }

        else if tag == "slider" {
            let min = attrs.get_as::<f64>("min", elem.src_pos).unwrap_or(0f64);
            let max = attrs.get_as::<f64>("max", elem.src_pos).unwrap_or(1f64);
            src!("let mut slider = Slider::with_range({min},{max})");
        }

        else if tag == "spinner" {
            src!("let mut spinner = druid::widget::Spinner::new();\n");
            style!("spinner.set_color(", "color", ");\n");
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
                src!("let mut split = druid::widget::Split::columns(one, two);\n");
            } else {
                src!("let mut split = druid::widget::Split::rows(one, two);\n");
            }
            
            attr!("split = split.split_point(", b"split_point", "f64);\n");
            attr!("split = split.min_size(", b"min_size", "f64);\n");
            attr!("split = split.bar_size(", b"bar_size", "f64);\n");
            attr!("split = split.min_bar_area(", b"min_bar_area", "f64);\n");
            attr!("split = split.draggable(", b"draggable", ");\n");
            attr!("split = split.solid_bar(", b"solid_bar", ");\n");

            // if let Some(v) = attrs.get(b"split_point") {
            //     src!("split = split.split_pointer({});\n", String::from_utf8_lossy(&v) );
            // }
            // if let Some(v) = attrs.get(b"min_size") {
            //     src!("split = split.min_size({});\n", String::from_utf8_lossy(&v) );
            // }
            // if let Some(v) = attrs.get(b"bar_size") {
            //     src!("split = split.bar_size({});\n", String::from_utf8_lossy(&v) );
            // }
            // if let Some(v) = attrs.get(b"min_bar_area") {
            //     src!("split = split.min_bar_area({});\n", String::from_utf8_lossy(&v) );
            // }
            // if let Some(v) = attrs.get(b"draggable") {
            //     src!("split = split.min_bar_area({});\n", String::from_utf8_lossy(&v) );
            // }
            // if let Some(v) = attrs.get(b"solid_bar") {
            //     src!("split = split.solid_bar({});\n", String::from_utf8_lossy(&v) );
            // }
        }

        else if tag == "stepper" {
            let min = attrs.get_as::<f64>("min", elem.src_pos).unwrap_or(std::f64::MIN);
            let max = attrs.get_as::<f64>("max", elem.src_pos).unwrap_or(std::f64::MAX);
            let step = attrs.get_as::<f64>("step", elem.src_pos).unwrap_or(std::f64::MAX);
            let wrap = attrs.get_as::<bool>("wraparound", elem.src_pos).unwrap_or(false);
            src!("let mut stepper = druid::widget::Stepper::with_range({min},{max});\n");
            src!("stepper = stepper.with_step({step});\n");
            src!("stepper = stepper.with_wraparound({wrap});\n");
        }

        else if tag == "switch" {
            //TODO : style
            src!("let mut switch = Switch::new();\n");
        }

        //TODO
        else if tag == "painter" || tag == "canvas" {
            tag_wrap = "painter";
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

            src!( "let mut container = druid::widget::Container::new(child);\n");
            style!("container.set_background(", "background-color", ");\n");
            style!("container.set_border(", "border", ");\n");
        }
        else {
            unimplemented!("Unknown tag : {}",tag);
        }


        //all component
        //background, padding, 
        {
            //wrap Lens
            if depth > 0 {
                if let Some(lens) = attrs.get(b"lens") {
                    let lens = String::from_utf8_lossy(&lens);
                    src!("let {tag_wrap} = druid::WidgetExt::lens({tag_wrap}, {lens});\n");
                }
            }

            //wrap `Padding`
            style!("let {tag_wrap} = druid::WidgetExt::padding({tag_wrap}, " , "padding", ");\n" );

            //wrap `SizedBox` with optimize
            if attrs.get(b"width").is_some() && attrs.get(b"height").is_some() {
                style!("let {tag_wrap} = druid::WidgetExt::fix_size({tag_wrap}, " , "width-height", ");\n" );
            } else {
                style!("let {tag_wrap} = druid::WidgetExt::fix_width({tag_wrap}, " , "width", ");\n" );
                style!("let {tag_wrap} = druid::WidgetExt::fix_height({tag_wrap}, " , "height", ");\n" );    
            }
            
            //wrap 'Container' 
            if tag != "container" {
                style!("let {tag_wrap} = druid::WidgetExt::background({tag_wrap}, " , "background-color", ");\n" );
                style!("let {tag_wrap} = druid::WidgetExt::border({tag_wrap}, " , "border", ");\n" );
            }
        }

        src!("{}\n", tag_wrap ); //return element

        Ok(())
    }
}


impl SourceGenerator for DruidGenerator {
    fn write_raw(&mut self, code:&str) -> Result<(),Error> {
        self.writer.push_str(code);
        Ok(())
    }

    fn write(&mut self, elem:&Element, css:&StyleSheet) -> Result<(),Error> {
        self.impl_write(&mut vec![], elem, css)
    }
}


struct CSSAttribute;

impl CSSAttribute {
    fn padding(w:&mut String, v:&str) -> Result<(),Error> {
        let mut splits = v.split_whitespace();
        let count = splits.clone().count();
        if count == 1 {
            Self::size(w, splits.next().unwrap())?
        } else if count == 2 || count == 4 {
            write!(w,"(").unwrap();
            splits.for_each(|e| write!(w,"{},",e).unwrap() );
            write!(w,")").unwrap();
        } else {
            panic!("The number of padding parameters must be one of 1,2,4. but \"{}\"",v);
        }
        Ok(())
    }

    //TODO : Error check
    /// [O] : rgb(0,255,0)
    /// [O] : rgba(0,255,0,88)
    /// [O] : #FF33FF
    /// [O] : #FF33FF22
    /// [X] : rgb(100%, 0, 25%, 2)
    fn color(w:&mut String, v:&str) -> Result<(),Error> {
        let tv = v.trim();
        if tv.starts_with('#') {
            write!(w,"druid::Color::from_hex_str({})", &tv[1..]).unwrap();
        } else if tv.starts_with("rgb") && tv.ends_with(')') {
            write!(w,"druid::Color::rgba8({})", &tv[tv.find('(').unwrap() .. tv.rfind(')').unwrap()]).unwrap();
        } else if tv.starts_with("rgba") && tv.ends_with(')') {
            write!(w,"druid::Color::rgba({})", &tv[tv.find('(').unwrap() .. tv.rfind(')').unwrap()]).unwrap();
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
            [val @ .. , b'p', b'x'] => write!(w,"{}f64", String::from_utf8_lossy(val) ).unwrap(),
            [val @ .. , b'e', b'm'] => write!(w, "{}f64", String::from_utf8_lossy(val).parse::<f64>().map( |v| v / 0.0625).unwrap() ).unwrap(),
            val @ _ => write!(w, "{}f64", String::from_utf8_lossy(val).parse::<f64>().unwrap() ).unwrap()
        }
        Ok(())
    }

    //Reference : https://simplecss.eu/pxtoems.html or https://websemantics.uk/tools/font-size-conversion-pixel-point-em-rem-percent/
    fn font_size(w:&mut String, v:&str) -> Result<(),Error> {
        let tv = v.trim();
        match tv.as_bytes() {
            b"xx-small" => write!(w,"9f64" ).unwrap(),
            b"x-small" => write!(w,"10f64" ).unwrap(),
            b"small" => write!(w,"13.333f64" ).unwrap(),
            b"medium" => write!(w,"16f64" ).unwrap(),
            b"large" => write!(w,"18f64" ).unwrap(),
            b"x-large" => write!(w,"24f64" ).unwrap(),
            b"xx-large" => write!(w,"32f64" ).unwrap(),
            [val @ .. , b'p', b'x'] => write!(w,"{}f64", String::from_utf8_lossy(val) ).unwrap(),
            [val @ .. , b'e', b'm'] => write!(w, "{}f64", String::from_utf8_lossy(val).parse::<f64>().map( |v| v / 0.0625).unwrap() ).unwrap(),
            [val @ .. , b'p', b't'] => write!(w, "{}f64", String::from_utf8_lossy(val).parse::<f64>().map( |v| v * 1.333).unwrap() ).unwrap() ,
            [val @ .. , b'%'] => write!(w, "{}f64", String::from_utf8_lossy(val).parse::<f64>().map( |v| v / 100f64 / 0.0625 ).unwrap() ).unwrap(),
            val @ _ => write!(w, "{}f64", String::from_utf8_lossy(val).parse::<f64>().unwrap() ).unwrap()
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
            "none" => write!(w,"druid::widget::FillStart::None").unwrap(), //Do not scale
            "fill" | "" => write!(w,"druid::widget::FillStart::Fill").unwrap(), //(default) Fill the widget with no dead space, aspect ratio of widget is used
            "contain" => write!(w,"druid::widget::illStart::Contain").unwrap(), //As large as posible without changing aspect ratio of image and all of image shown
            "cover" => write!(w,"druid::widget::FillStart::Cover").unwrap(), //As large as posible with no dead space so that some of the image may be clipped
            "scale-down" => write!(w,"druid::widget::FillStart::ScaleDown").unwrap(), //Scale down to fit but do not scale up

            //WARN : None-standard css attribute
            "fit-width" => write!(w,"druid::widget::FillStart::FitWidth").unwrap(), //Fill the width with the images aspect ratio, some of the image may be clipped
            "fit-height" => write!(w,"druid::widget::FillStart::FitHeight").unwrap(), //Fill the hight with the images aspect ratio, some of the image may be clipped
            _ => return Err(Error::InvalidAttributeValue((0,"object-fit")))
        }
        Ok(())
    }

    //https://developer.mozilla.org/en-US/docs/Web/CSS/image-rendering
    fn image_rendering(w:&mut String, v:&str) -> Result<(), Error> {
        match v.trim() {
            //TODO 
            "auto" | "smooth" | "high-quality" | "crisp-edges" => write!(w,"InterpolationMode::Bilinear").unwrap(),

            "pixelated" => write!(w,"druid::piet::InterpolationMode::NearestNeighbor").unwrap(),

            _ => return Err(Error::InvalidAttributeValue((0,"image_rendering")))
        }
        Ok(())
    }
}