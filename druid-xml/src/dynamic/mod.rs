use std::borrow::Cow;
use std::collections::HashMap;

use druid::{Widget,WidgetExt,TextAlignment,Color};
use druid::widget::*;
use quick_xml::{Reader, events::Event};
use simplecss::{StyleSheet, Declaration, DeclarationTokenizer};

use crate::writer::ElementQueryWrap;
use crate::{Element, Error, AttributeGetter, DummyLens};

mod color;


pub fn generate_widget(xml:&str) -> Result< HashMap<String,Box<dyn Widget<()>>>, Error > {
	let mut style = StyleSheet::new();
	let mut reader = Reader::from_str(xml);
    let mut map = HashMap::new();
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
                            let widget = build_widget(&[], &elem, &style)?;
                            map.insert(elem.attributes().get_as_result::<String>("fn", elem.src_pos)?, widget );
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
	Ok( map )
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
                $widget.set_text_color( color::to_color(value, Some(Color::rgb8(0,0,0)) ) );
            }
        };
        ( $widget:ident, "font-size" ) => {
            if let Some(value) = get_style!("font-size") {
                $widget.set_text_size( value.parse::<f64>().unwrap() )
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
                    "left" => $widget.set_text_alignment(TextAlignment::Start),
                    "right" => $widget.set_text_alignment(TextAlignment::End),
                    "center" => $widget.set_text_alignment(TextAlignment::Center),
                    "justify" => $widget.set_text_alignment(TextAlignment::Justified),
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
    let mut child:Box<dyn Widget<()>> = 
    if tag == "flex" {
        let mut flex = if let Some( Cow::Borrowed(b"column") ) = attrs.get(b"direction") {
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
                b"start" => druid::widget::MainAxisAlignment::Start,
                b"center" => druid::widget::MainAxisAlignment::Center,
                b"end" => druid::widget::MainAxisAlignment::End,
                b"spacebetween" => druid::widget::MainAxisAlignment::SpaceBetween,
                b"spaceevenly" => druid::widget::MainAxisAlignment::SpaceEvenly,
                b"spacearound" => druid::widget::MainAxisAlignment::SpaceAround,
                _ => return Err(Error::InvalidAttributeValue((elem.src_pos, "axis_alignment")))
            };
            flex.set_main_axis_alignment(v);
        }
        

        let new_stack = new_parent_stack!();
        for child in elem.childs.iter() {
            if child.tag().as_ref() == b"spacer" {
                if let Some(v) = child.attributes().get(b"flex") {
                    flex.add_flex_spacer( String::from_utf8_lossy(&v).parse::<f64>().unwrap() );
                } else {
                    flex.add_default_spacer( );
                }
            } else {
                
                let child_widget = build_widget(&new_stack, child, css)?;
                if let Some(flex_param) = child.attributes().get_as::<f64>(b"flex") {
                    flex.add_flex_child(child_widget, flex_param );
                } else {
                    flex.add_child( child_widget );
                }
            }
        }
        flex.boxed()
    }

    //WARN : checkbox is none-standard
    else if tag == "label" || tag == "button" {
        let name = elem.text.as_ref().map( |e| String::from_utf8_lossy(&e) ).unwrap_or( std::borrow::Cow::Borrowed("Label") );
        let mut label = druid::widget::Label::new( name.as_ref() );
        style!(label, "color");
        style!(label, "font-size");
        style!(label, "text-align");

        if tag == "button" {
            Button::from_label(label).boxed()
        } else {
            label.boxed()
        }
    }

    else if tag == "checkbox" || (tag == "input" && input_type == "checkbox") {
        let name = elem.text.as_ref().map( |e| String::from_utf8_lossy(&e) ).unwrap_or( std::borrow::Cow::Borrowed("Label") );
        tag_wrap = "checkbox";
        Checkbox::new(name.as_ref() ).lens( DummyLens::<(),_>::new( false) ).boxed()
    }

    //TODO : password type?
    else if tag == "textbox" || (tag == "input" && input_type == "text") {
        tag_wrap = "textbox";
        let mut textbox = druid::widget::TextBox::new();
        style!(textbox, "color");
        style!(textbox, "font-size");
        style!(textbox, "text-align");
        style!(textbox, "placeholder");
        textbox.lens( DummyLens::<(),String>::new(String::new()) ).boxed()
    }

    //WARN : "image" is none-standard
    else if tag == "image" || tag == "img" {
        tag_wrap = "image";
        // let file_src_holder = &attrs.get_result("src", 0)?;
        // let file_src = String::from_utf8_lossy( file_src_holder );
        // let image_buf = druid::ImageBuf::from_bytes( inclue_bytes!(\"{}\") ).unwrap();\n", &file_src);
        // let mut image = druid::widget::Image::new(image_buf);
        // style!(image, "object-fit");
        // style!(image, "image-rendering");
        todo!()
    }

    //WARN : list is none-standard
    else if tag == "list" {
        todo!()
        // style!( "let mut list = druid::widget::List::new(", "fn" ,");\n");
        // if let Some( Cow::Borrowed(b"horizontal") ) = attrs.get(b"direction") {
        //     src!( "list = list.horizontal();\n");
        // }
        // style!( "list.set_spacing(", "spacing", ");\n");
    }

    else if tag == "scroll" {
        if elem.childs.len() != 1 {
            return Err(Error::InvalidScrollChildNum(elem.src_pos))
        }
        let new_stack = new_parent_stack!();
        let child = build_widget(&new_stack, &elem.childs[0], css)?;
        Scroll::new(child).boxed()
    }

    else if tag == "slider" {
        let min = attrs.get_as_result::<f64>("min", elem.src_pos).unwrap_or(0f64);
        let max = attrs.get_as_result::<f64>("max", elem.src_pos).unwrap_or(1f64);
        let mut slider = Slider::new();
        slider.with_range(min,max).lens( DummyLens::<(),f64>::new(0f64) ).boxed()
    }

    else if tag == "spinner" {
        let mut spinner = Spinner::new();
        if let Some(v) = attrs.get(b"color") {
            spinner.set_color( color::to_color(&String::from_utf8_lossy(&v), None) );
        }
        spinner.boxed()
    }

    //TODO : child must be two item
    else if tag == "split" {
        if elem.childs.len() != 2 {
            return Err(Error::InvalidSplitChildNum(elem.src_pos))
        }
        let new_stack = new_parent_stack!();
        let one = build_widget(&new_stack, &elem.childs[0], css)?;
        let two = build_widget(&new_stack, &elem.childs[0], css)?;

        let mut split = if let Some( Cow::Borrowed(b"column") ) = attrs.get(b"direction") {
            Split::columns(one, two)
        } else {
            Split::rows(one, two)
        };
        
        if let Some(v) = attrs.get(b"split_point") {
            split = split.split_point( String::from_utf8_lossy(&v).parse::<f64>().unwrap() );
        }
        if let Some(v) = attrs.get(b"min_size") {
            let v = String::from_utf8_lossy(&v);
            let mut splits = v.split(',');
            split = split.min_size( splits.next().unwrap().parse::<f64>().unwrap(),splits.next().unwrap().parse::<f64>().unwrap() );
        }
        if let Some(v) = attrs.get(b"bar_size") {
            split = split.bar_size( String::from_utf8_lossy(&v).parse::<f64>().unwrap() );
        }
        if let Some(v) = attrs.get(b"min_bar_area") {
            split = split.min_bar_area( String::from_utf8_lossy(&v).parse::<f64>().unwrap() );
        }
        if let Some(v) = attrs.get(b"draggable") {
            split = split.draggable( String::from_utf8_lossy(&v).parse::<bool>().unwrap() );
        }
        if let Some(v) = attrs.get(b"solid_bar") {
            split = split.solid_bar( String::from_utf8_lossy(&v).parse::<bool>().unwrap() );
        }

        split.boxed()
    }

    else if tag == "stepper" {
        let min = attrs.get_as_result::<f64>("min", elem.src_pos).unwrap_or(std::f64::MIN);
        let max = attrs.get_as_result::<f64>("max", elem.src_pos).unwrap_or(std::f64::MAX);
        let step = attrs.get_as_result::<f64>("step", elem.src_pos).unwrap_or(std::f64::MAX);
        let wrap = attrs.get_as_result::<bool>("wraparound", elem.src_pos).unwrap_or(false);
        let mut stepper = Stepper::new();
        stepper = stepper.with_range(min,max);
        stepper = stepper.with_step(step);
        stepper = stepper.with_wraparound(wrap);
        stepper.lens(DummyLens::<(),f64>::new(0f64)).boxed()
    }

    else if tag == "switch" {
        //TODO : style
        Switch::new().lens(DummyLens::<(),bool>::new(false)).boxed()
    }

    //TODO
    else if tag == "painter" || tag == "canvas" {
        tag_wrap = "painter";
        todo!()
    }

    //WARN : container is none-standard
    else if tag == "container" {
        if elem.childs.len() != 1 {
            return Err(Error::InvalidContainerChildNum(elem.src_pos))
        }
        let new_stack = new_parent_stack!();
        let child = build_widget(&new_stack, &elem.childs[0], css)?;
        Container::new( child ).boxed()
    }
    else {
        unimplemented!("Unknown tag : {}",tag)
    };


    //all component
    //background, padding, 
    {
        //wrap Lens
        //None

        //wrap `Padding`
        if let Some(padding) = attrs.get(b"padding") {
            let padding = String::from_utf8_lossy(&padding);
            let mut splits = padding.split_whitespace();
            let count = splits.clone().count();
            if count == 1 {
                child = child.padding( splits.next().unwrap().parse::<f64>().unwrap() ).boxed();
            } else if count == 2 {
                child = child.padding( (splits.next().unwrap().parse::<f64>().unwrap(), splits.next().unwrap().parse::<f64>().unwrap()) ).boxed();
            } else if count == 4 {
                child = child.padding( (splits.next().unwrap().parse::<f64>().unwrap(), splits.next().unwrap().parse::<f64>().unwrap(), splits.next().unwrap().parse::<f64>().unwrap(), splits.next().unwrap().parse::<f64>().unwrap()) ).boxed();
            } else {
                panic!("The number of padding parameters must be one of 1,2,4. but \"{}\"",count);
            }
        }

        //wrap `SizedBox` with optimize
        if attrs.get(b"width").is_some() && attrs.get(b"height").is_some() {
            child = child.fix_size(attrs.get_as_result::<f64>("width", elem.src_pos)?, attrs.get_as_result::<f64>("height", elem.src_pos)? ).boxed();
        } else {
            if let Ok(width) = attrs.get_as_result::<f64>("width", elem.src_pos) {
                child = child.fix_width(width).boxed();
            }
            if let Ok(height) = attrs.get_as_result::<f64>("height", elem.src_pos) {
                child = child.fix_height(height).boxed();
            }
        }
        
        //wrap 'Container' 
        if tag != "container" {
            if let Some(background) = attrs.get(b"background-color") {
                let col = color::to_color( &String::from_utf8_lossy(&background), Some(Color::rgb8(255,255,255)) );
                child = child.background( col ).boxed();
            }
            if let Some(border) = attrs.get(b"border") {
                let v = String::from_utf8_lossy(&border);
                let mut splited = v.split_whitespace();
                let width = splited.next().map( |v| v[..v.find("px").unwrap_or(v.len())].parse::<f64>().unwrap() ).unwrap_or(1f64);
                //TODO : support other border style?
                let _border_style = splited.next().unwrap_or("solid");
                if _border_style != "solid" {
                    return Err(Error::InvalidAttributeValue((0,"border")))
                } else {
                    let color = color::to_color(splited.next().unwrap_or("black"), None);
                    child = child.border(color, width).boxed()
                }
            }
        }
    }

    Ok( child )
}