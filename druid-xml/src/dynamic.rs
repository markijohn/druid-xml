use std::borrow::Cow;

use druid::{Widget,WidgetExt};
use druid::widget::*;
use quick_xml::{Reader, events::Event};
use simplecss::{StyleSheet, Declaration, DeclarationTokenizer};

use crate::writer::ElementQueryWrap;
use crate::{Element, Error};

mod color;


pub fn generate_widget(xml:&str) -> impl Box<dyn Widget<()>> {
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
						if let Some(elem) = crate::parse_element(pos, Some(Event::Start(e)), &mut reader )? {
                            //TODO
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

fn build_widget(parent_stack:&[&Element], elem:&Element, css:&StyleSheet) -> Result<Box<dyn Widget<()>>,Error> {
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

    macro_rules! style {
        ( $widget:ident, "width-height" ) => {
            if let (Some(width), Some(height)) = (get_style!("width") , get_style!("height")) {
                $widget.fix_size( width.parse<f64>().unwrap(), height.parse<f64>().unwrap() )
            } else {
                $widget
            }
        };
        ( $widget:ident, "width" ) => {
            if let Some(v) = get_style!("width") {
                $widget.fix_width( v.parse<f64>().unwrap() )
            } else {
                $widget
            }
        };
        ( $widget:ident, "height" ) => {
            if let Some(v) = get_style!("height") {
                $widget.fix_height( v.parse<f64>().unwrap() )
            } else {
                $widget
            }
        };
        ( $widget:ident, "padding" ) => {
            if let Some(v) = get_style!("padding") {
                let mut splits = v.split_whitespace();
                let count = splits.clone().count();
                if count == 1 {
                    $widget.padding( splits.next().unwrap().parse::<f64>().unwrap() )
                } else if count == 2 || count == 4 {
                    $widget.padding( (splits.next().unwrap().parse::<f64>().unwrap(),splits.next().unwrap().parse::<f64>().unwrap()) )
                } else if count ==4 {
                    $widget.padding( (splits.next().unwrap().parse::<f64>().unwrap(),splits.next().unwrap().parse::<f64>().unwrap(),splits.next().unwrap().parse::<f64>().unwrap(),splits.next().unwrap().parse::<f64>().unwrap()) )
                } else {
                    panic!("The number of padding parameters must be one of 1,2,4. but \"{}\"",v);
                }
                Ok(())
            }
        };
        ( $widget:ident, "color" ) => {
            if let Some(value) = get_style!("color") {
                $widget.set_text_color( color::to_color(value) )
            }
        };
        ( $widget:ident, "font-size" ) => {
            if let Some(value) = get_style!("font-size") {
                $widget.set_font_size( value.parse::<f64>().unwrap() )
            }
        };
        ( $widget:ident, "border" ) => {
            if let Some(v) = get_style!("border") {
                let mut splited = v.split_whitespace();
                let width = splited.next().map( |v| v[..v.find("px").unwrap_or(v.len())].parse::<f64>().unwrap() ).unwrap_or(1f64);
                //TODO : support other border style?
                let _border_style = splited.next().unwrap_or("solid");
                if _border_style != "solid" {
                    Err(Error::InvalidAttributeValue((0,"border")))
                } else {
                    let color = splited.next().unwrap_or("black");
                    $widget.border( width, color::to_color(color) )
                }
            } else {
                $widget
            }
        };
        ( $widget:ident, "text-align" ) => {
            if let Some(v) = get_style!("text-align") {
                match v {
                    "left" => $widget.set_text_align(druid::TextAlignment::Start),
                    "right" => $widget.set_text_align(druid::TextAlignment::End),
                    "center" => $widget.set_text_align(druid::TextAlignment::Center),
                    "justify" => $widget.set_text_align(druid::TextAlignment::Justify),
                    _ => return Err( Error::InvalidAttributeValue((0,"text-align")) )
                }
            }
        };

        ( $widget:ident, "placeholder" ) => {
            if let Some(v) = get_style!("placeholder") {
                $widget.set_placeholder( v.to_owned() );
            }
        };

        ( $widget:ident, "object-fit" ) => {
            if let Some(v) = get_style!("text-align") {
                match v {
                    "left" => $widget.set_text_align(druid::TextAlignment::Start),
                    "right" => $widget.set_text_align(druid::TextAlignment::End),
                    "center" => $widget.set_text_align(druid::TextAlignment::Center),
                    "justify" => $widget.set_text_align(druid::TextAlignment::Justify),
                    _ => return Err( Error::InvalidAttributeValue((0,"text-align")) )
                }
            }
        };

        ( $widget:ident, "text-align" ) => {
            if let Some(v) = get_style!("text-align") {
                match v {
                    "left" => $widget.set_text_align(druid::TextAlignment::Start),
                    "right" => $widget.set_text_align(druid::TextAlignment::End),
                    "center" => $widget.set_text_align(druid::TextAlignment::Center),
                    "justify" => $widget.set_text_align(druid::TextAlignment::Justify),
                    _ => return Err( Error::InvalidAttributeValue((0,"text-align")) )
                }
            }
        };

        ( $widget:ident, "image-rendering" ) => {
            if let Some(v) = get_style!("text-align") {
                match v {
                    "left" => $widget.set_text_align(druid::TextAlignment::Start),
                    "right" => $widget.set_text_align(druid::TextAlignment::End),
                    "center" => $widget.set_text_align(druid::TextAlignment::Center),
                    "justify" => $widget.set_text_align(druid::TextAlignment::Justify),
                    _ => return Err( Error::InvalidAttributeValue((0,"text-align")) )
                }
            }
        };
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
        let flex = if let Some( Cow::Borrowed(b"column") ) = attrs.get(b"direction") {
            druid::widget::Flex::column()
        } else {
            druid::widget::Flex::row()
        };
        if elem.childs.len() < 1 {
            return Err(Error::InvalidFlexChildNum((elem.src_pos)))
        }

        if let Some(v) = attrs.get(b"must_fill_main_axis") {
            if "true" == String::from_utf8_lossy(&v) {
               flex = flex.must_fill_main_axis(true); 
            }
        }

        if let Some(v) = attrs.get(b"cross_axis_alignment") {
            let v = match v.as_ref() {
                b"start" => druid::widget::CrossAxisAlignment::Start,
                b"center" => druid::widget::CrossAxisAlignment::Center,
                b"end" => druid::widget::CrossAxisAlignment::End,
                b"baseline" => druid::widget::CrossAxisAlignment::Baseline,
                _ => return Err(Error::InvalidAttributeValue((elem.src_pos, "cross_axis_alignment")))
            };
            flex.set_cross_axis_alignment(v);
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