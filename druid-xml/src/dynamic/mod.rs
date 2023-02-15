use std::borrow::Cow;
use std::collections::HashMap;

use druid::kurbo::Line;
use druid::{Widget,WidgetExt,TextAlignment,Color};
use druid::widget::*;
use quick_xml::{Reader, events::Event};
use simplecss::{StyleSheet, Declaration, DeclarationTokenizer};

use crate::simple_style::{AnimationState, Pseudo, BorderStyle};
use crate::writer::{ElementQueryWrap, PseudoOrderTrapQueryWrap};
use crate::{Element, Error, AttributeGetter, DummyLens, AttributesWrapper};

mod color;
pub(crate) mod ex_custom_widget;

struct LazyWrapperWidget<W:Widget<D>,D> {
    child : Option<W>,
    d : std::marker::PhantomData<D>
}

impl <D, W:Widget<D>> Widget<D> for LazyWrapperWidget<W,D> {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut D, env: &druid::Env) {
        if let Some(child) = self.child.as_mut() {
            child.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &D, env: &druid::Env) {
        if let Some(child) = self.child.as_mut() {
            child.lifecycle(ctx, event, data, env)
        }
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _old_data: &D, data: &D, env: &druid::Env) {
        if let Some(child) = self.child.as_mut() {
            child.update(ctx, _old_data, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &D,
        env: &druid::Env,
    ) -> druid::Size {
        if let Some(child) = self.child.as_mut() {
            child.layout(ctx, bc, data, env)
        } else {
            bc.constrain((100.0, 100.0))
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &D, env: &druid::Env) {
        if let Some(child) = self.child.as_mut() {
            child.paint(ctx, data, env);
        }
    }
}

pub fn generate_widget(xml:&str) -> Result< Box<dyn Widget<()>>, Error > {
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
						if let Some(elem) = crate::parse_element(pos, Some(Event::Start(e)), &mut reader )? {
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

    let widget = if let Some(main) = expected_main_widget.and( last_widget ) {
        if let Some(elem ) = elem_map.get(&main) {
            build_widget(None, &elem_map, &[], &elem, &style)?
        } else {
            Label::new(format!("Can't find main widget : {}", main) ).boxed()
        }
    } else {
        Label::new("There has no root element").boxed()
    };
    

	Ok( widget )
}

fn build_widget<'a>(parameter:Option<&AttributesWrapper<'a>>,parsed_map:&HashMap<String,Element>, parent_stack:&[&Element], elem:&Element, css:&StyleSheet) -> Result<Box<dyn Widget<()>>,Error> {
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

    let attrs = elem.attributes( parameter );
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
            if let (Some(mut width), Some(mut height)) = (get_style!("width") , get_style!("height")) {
                width = &width[ .. width.find("px").unwrap_or(width.len())];
                height = &height[ .. height.find("px").unwrap_or(height.len())];
                if let (Ok(width),Ok(height)) = ( width.parse::<f64>(), height.parse::<f64>() ) {
                    $widget.fix_size(width, height).boxed()
                } else {
                    $widget
                }
            } else {
                $widget
            }
        };
        ( $widget:ident, "width" ) => {
            if let Some(mut v) = get_style!("width") {
                if v.ends_with("px") {
                    v = &v[ .. v.len()-2];
                }
                if let Ok(v) = v.parse::<f64>() {
                    $widget.fix_width(v).boxed()
                } else {
                    $widget
                }
            } else {
                $widget
            }
        };
        ( $widget:ident, "height" ) => {
            if let Some(mut v) = get_style!("height") {
                if v.ends_with("px") {
                    v = &v[ .. v.len()-2];
                }
                if let Ok(v) = v.parse::<f64>() {
                    $widget.fix_height(v).boxed()
                } else {
                    $widget
                }
            } else {
                $widget
            }
        };
        ( $widget:ident, "background-color" ) => {
            if let Some(background) = get_style!("background-color") {
                let col = color::to_color( background, Some(Color::rgb8(255,255,255)) );
                $widget.background( col ).boxed()
            } else {
                $widget
            }
        };
        ( $widget:ident, "padding" ) => {
            if let Some(v) = get_style!("padding") {
                let mut splits = v.split_whitespace().map( |s| &s[..s.find("px").unwrap_or(s.len())] );
                let count = splits.clone().count();
                if count == 1 {
                    $widget.padding( splits.next().unwrap_or("0").parse::<f64>().unwrap_or(0f64) ).boxed()
                } else if count == 2 || count == 4 {
                    $widget.padding( (splits.next().unwrap_or("0").parse::<f64>().unwrap_or(0f64), splits.next().unwrap_or("0").parse::<f64>().unwrap_or(0f64)) ).boxed()
                } else if count ==4 {
                    $widget.padding( (splits.next().unwrap_or("0").parse::<f64>().unwrap_or(0f64), splits.next().unwrap_or("0").parse::<f64>().unwrap_or(0f64), splits.next().unwrap_or("0").parse::<f64>().unwrap_or(0f64), splits.next().unwrap_or("0").parse::<f64>().unwrap_or(0f64)) ).boxed()
                } else {
                    return Err(Error::InvalidAttributeValue( (attrs.pos(),"padding") ) )
                }
            } else {
                $widget
            }
        };
        ( $widget:ident, "color" ) => {
            if let Some(value) = get_style!("color") {
                $widget.with_text_color( color::to_color(value, Some(Color::rgb8(0,0,0)) ) )
            } else {
                $widget
            }
        };
        ( $widget:ident, "font-size" ) => {
            if let Some(value) = get_style!("font-size") {
                let tv = value.trim();
                let font_size = match tv.as_bytes() {
                    b"xx-small" => 9f64,
                    b"x-small" => 10f64,
                    b"small" => 13.333f64,
                    b"medium" => 16f64,
                    b"large" => 18f64,
                    b"x-large" => 24f64,
                    b"xx-large" => 32f64,
                    [val @ .. , b'p', b'x'] => String::from_utf8_lossy(val).parse::<f64>().unwrap_or(13.333f64),
                    [val @ .. , b'e', b'm'] => String::from_utf8_lossy(val).parse::<f64>().map( |v| v / 0.0625).unwrap_or(13.333f64),
                    [val @ .. , b'p', b't'] => String::from_utf8_lossy(val).parse::<f64>().map( |v| v * 1.333).unwrap_or(13.333f64),
                    [val @ .. , b'%'] => String::from_utf8_lossy(val).parse::<f64>().map( |v| v / 100f64 / 0.0625 ).unwrap_or(13.333f64),
                    val @ _ => String::from_utf8_lossy(val).parse::<f64>().unwrap_or(13.333f64)
                };
                $widget.with_text_size( font_size )
            } else {
                $widget
            }
        };
        ( $widget:ident, "border" ) => {
            if let Some(border) = get_style!("border") {
                let v = border.trim();
                let mut splited = v.split_whitespace();
                let width = splited.next().map( |v| v[..v.find("px").unwrap_or(v.len())].parse::<f64>().unwrap_or(1f64) ).unwrap_or(1f64);
                //TODO : support other border style?
                let _border_style = splited.next().unwrap_or("solid");
                if _border_style != "solid" {
                    return Err(Error::InvalidBorderAttributeValue(attrs.pos()))
                } else {
                    let color = color::to_color(splited.next().unwrap_or("black"), None);
                    $widget.border(color, width).boxed()
                }
            } else {
                $widget
            }
        };
        ( $widget:ident, "text-align" ) => {
            if let Some(v) = get_style!("text-align") {
                match v {
                    "left" => $widget.with_text_alignment(TextAlignment::Start),
                    "right" => $widget.with_text_alignment(TextAlignment::End),
                    "center" => $widget.with_text_alignment(TextAlignment::Center),
                    "justify" => $widget.with_text_alignment(TextAlignment::Justified),
                    _ => return Err( Error::InvalidAttributeValue((0,"text-align")) )
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
            return Err(Error::InvalidFlexChildNum(elem.src_pos))
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
                if let Some(v) = child.attributes(None).get(b"flex") {
                    flex.add_flex_spacer( String::from_utf8_lossy(&v).parse::<f64>().unwrap_or(1f64) );
                } else {
                    flex.add_default_spacer( );
                }
            } else {
                let child_widget = build_widget(parameter, parsed_map, &new_stack, child, css)?;
                if let Some(flex_param) = child.attributes(None).get_as::<f64>(b"flex") {
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
        let label_text = if text == "" {
            tag.as_ref()
        } else {
            &text
        };
        //let mut label = druid::widget::Label::new( label_text );
        // label = style!(label, "color");
        // label = style!(label, "font-size");
        // label = style!(label, "text-align");

        let mut label = crate::widget::DXLabel::new( label_text );

        if let Some(lbk) = attrs.get(b"line-break") {
            match lbk.as_ref() {
                b"wordwrap" => label.set_line_break_mode( LineBreaking::WordWrap ),
                b"clip" => label.set_line_break_mode( LineBreaking::Clip ),
                b"overflow" => label.set_line_break_mode( LineBreaking::Overflow ),
                _ => ()
            }
        }

        if tag == "button" {
            crate::widget::DXButton::from_label(label).boxed()
        } else {
            label.boxed()
        }
    }

    else if tag == "checkbox" || (tag == "input" && input_type == "checkbox") {
        tag_wrap = "checkbox";
        let label_text = if text == "" {
            "Checkbox"
        } else {
            &text
        };
        Checkbox::new(label_text ).lens( DummyLens::<(),_>::new( false) ).boxed()
    }

    //TODO : password type?
    else if tag == "textbox" || (tag == "input" && input_type == "text") {
        tag_wrap = "textbox";
        let mut textbox = if let Some(Cow::Borrowed(b"true")) = attrs.get(b"multiline") {
            druid::widget::TextBox::multiline()
        } else {
            druid::widget::TextBox::new()
        };
        textbox = style!(textbox, "color");
        textbox = style!(textbox, "font-size");
        textbox = style!(textbox, "text-align");

        if let Some(placeholder) = attrs.get_as::<String>(b"placeholder") {
            textbox.set_placeholder(placeholder);
        }

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
        let child = build_widget(parameter, parsed_map,&new_stack, &elem.childs[0], css)?;
        Scroll::new(child).boxed()
    }

    else if tag == "slider" {
        let min = attrs.get_as_result::<f64>("min").unwrap_or(0f64);
        let max = attrs.get_as_result::<f64>("max").unwrap_or(1f64);
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
        let one = build_widget(parameter, parsed_map, &new_stack, &elem.childs[0], css)?;
        let two = build_widget(parameter, parsed_map, &new_stack, &elem.childs[1], css)?;

        let mut split = if let Some( Cow::Borrowed(b"column") ) = attrs.get(b"direction") {
            Split::columns(one, two)
        } else {
            Split::rows(one, two)
        };
        
        if let Some(v) = attrs.get(b"split_point") {
            let sp = String::from_utf8_lossy(&v).parse::<f64>().map( |e| if e <= 0f64 {0.1} else {e}).unwrap_or(0.5);
            split = split.split_point( sp );
        }
        if let Some(v) = attrs.get(b"min_size") {
            let v = String::from_utf8_lossy(&v);
            let mut splits = v.split(',');
            split = split.min_size( splits.next().unwrap_or("0.0").parse::<f64>().unwrap_or(0.0), splits.next().unwrap_or("0.0").parse::<f64>().unwrap_or(0.0) );
        }
        if let Some(v) = attrs.get(b"bar_size") {
            split = split.bar_size( String::from_utf8_lossy(&v).parse::<f64>().unwrap_or(6.0) );
        }
        if let Some(v) = attrs.get(b"min_bar_area") {
            split = split.min_bar_area( String::from_utf8_lossy(&v).parse::<f64>().unwrap_or(6.0) );
        }
        if let Some(v) = attrs.get(b"draggable") {
            split = split.draggable( String::from_utf8_lossy(&v).parse::<bool>().unwrap_or(false) );
        }
        if let Some(v) = attrs.get(b"solid_bar") {
            split = split.solid_bar( String::from_utf8_lossy(&v).parse::<bool>().unwrap_or(true) );
        }

        split.boxed()
    }

    else if tag == "stepper" {
        let min = attrs.get_as_result::<f64>("min").unwrap_or(std::f64::MIN);
        let max = attrs.get_as_result::<f64>("max").unwrap_or(std::f64::MAX);
        let step = attrs.get_as_result::<f64>("step").unwrap_or(std::f64::MAX);
        let wrap = attrs.get_as_result::<bool>("wraparound").unwrap_or(false);
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
    // else if tag == "container" {
    //     if elem.childs.len() != 1 {
    //         return Err(Error::InvalidContainerChildNum(elem.src_pos))
    //     }
    //     let new_stack = new_parent_stack!();
    //     let child = build_widget(parameter, parsed_map, &new_stack, &elem.childs[0], css)?;
    //     Container::new( child ).boxed()
    // }

    else {
        if tag == "demo_custom_widget" {
            ex_custom_widget::CustomWidget{}.boxed()
        } else {
            if let Some(elem) = parsed_map.get( tag.as_ref() ) {
                let new_stack = new_parent_stack!();
                build_widget(Some(&attrs), parsed_map, &new_stack, elem, css)?
            } else {
                return Err(Error::UnknownTag( (elem.src_pos, tag.as_ref().to_owned() )));
            }
        }
    };

    


    //all component
    //background, padding, 
    {
        //wrap Lens
        //None

        

        //wrap `SizedBox` with optimize
        if get_style!("width").is_some() && get_style!("height").is_some() {
            child = style!(child, "width-height");
        } else {
            child = style!(child, "width");
            child = style!(child, "height");
        }

        //wrap `Padding`
        //child = style!(child, "padding").boxed();
        child = druid::WidgetExt::padding( child, crate::widget::theme::PADDING ).boxed();
        
        //wrap 'Container' 
        //deprecated 
        // if tag != "container" {
        //     child = style!(child, "background-color");
        //     child = style!(child, "border");
        // }
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

        fn transition_option(define:Option<&str>, item:&str) -> Option<AnimationState> {
            let define = if let Some(v) = define {
                v
            } else {
                return None
            };

            for n in define.split(",") {
                let mut duration = 0;
                let mut delay = 0;
                let mut timing_function = crate::simple_style::TimingFunction::Linear;
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
                    "ease" => crate::simple_style::TimingFunction::Ease,
                    "ease-in" => crate::simple_style::TimingFunction::EaseIn,
                    "ease-out" => crate::simple_style::TimingFunction::EaseOut,
                    "ease-in-out" => crate::simple_style::TimingFunction::EaseInOut,
                    "linear" => crate::simple_style::TimingFunction::Linear,
                    "step-start" => crate::simple_style::TimingFunction::Ease,
                    "step-end" => crate::simple_style::TimingFunction::Ease,
                    _ => if expect_tf.starts_with("cubic-bezier(") {
                        let mut params = expect_tf["cubic-bezier(".len() .. expect_tf.rfind(')').unwrap_or(expect_tf.len()-1)].split(',');
                        crate::simple_style::TimingFunction::CubicBezier{
                            p1: params.next().unwrap_or("0").parse().unwrap_or(0.),
                            p2: params.next().unwrap_or("0").parse().unwrap_or(0.),
                            p3: params.next().unwrap_or("0").parse().unwrap_or(0.),
                            p4: params.next().unwrap_or("0").parse().unwrap_or(0.)
                        }
                    } else if expect_tf.starts_with("steps(") {
                        let mut params = expect_tf["steps(".len() .. expect_tf.rfind(')').unwrap_or(expect_tf.len()-1)].split(',');
                        let mut cb = "".to_string();
                        cb.push_str("n:"); params.next().unwrap_or("0"); cb.push_str("f64, ");
                        let jumpterm = match params.next().unwrap_or("jump-start") {
                            "jump-start" => crate::simple_style::JumpTerm::JumpStart,
                            "jump-end" => crate::simple_style::JumpTerm::JumpEnd,
                            "jump-none" => crate::simple_style::JumpTerm::JumpNone,
                            "jump-both" => crate::simple_style::JumpTerm::JumpBoth,
                            "start" => crate::simple_style::JumpTerm::Start,
                            "end" => crate::simple_style::JumpTerm::End,
                            _ => crate::simple_style::JumpTerm::JumpStart
                        };
                        crate::simple_style::TimingFunction::Steps{n:params.next().unwrap_or("0").parse().unwrap(), jumpterm}
                    } else {
                        crate::simple_style::TimingFunction::Linear
                    }
                };
                return Some(crate::simple_style::AnimationState::from( crate::simple_style::Animation{ delay:delay as _, direction: crate::simple_style::Direction::Normal, duration:duration as _, iteration: 1., name: 1., timing_function, fill_mode: 0. } ));
            }
            return None
        }

        
        macro_rules! styler_item {
            ( color, $caller:expr ) => {
                $caller.map(|background| {
                    color::to_color( background, Some(Color::rgb8(255,255,255)) )
                })
            };
            ( insets, $caller:expr ) => {
                $caller.map( |v| {
                    let mut splits = v.split_whitespace().map( |s| &s[..s.find("px").unwrap_or(s.len())] );                    
                    let count = splits.clone().count();
                    if count == 1 {
                        druid::Insets::from( splits.next().unwrap_or("0").parse::<f64>().unwrap_or(0f64) )
                    } else if count == 2 {
                        druid::Insets::from( (splits.next().unwrap_or("0").parse::<f64>().unwrap_or(0f64), splits.next().unwrap_or("0").parse::<f64>().unwrap_or(0f64)) )
                    } else {
                        druid::Insets::from( (splits.next().unwrap_or("0").parse::<f64>().unwrap_or(0f64), splits.next().unwrap_or("0").parse::<f64>().unwrap_or(0f64), splits.next().unwrap_or("0").parse::<f64>().unwrap_or(0f64), splits.next().unwrap_or("0").parse::<f64>().unwrap_or(0f64)) )
                    }
                })
            };
           
            ( f64, $caller:expr ) => {
                $caller.map( |value| {
                    let tv = value.trim();
                    let font_size = match tv.as_bytes() {
                        b"xx-small" => 9f64,
                        b"x-small" => 10f64,
                        b"small" => 13.333f64,
                        b"medium" => 16f64,
                        b"large" => 18f64,
                        b"x-large" => 24f64,
                        b"xx-large" => 32f64,
                        [val @ .. , b'p', b'x'] => String::from_utf8_lossy(val).parse::<f64>().unwrap_or(13.333f64),
                        [val @ .. , b'e', b'm'] => String::from_utf8_lossy(val).parse::<f64>().map( |v| v / 0.0625).unwrap_or(13.333f64),
                        [val @ .. , b'p', b't'] => String::from_utf8_lossy(val).parse::<f64>().map( |v| v * 1.333).unwrap_or(13.333f64),
                        [val @ .. , b'%'] => String::from_utf8_lossy(val).parse::<f64>().map( |v| v / 100f64 / 0.0625 ).unwrap_or(13.333f64),
                        val @ _ => String::from_utf8_lossy(val).parse::<f64>().unwrap_or(13.333f64)
                    };
                    font_size
                } )
            };
            ( border, $caller:expr, $caller_rad:expr ) => {
                $caller.map(|border| {
                    let v = border.trim();
                    let mut splited = v.split_whitespace();
                    let width = splited.next().map( |v| v[..v.find("px").unwrap_or(v.len())].parse::<f64>().unwrap_or(1f64) ).unwrap_or(1f64);
                    //TODO : support other border style?
                    let _border_style = splited.next().unwrap_or("solid");
                    let color = color::to_color(splited.next().unwrap_or("black"), None);
                    let radius = $caller_rad.unwrap_or("0").parse::<f64>().unwrap_or( 0. );
                    BorderStyle::new(width, radius, color)
                })
            };
        }

        let normal_transition = get_style!("transition");
        let normal_style = 
        crate::simple_style::Styler {
            padding : ( styler_item!(insets, get_style!("padding")), transition_option(normal_transition, "padding")),
            margin : ( styler_item!(insets, get_style!("margin")), transition_option(normal_transition, "margin")),
            font_size : ( styler_item!(f64, get_style!("font-size")), transition_option(normal_transition, "font-size")),
            width : ( styler_item!(f64, get_style!("width")), transition_option(normal_transition, "width")),
            height : ( styler_item!(f64, get_style!("height")), transition_option(normal_transition, "height")),
            text_color : ( styler_item!(color, get_style!("color")), transition_option(normal_transition, "color")),
            background_color : ( styler_item!(color, get_style!("background-color")), transition_option(normal_transition, "background-color")),
            border : ( styler_item!(border, get_style!("border"), get_style!("border-radius")), transition_option(normal_transition, "border")),
        };

        let mut pseudo_styles = [None,None,None,None];
        let mut temp_styles = vec![];
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

                macro_rules! get_pseudo_style {
                    ($name:tt) => {
                        rule.declarations.iter().find( |e| e.name == $name ).map( |e| e.value )
                    }
                }

                if let Some(pseudo) = pseudo {
                    pseudo_count += 1;
                    let pseudo_transition = rule.declarations.iter().find( |e| e.name == "transition" ).map( |e| e.value );
                    let styler = crate::simple_style::Styler {
                        padding : ( styler_item!(insets, get_pseudo_style!("padding")), transition_option(pseudo_transition, "padding")),
                        margin : ( styler_item!(insets, get_pseudo_style!("margin")), transition_option(pseudo_transition, "margin")),
                        font_size : ( styler_item!(f64, get_pseudo_style!("font-size")), transition_option(pseudo_transition, "font-size")),
                        width : ( styler_item!(f64, get_pseudo_style!("width")), transition_option(pseudo_transition, "width")),
                        height : ( styler_item!(f64, get_pseudo_style!("height")), transition_option(pseudo_transition, "height")),
                        text_color : ( styler_item!(color, get_pseudo_style!("color")), transition_option(pseudo_transition, "color")),
                        background_color : ( styler_item!(color, get_pseudo_style!("background-color")), transition_option(pseudo_transition, "background-color")),
                        border : ( styler_item!(border, get_pseudo_style!("border"), get_pseudo_style!("border-radius")), transition_option(pseudo_transition, "border")),
                    };
                    
                    let pseudo_styler = match pseudo {
                        Pseudo::Focus => Some(crate::simple_style::PseudoStyle::focus( styler )),
                        Pseudo::Hover => Some(crate::simple_style::PseudoStyle::hover( styler )),
                        Pseudo::Active => Some(crate::simple_style::PseudoStyle::active( styler )),
                        Pseudo::Disabled => Some(crate::simple_style::PseudoStyle::disabled( styler ))
                    };
                    temp_styles.push(pseudo_styler);
                }
            }
        }

        //fill 'None' 
        for (i,e) in temp_styles.into_iter().enumerate() {
            pseudo_styles[i] = e;
        }

        child = crate::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, child ).boxed();
    }

    Ok( child )
}