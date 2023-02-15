

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use quick_xml::events::BytesStart;
use quick_xml::name::{self, QName};
use simplecss::{Declaration, DeclarationTokenizer, StyleSheet, PseudoClass};
use std::fmt::Write;

use crate::simple_style::Pseudo;
use crate::{named_color, AttributesWrapper};
use crate::{AttributeGetter, Element, Error};


pub(crate) struct PseudoOrderTrapQueryWrap<'a> {
    pub pseudo : Rc<RefCell<Option<Pseudo>>>,
    pub origin : ElementQueryWrap<'a>,
}

impl <'a> PseudoOrderTrapQueryWrap<'a> {
    pub fn new(origin:ElementQueryWrap<'a>) -> Self {
        Self { pseudo : Rc::new(RefCell::new(None)), origin }
    }

    pub fn get_pseudo(self) -> Option<Pseudo> {
        self.pseudo.take()
    }
}


impl <'a> simplecss::Element for PseudoOrderTrapQueryWrap<'a> {
    fn parent_element(&self) -> Option<Self> {
        self.origin.parent_element().map( |origin| {
            Self { pseudo:self.pseudo.clone(), origin }
        })
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.origin.prev_sibling_element().map( |origin| {
            Self { pseudo:self.pseudo.clone(), origin }
        })
    }

    fn has_local_name(&self, name: &str) -> bool {
        self.origin.has_local_name(name)
    }

    fn attribute_matches(&self, local_name: &str, operator: simplecss::AttributeOperator) -> bool {
        self.origin.attribute_matches(local_name, operator)
    }

    fn pseudo_class_matches(&self, _class: simplecss::PseudoClass) -> bool {
        let mut cell = self.pseudo.borrow_mut();
        if cell.is_none() {
            match _class {
                PseudoClass::Hover => { cell.replace( Pseudo::Hover ); true},
                PseudoClass::Active => { cell.replace( Pseudo::Active ); true},
                PseudoClass::Focus => { cell.replace( Pseudo::Focus ); true},
                _ => false
            }
        } else {
            false
        }
    }
}

/// stack[parent .. elem]
pub(crate) struct ElementQueryWrap<'a> {
	pub parent_stack : &'a [&'a Element<'a>],
    pub elem : &'a Element<'a>
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
		if let Some(v) = self.elem.attributes(None).get(local_name.as_bytes()) {
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
    fn write(&mut self, parsed_map:&HashMap<String,Element>, elem:&Element, css:&StyleSheet, wrappers:&HashMap<String,String>) -> Result<(),Error>;
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
    fn impl_write<'a>(&mut self, parameter:Option<&AttributesWrapper<'a>>, parsed_map:&HashMap<String,Element>, parent_stack:&[&Element], elem:&Element, css:&StyleSheet, wrappers:&HashMap<String,String>) -> Result<(),Error> {
        let depth = parent_stack.len();
        let elem_query = ElementQueryWrap { parent_stack, elem };

        //just simplify ordered iteration without vec allocation (#id query first)
        //Reference : https://www.w3.org/TR/selectors/#specificity
        let css_iter = 
        css.rules.iter()

        //id first
        .filter( |e| { let spec = e.selector.specificity(); spec[0] > 0 && e.selector.matches(&elem_query) } )
    
        //class 
        .chain(
            css.rules.iter()
            .filter( |e| { let spec = e.selector.specificity(); spec[0] == 0 && spec[1] > 0 && e.selector.matches(&elem_query) } ) )
    
        //global types
        .chain(
            css.rules.iter()
            .filter( |e| { let spec = e.selector.specificity(); spec[0] == 0 && spec[1] == 0 && spec[2] > 0 && e.selector.matches(&elem_query) } ) )
        .map( |e| &e.declarations );

        let tag_qname = elem.tag();
        let tag = String::from_utf8_lossy(tag_qname.as_ref());
        let mut tag_wrap:&str = &tag;

        let attrs = elem.attributes(parameter);
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

        macro_rules! attr_write {
            ( $name:literal, $value:ident ) => {
                match $name {
                    "margin" => CSSAttribute::padding(&mut self.writer, $value).unwrap(),
                    "padding" => CSSAttribute::padding(&mut self.writer, $value).unwrap(),
                    "background-color" => CSSAttribute::color(&mut self.writer, $value).unwrap(),
                    "color" => CSSAttribute::color(&mut self.writer, $value).unwrap(),
                    "font-size" => CSSAttribute::font_size(&mut self.writer, $value).unwrap(),
                    "border" => CSSAttribute::border_color_and_width(&mut self.writer, get_style!("border-radius"), $value).unwrap(),
                    "text-align" => CSSAttribute::text_align(&mut self.writer, $value).unwrap(),
                    "placeholder" => { write!(self.writer, "{}", $value ).unwrap() },
                    "object-fit" => CSSAttribute::object_fit(&mut self.writer, $value).unwrap(),
                    "width" | "height" => CSSAttribute::size(&mut self.writer, $value).unwrap(),
                    "image-rendering" => CSSAttribute::image_rendering(&mut self.writer, $value).unwrap(),
                    _ => unimplemented!("unknown css attribute : {}", $name)
                }
            }
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
                    attr_write!($name, value);
                    _src!( 0, $end );
                }
            }
        }

        macro_rules! style_opt {
            ( $name:literal ) => {
                style_opt!("", $name, "");
            };
            ( $start:literal, $name:literal, $end:literal ) => {
                if let Some(value) = get_style!($name) {
                    _src!( 0, "Some(");
                    _src!( 0, $start);
                    attr_write!($name, value);
                    _src!( 0, $end );
                    _src!( 0, ")" );
                } else {
                    _src!( 0, "None" );
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

        let mut text = elem.text.as_ref().map( |e| String::from_utf8_lossy(&e) ).unwrap_or( std::borrow::Cow::Borrowed("") );

        if text.starts_with("${") && text.ends_with('}') {
            let key = text[2..text.len()-1].trim();
            if let Some(parameter) = parameter {
                if let Some(param_value) = parameter.get( key.as_bytes() ) {
                    text = Cow::Owned(String::from_utf8_lossy(&param_value).as_ref().to_owned());
                    //TODO : how to avoid?
                    // text = String::from_utf8_lossy(param_value);
                }
            } 
        }
        
        let input_type_holder = &attrs.get(b"type").unwrap_or(Cow::Borrowed(b""));
        let input_type = String::from_utf8_lossy( input_type_holder );

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
                    if let Some(flex) = child.attributes(None).get(b"flex") {
                        src!("flex.add_flex_spacer({});\n", String::from_utf8_lossy(&flex));
                    } else {
                        src!("flex.add_default_spacer( );\n");
                    }
                } else {
                    src!("let child = {{\n");
                    self.impl_write(parameter, parsed_map, &new_stack, child, css, wrappers)?;
                    src!("}};\n");
                    if let Some(flex) = child.attributes(None).get(b"flex") {
                        src!("flex.add_flex_child(child, {}f64);\n", String::from_utf8_lossy(&flex));
                    } else {
                        src!("flex.add_child( child );\n");
                    }
                }
            }
        }

        //WARN : checkbox is none-standard
        else if tag == "label" || tag == "button" {
            let label_text = if text == "" {
                tag.as_ref()
            } else {
                &text
            };
            src!("let mut label = druid::widget::Label::new(\"{label_text}\");\n" );
            //src!("let mut label = druid_xml::widget::DXLabel::new(\"{label_text}\");\n" );
            // style!("label.set_text_color(", "color", ");\n");
            // style!("label.set_text_size(", "font-size", ");\n");
            style!("label.set_text_alignment(\"", "text-align", "\");\n");

            if tag == "button" {
                src!("let button = druid::widget::Button::from_label(label);\n");
                //src!("let button = druid_xml::widget::DXButton::from_label(label);\n");
            }
        }

        else if tag == "checkbox" || (tag == "input" && input_type == "checkbox") {
            //TODO : checkbox has not label like button. color and text_size
            tag_wrap = "checkbox";
            let label_text = if text == "" {
                "checkbox"
            } else {
                &text
            };
            src!("let checkbox = druid::widget::Checkbox::new(\"{label_text}\");\n");
        }

        //TODO : password type?
        else if tag == "textbox" || (tag == "input" && input_type == "text") {
            tag_wrap = "textbox";
            src!("let mut textbox = druid::widget::TextBox::new();\n" );
            style!("textbox.set_text_color(", "color", ");\n");
            style!("textbox.set_text_size(", "font-size", ");\n");
            style!("textbox.set_text_alignment(\"", "text-align", "\");\n");
            if let Some(placeholder) = attrs.get_as::<String>(b"placeholder") {
                src!("textbox.set_placeholder(\"{placeholder}\");\n");
            }
        }

        //WARN : "image" is none-standard
        else if tag == "image" || tag == "img" {
            tag_wrap = "image";
            let file_src_holder = &attrs.get_result("src")?;
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
            self.impl_write(parameter, parsed_map, &new_stack, &elem.childs[0], css, wrappers)?;
            src!("}};\n");
            src!("let mut scroll = druid::widget::Scroll::new(child);\n");
        }

        else if tag == "slider" {
            let min = attrs.get_as_result::<f64>("min").unwrap_or(0f64);
            let max = attrs.get_as_result::<f64>("max").unwrap_or(1f64);
            src!("let mut slider = Slider::new;\n");
            src!("slider = slider.with_range({min},{max});\n");
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
            self.impl_write(parameter, parsed_map, &new_stack, &elem.childs[0], css, wrappers)?;
            src!("}};");

            src!("let two = {{\n");
            self.impl_write(parameter, parsed_map, &new_stack, &elem.childs[1], css, wrappers)?;
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
        }

        else if tag == "stepper" {
            let min = attrs.get_as_result::<f64>("min").unwrap_or(std::f64::MIN);
            let max = attrs.get_as_result::<f64>("max").unwrap_or(std::f64::MAX);
            let step = attrs.get_as_result::<f64>("step").unwrap_or(std::f64::MAX);
            let wrap = attrs.get_as_result::<bool>("wraparound").unwrap_or(false);
            src!("let mut stepper = Stepper::new();\n");
            src!("let mut stepper = stepper.with_range({min},{max});\n");
            src!("stepper = stepper.with_step({step});\n");
            src!("stepper = stepper.with_wraparound({wrap});\n");
        }

        else if tag == "switch" {
            //TODO : style
            src!("let mut switch = druid::widget::Switch::new();\n");
        }

        //TODO
        else if tag == "painter" || tag == "canvas" {
            tag_wrap = "painter";
        }

        //The Container has been replaced by SimpleStyleWidget.
        // //WARN : container is none-standard
        // else if tag == "container" {
        //     if elem.childs.len() != 1 {
        //         return Err(Error::InvalidContainerChildNum(elem.src_pos))
        //     }
        //     let new_stack = new_parent_stack!();
        //     src!("let child = {{\n");
        //     self.impl_write(parameter, parsed_map, &new_stack, &elem.childs[0], css, wrappers)?;
        //     src!("}};\n");
        //     src!( "let mut container = druid::widget::Container::new(child);\n");
        //     style!("container.set_background(", "background-color", ");\n");
        //     style!("container.set_border(", "border", ");\n");
        // }
        else {
            tag_wrap = "custom_widget";
            if let Some(elem) = parsed_map.get( tag.as_ref() ) {
                let new_stack = new_parent_stack!();
                src!("let custom_widget = {{\n");
                self.impl_write(Some(&attrs), parsed_map, &new_stack, elem, css, wrappers)?;
                src!("}};\n");
            } else {
                src!("let custom_widget = {tag}();\n");
                //return Err(Error::UnknownTag( (elem.src_pos, tag.as_ref().to_owned() )));
            }
        }

        //build styler
        //TODO : 
        // - need optimization for duplicated style(use Rc)
        // - inherit style
        {
            fn parse_time(v:&str) -> u64 {
                let v = v.to_lowercase();
                if v.ends_with("s") {
                    u64::from_str_radix( &v[..v.len()-1], 10 ).unwrap_or(0) * 1000_000_000
                } else if v.ends_with("ms") {
                    u64::from_str_radix( &v[..v.len()-2], 10 ).unwrap_or(0) * 1000_000
                } else {
                    u64::from_str_radix( v.as_str(), 10 ).unwrap() * 1000_000
                }
            }

            fn transition_option(define:&str, item:&str) -> String {
                for n in define.split(",") {
                    let mut duration = 0;
                    let mut delay = 0;
                    let mut timing_function = Cow::Borrowed("druid_xml::simple_style::TimingFunction::Linear");
                    //[sec] [name] => duration,item
                    //[name] [sec] => duration,item
                    //[sec] [name] [sec] => duration,item,delay
                    let mut wsplited = n.split_whitespace();
                    let expect_duration = wsplited.next().unwrap();
                    let expect_property = if expect_duration.chars().next().unwrap().is_numeric() {
                        //duration
                        duration = parse_time(expect_duration);
                        if duration == 0 {
                            //ignore
                            continue;
                        }
                        if let Some(s) = wsplited.next() {
                            s
                        } else {
                            continue
                        }
                    } else {
                        //pass
                        expect_duration
                    };

                    if expect_property != item {
                        //not target
                        continue;
                    }

                    let expect_delay = if let Some(s) = wsplited.next() {
                        s
                    } else {
                        "0"
                    };

                    let expect_tf = if expect_delay.chars().next().unwrap().is_numeric() {
                        //delay
                        delay = parse_time(expect_delay);
                        if let Some(s) = wsplited.next() {
                            s
                        } else {
                            "linear"
                        }
                    } else {
                        //timning-funciton
                        expect_delay
                    };

                    timing_function = match expect_tf {
                        "ease" => Cow::Borrowed("druid_xml::simple_style::TimingFunction::Ease"),
                        "ease-in" => Cow::Borrowed("druid_xml::simple_style::TimingFunction::EaseIn"),
                        "ease-out" => Cow::Borrowed("druid_xml::simple_style::TimingFunction::EaseOut"),
                        "ease-in-out" => Cow::Borrowed("druid_xml::simple_style::TimingFunction::EaseInOut"),
                        "linear" => Cow::Borrowed("druid_xml::simple_style::TimingFunction::Linear"),
                        "step-start" => Cow::Borrowed("druid_xml::simple_style::TimingFunction::Ease"),
                        "step-end" => Cow::Borrowed("druid_xml::simple_style::TimingFunction::Ease"),
                        _ => if expect_tf.starts_with("cubic-bezier(") {
                            let mut params = expect_tf["cubic-bezier(".len() .. expect_tf.rfind(')').unwrap_or(expect_tf.len()-1)].split(',');
                            let mut cb = "druid_xml::simple_style::TimingFunction::CubicBezier{".to_string();
                            cb.push_str("p1:"); params.next().unwrap_or("0"); cb.push_str("f64, ");
                            cb.push_str("p2:"); params.next().unwrap_or("0"); cb.push_str("f64, ");
                            cb.push_str("p3:"); params.next().unwrap_or("0"); cb.push_str("f64, ");
                            cb.push_str("p4:"); params.next().unwrap_or("0"); cb.push_str("f64, ");
                            cb.push_str("}");
                            Cow::Owned(cb)
                        } else if expect_tf.starts_with("steps(") {
                            let mut params = expect_tf["steps(".len() .. expect_tf.rfind(')').unwrap_or(expect_tf.len()-1)].split(',');
                            let mut cb = "druid_xml::simple_style::TimingFunction::Steps{".to_string();
                            cb.push_str("n:"); params.next().unwrap_or("0"); cb.push_str("f64, ");
                            let jumpterm = match params.next().unwrap_or("jump-start") {
                                "jump-start" => "druid_xml::simple_style::JumpTerm::JumpStart",
                                "jump-end" => "druid_xml::simple_style::JumpTerm::JumpEnd",
                                "jump-none" => "druid_xml::simple_style::JumpTerm::JumpNone",
                                "jump-both" => "druid_xml::simple_style::JumpTerm::JumpBoth",
                                "start" => "druid_xml::simple_style::JumpTerm::Start",
                                "end" => "druid_xml::simple_style::JumpTerm::End",
                                _ => "druid_xml::simple_style::JumpTerm::JumpStart"
                            };
                            cb.push_str("jumpterm:"); params.next().unwrap_or("0"); cb.push_str("f64, ");
                            cb.push_str("}");
                            Cow::Owned(cb)
                        } else {
                            Cow::Borrowed("druid_xml::simple_style::TimingFunction::Linear")
                        }
                    };
                    return format!("Some(druid_xml::simple_style::AnimationState::from( druid_xml::simple_style::Animation{{ delay: {delay}, direction: druid_xml::simple_style::Direction::Normal, duration: {duration}, iteration: 1., name: 1., timing_function: {timing_function}, fill_mode: 0. }} ))");
                }
                return "None".to_string()
            }

            let normal_transition = get_style!("transition");
            src!("let mut normal_style = \n");
            src!("druid_xml::simple_style::Styler {{\n");
            src!("     padding : ("); style_opt!("druid::Insets::from(", "padding", ")"); _src!(0, ", {}),\n", normal_transition.map(|e| transition_option(e,"padding")).unwrap_or("None".to_string()) );
            src!("     margin : ("); style_opt!("druid::Insets::from(", "margin", ")"); _src!(0, ", {}),\n", normal_transition.map(|e| transition_option(e,"margin")).unwrap_or("None".to_string()) );
            src!("     font_size : ("); style_opt!("( ", "font-size", ")"); _src!(0, ", {}),\n", normal_transition.map(|e| transition_option(e,"font-size")).unwrap_or("None".to_string()) );
            src!("     width : ("); style_opt!("width"); _src!(0, ", {}),\n", normal_transition.map(|e| transition_option(e,"width")).unwrap_or("None".to_string()) );
            src!("     height : ("); style_opt!("height"); _src!(0, ", {}),\n", normal_transition.map(|e| transition_option(e,"height")).unwrap_or("None".to_string()) );
            src!("     text_color : ("); style_opt!("color"); _src!(0, ", {}),\n", normal_transition.map(|e| transition_option(e,"color")).unwrap_or("None".to_string()) );
            src!("     background_color : ("); style_opt!("background-color");  _src!(0, ", {}),\n", normal_transition.map(|e| transition_option(e,"background-color")).unwrap_or("None".to_string()) );
            src!("     border : ("); style_opt!("druid_xml::simple_style::BorderStyle::new(", "border", ")"); _src!(0, ", {}),\n", normal_transition.map(|e| transition_option(e,"border")).unwrap_or("None".to_string()) );
            src!("}};\n");

            src!("let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [\n");
            let mut pseudo_count = 0;
            for rule in css.rules.iter() {
                let pseudo_trap_hack = PseudoOrderTrapQueryWrap::new( ElementQueryWrap { parent_stack, elem } );
                
                if rule.selector.matches(&pseudo_trap_hack) {
                    let pseudo = pseudo_trap_hack.get_pseudo();

                    /// check disabled. simplecss 'disabled' pseudo not support
                    let selector = rule.selector.to_string();
                    let mut check_disabled = simplecss::SelectorTokenizer::from( selector.as_str() );
                    let is_disabled = check_disabled.find( |e| {
                        if let Ok(e) = e {
                            match e {
                                simplecss::SelectorToken::PseudoClass(p) => *p == "disabled",
                                _ => false
                            }
                        } else {
                            false
                        }
                    }).is_some();
                    
                    let pseudo = if is_disabled {
                        Some(Pseudo::Disabled)
                    } else {
                        pseudo
                    };

                    macro_rules! pseudo_style_opt {
                        ( $name:literal ) => {
                            pseudo_style_opt!("", $name, "");
                        };
                        ( $start:literal, $name:literal, $end:literal ) => {
                            if let Some(value) = rule.declarations.iter().find( |e| e.name == $name ).map( |e| e.value ) {
                                _src!( 0, "Some(");
                                _src!( 0, $start);
                                attr_write!($name, value);
                                _src!( 0, $end );
                                _src!( 0, ")" );
                            } else {
                                _src!( 0, "None" );
                            }
                        }
                    }

                    if let Some(pseudo) = pseudo {
                        pseudo_count += 1;
                        let pseudo_transition = rule.declarations.iter().find( |e| e.name == "transition" ).map( |e| e.value );
                        match pseudo {
                            Pseudo::Focus => { src!("Some(druid_xml::simple_style::PseudoStyle::focus( druid_xml::simple_style::Styler {{\n"); },
                            Pseudo::Hover => { src!("Some(druid_xml::simple_style::PseudoStyle::hover( druid_xml::simple_style::Styler {{\n"); },
                            Pseudo::Active => { src!("Some(druid_xml::simple_style::PseudoStyle::active( druid_xml::simple_style::Styler {{\n"); },
                            Pseudo::Disabled => { src!("Some(druid_xml::simple_style::PseudoStyle::disabled( druid_xml::simple_style::Styler {{\n"); },
                        }
                        
                        src!("     padding : ("); pseudo_style_opt!("druid::Insets::from(", "padding", ")"); _src!(0, ", {}),\n", pseudo_transition.map(|e| transition_option(e,"padding")).unwrap_or("None".to_string()) );
                        src!("     margin : ("); pseudo_style_opt!("druid::Insets::from(", "margin", ")"); _src!(0, ", {}),\n", pseudo_transition.map(|e| transition_option(e,"margin")).unwrap_or("None".to_string()) );
                        src!("     font_size : ("); pseudo_style_opt!("( ", "font-size", ")"); _src!(0, ", {}),\n", pseudo_transition.map(|e| transition_option(e,"font-size")).unwrap_or("None".to_string()) );
                        src!("     width : ("); pseudo_style_opt!("width"); _src!(0, ", {}),\n", pseudo_transition.map(|e| transition_option(e,"width")).unwrap_or("None".to_string()) );
                        src!("     height : ("); pseudo_style_opt!("height"); _src!(0, ", {}),\n", pseudo_transition.map(|e| transition_option(e,"height")).unwrap_or("None".to_string()) );
                        src!("     text_color : ("); pseudo_style_opt!("color"); _src!(0, ", {}),\n", pseudo_transition.map(|e| transition_option(e,"color")).unwrap_or("None".to_string()) );
                        src!("     background_color : ("); pseudo_style_opt!("background-color");  _src!(0, ", {}),\n", pseudo_transition.map(|e| transition_option(e,"background-color")).unwrap_or("None".to_string()) );
                        src!("     border : ("); pseudo_style_opt!("druid_xml::simple_style::BorderStyle::new(", "border", ")"); _src!(0, ", {}),\n", pseudo_transition.map(|e| transition_option(e,"border")).unwrap_or("None".to_string()) );
                        
                        src!("}}) ), ");
                    }
                }
            }

            //fill 'None' 
            for i in pseudo_count .. 4 {
                src!("None,\n");
            }
            src!("];\n");
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

            //wrap `SizedBox` with optimize
            if attrs.get(b"width").is_some() && attrs.get(b"height").is_some() {
                style!("let {tag_wrap} = druid::WidgetExt::fix_size({tag_wrap}, " , "width-height", ");\n" );
            } else {
                style!("let {tag_wrap} = druid::WidgetExt::fix_width({tag_wrap}, " , "width", ");\n" );
                style!("let {tag_wrap} = druid::WidgetExt::fix_height({tag_wrap}, " , "height", ");\n" );    
            }

            //wrap `Padding` for Label,Button
            //style!("let {tag_wrap} = druid::WidgetExt::padding({tag_wrap}, " , "padding", ");\n" );
            //src!("let {tag_wrap} = druid::WidgetExt::padding( {tag_wrap}, druid_xml::widget::theme::PADDING );\n");
            

            //custom query wrapper
            for (query, wrapper) in wrappers.iter() {
                if let Some(selector) = simplecss::Selector::parse(query) {
                    if selector.matches( &elem_query ) {
                        //src!("let {tag_wrap} = ({wrapper})({tag_wrap});\n");
                        src!("let widget = {tag_wrap};\n");
                        src!("let {tag_wrap} = {wrapper};\n");
                    }
                }
            }

            //finally wrapping styler widget
            //we must have wrapped above 'Padding' widget
            //src!("let {tag_wrap} = druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, {tag_wrap} );\n");
        }

        src!("{tag_wrap}\n" ); //return element

        Ok(())
    }
}


impl SourceGenerator for DruidGenerator {
    fn write_raw(&mut self, code:&str) -> Result<(),Error> {
        self.writer.push_str(code);
        Ok(())
    }

    fn write(&mut self, elem_map:&HashMap<String,Element>, elem:&Element, css:&StyleSheet, wrappers:&HashMap<String,String>) -> Result<(),Error> {
        self.impl_write(None,elem_map, &mut vec![], elem, css, wrappers)
    }
}

struct CSSAttribute;

impl CSSAttribute {
    fn padding(w:&mut String, v:&str) -> Result<(),Error> {
        let mut splits = v.split_whitespace().map( |s| &s[..s.find("px").unwrap_or(s.len())] );
        let count = splits.clone().count();
        if count == 1 {
            Self::size(w, splits.next().unwrap())?
        } else if count == 2 || count == 4 {
            write!(w,"(").unwrap();
            splits.for_each(|e| write!(w,"{}f64,",e).unwrap() );
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
            write!(w,"druid::Color::from_hex_str({})", tv).unwrap();
        } else if tv.starts_with("rgba") && tv.ends_with(')') {
            write!(w,"druid::Color::rgba8({})", &tv[tv.find('(').unwrap()+1 .. tv.rfind(')').unwrap()]).unwrap();
        } else if tv.starts_with("rgb") && tv.ends_with(')') {
            write!(w,"druid::Color::rgb8({})", &tv[tv.find('(').unwrap()+1 .. tv.rfind(')').unwrap()]).unwrap();
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
            "justify" => write!(w, "druid::TextAlignment::Justified").unwrap(),
            _ => return Err( Error::InvalidAttributeValue((0,"text-align")) )
        }
        Ok(())
    }

    fn border_color_and_width(w:&mut String, radius:Option<&str>, v:&str) -> Result<(), Error> {
        let mut splited = v.split_whitespace();
        let width = splited.next().map( |v| v[..v.find("px").unwrap_or(v.len())].parse::<f64>().unwrap() ).unwrap_or(1f64);
        //TODO : support other border style?
        let _border_style = splited.next().unwrap_or("solid");
        if _border_style != "solid" {
            Err(Error::InvalidAttributeValue((0,"border")))
        } else {
            let color = splited.next().unwrap_or("black");
            write!(w,"{}f64, ", width).unwrap();
            Self::size(w, radius.unwrap_or("0")).unwrap();
            write!(w,",").unwrap();
            Self::color(w,color).unwrap();
            Ok(())
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