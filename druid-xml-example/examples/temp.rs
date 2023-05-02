
#[macro_use]
extern crate druid_xml_macro;

use druid::{AppLauncher, WindowDesc, Lens, Data};

#[derive(Default,Clone,Data,Lens)]
struct MyApplication {
    name : String,
    hot : bool,
    human : bool,
    address : String
}

fn build_main() -> impl druid::Widget<MyApplication> {
    println!("Start....");
    let mut flex = druid::widget::Flex::column();
    let child = {
            let mut flex = druid::widget::Flex::row();
            let child = {
                    let label = druid_xml::widget::DXLabel::new("MyApplication");
                    let normal_style =
                    druid_xml::simple_style::Styler {
                         padding : (None, None),
                         margin : (None, None),
                         font_size : (None, None),
                         width : (None, None),
                         height : (None, None),
                         text_color : (None, None),
                         background_color : (None, None),
                         border : (None, None),
                    };
                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                    None,
                    None,
                    None,
                    None,
                    ];
                    let label = druid::WidgetExt::padding( label, druid_xml::widget::theme::PADDING );
                    
                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, label )
            };
            flex.add_flex_child(child, 1f64);
            let normal_style =
            druid_xml::simple_style::Styler {
                 padding : (None, None),
                 margin : (None, None),
                 font_size : (None, None),
                 width : (None, None),
                 height : (None, None),
                 text_color : (None, None),
                 background_color : (None, None),
                 border : (None, None),
            };
            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
            None,
            None,
            None,
            None,
            ];
            let flex = druid::WidgetExt::padding( flex, druid_xml::widget::theme::PADDING );
            
            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, flex )
    };
    flex.add_child( child );
    let child = {
            let custom_widget = {
                    let mut flex = druid::widget::Flex::row();
                    flex = flex.must_fill_main_axis(true);
                    flex.set_main_axis_alignment(druid::widget::MainAxisAlignment::SpaceEvenly);
                    let child = {
                            let custom_widget = {
                                    let mut flex = druid::widget::Flex::column();
                                    let child = {
                                            let label = druid_xml::widget::DXLabel::new("⌚");
                                            let normal_style =
                                            druid_xml::simple_style::Styler {
                                                 padding : (None, None),
                                                 margin : (None, None),
                                                 font_size : (None, None),
                                                 width : (None, None),
                                                 height : (None, None),
                                                 text_color : (None, None),
                                                 background_color : (None, None),
                                                 border : (None, None),
                                            };
                                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                            None,
                                            None,
                                            None,
                                            None,
                                            ];
                                            let label = druid::WidgetExt::padding( label, druid_xml::widget::theme::PADDING );
                                            
                                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, label )
                                    };
                                    println!("1");
                                    flex.add_child( child );
                                    let child = {
                                            let label = druid_xml::widget::DXLabel::new("Time");
                                            let normal_style =
                                            druid_xml::simple_style::Styler {
                                                 padding : (None, None),
                                                 margin : (None, None),
                                                 font_size : (None, None),
                                                 width : (None, None),
                                                 height : (None, None),
                                                 text_color : (None, None),
                                                 background_color : (None, None),
                                                 border : (None, None),
                                            };
                                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                            None,
                                            None,
                                            None,
                                            None,
                                            ];
                                            let label = druid::WidgetExt::padding( label, druid_xml::widget::theme::PADDING );
                                            
                                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, label )
                                    };
                                    flex.add_child( child );
                                    let normal_style =
                                    druid_xml::simple_style::Styler {
                                         padding : (None, None),
                                         margin : (None, None),
                                         font_size : (None, None),
                                         width : (None, None),
                                         height : (None, None),
                                         text_color : (None, None),
                                         background_color : (None, None),
                                         border : (None, None),
                                    };
                                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                    None,
                                    None,
                                    None,
                                    None,
                                    ];
                                    let flex = druid::WidgetExt::padding( flex, druid_xml::widget::theme::PADDING );
                                    
                                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, flex )
                            };
                            let normal_style =
                            druid_xml::simple_style::Styler {
                                 padding : (None, None),
                                 margin : (None, None),
                                 font_size : (None, None),
                                 width : (None, None),
                                 height : (None, None),
                                 text_color : (None, None),
                                 background_color : (None, None),
                                 border : (None, None),
                            };
                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                            None,
                            None,
                            None,
                            None,
                            ];
                            let custom_widget = druid::WidgetExt::padding( custom_widget, druid_xml::widget::theme::PADDING );
                            
                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, custom_widget )
                    };
                    flex.add_child( child );
                    let child = {
                            let custom_widget = {
                                    let mut flex = druid::widget::Flex::column();
                                    let child = {
                                            let label = druid_xml::widget::DXLabel::new("⌛");
                                            let normal_style =
                                            druid_xml::simple_style::Styler {
                                                 padding : (None, None),
                                                 margin : (None, None),
                                                 font_size : (None, None),
                                                 width : (None, None),
                                                 height : (None, None),
                                                 text_color : (None, None),
                                                 background_color : (None, None),
                                                 border : (None, None),
                                            };
                                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                            None,
                                            None,
                                            None,
                                            None,
                                            ];
                                            let label = druid::WidgetExt::padding( label, druid_xml::widget::theme::PADDING );
                                            
                                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, label )
                                    };
                                    flex.add_child( child );
                                    println!("2");
                                    let child = {
                                            let label = druid_xml::widget::DXLabel::new("Count");
                                            let normal_style =
                                            druid_xml::simple_style::Styler {
                                                 padding : (None, None),
                                                 margin : (None, None),
                                                 font_size : (None, None),
                                                 width : (None, None),
                                                 height : (None, None),
                                                 text_color : (None, None),
                                                 background_color : (None, None),
                                                 border : (None, None),
                                            };
                                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                            None,
                                            None,
                                            None,
                                            None,
                                            ];
                                            let label = druid::WidgetExt::padding( label, druid_xml::widget::theme::PADDING );
                                            
                                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, label )
                                    };
                                    flex.add_child( child );
                                    let normal_style =
                                    druid_xml::simple_style::Styler {
                                         padding : (None, None),
                                         margin : (None, None),
                                         font_size : (None, None),
                                         width : (None, None),
                                         height : (None, None),
                                         text_color : (None, None),
                                         background_color : (None, None),
                                         border : (None, None),
                                    };
                                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                    None,
                                    None,
                                    None,
                                    None,
                                    ];
                                    let flex = druid::WidgetExt::padding( flex, druid_xml::widget::theme::PADDING );
                                    
                                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, flex )
                            };
                            let normal_style =
                            druid_xml::simple_style::Styler {
                                 padding : (None, None),
                                 margin : (None, None),
                                 font_size : (None, None),
                                 width : (None, None),
                                 height : (None, None),
                                 text_color : (None, None),
                                 background_color : (None, None),
                                 border : (None, None),
                            };
                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                            None,
                            None,
                            None,
                            None,
                            ];
                            let custom_widget = druid::WidgetExt::padding( custom_widget, druid_xml::widget::theme::PADDING );
                            
                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, custom_widget )
                    };
                    flex.add_child( child );
                    let child = {
                            let custom_widget = {
                                    let mut flex = druid::widget::Flex::column();
                                    let child = {
                                            let label = druid_xml::widget::DXLabel::new("✅");
                                            let normal_style =
                                            druid_xml::simple_style::Styler {
                                                 padding : (None, None),
                                                 margin : (None, None),
                                                 font_size : (None, None),
                                                 width : (None, None),
                                                 height : (None, None),
                                                 text_color : (None, None),
                                                 background_color : (None, None),
                                                 border : (None, None),
                                            };
                                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                            None,
                                            None,
                                            None,
                                            None,
                                            ];
                                            let label = druid::WidgetExt::padding( label, druid_xml::widget::theme::PADDING );
                                            
                                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, label )
                                    };
                                    flex.add_child( child );
                                    let child = {
                                            let label = druid_xml::widget::DXLabel::new("Todo");
                                            let normal_style =
                                            druid_xml::simple_style::Styler {
                                                 padding : (None, None),
                                                 margin : (None, None),
                                                 font_size : (None, None),
                                                 width : (None, None),
                                                 height : (None, None),
                                                 text_color : (None, None),
                                                 background_color : (None, None),
                                                 border : (None, None),
                                            };
                                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                            None,
                                            None,
                                            None,
                                            None,
                                            ];
                                            let label = druid::WidgetExt::padding( label, druid_xml::widget::theme::PADDING );
                                            
                                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, label )
                                    };
                                    flex.add_child( child );
                                    let normal_style =
                                    druid_xml::simple_style::Styler {
                                         padding : (None, None),
                                         margin : (None, None),
                                         font_size : (None, None),
                                         width : (None, None),
                                         height : (None, None),
                                         text_color : (None, None),
                                         background_color : (None, None),
                                         border : (None, None),
                                    };
                                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                    None,
                                    None,
                                    None,
                                    None,
                                    ];
                                    let flex = druid::WidgetExt::padding( flex, druid_xml::widget::theme::PADDING );
                                    
                                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, flex )
                            };
                            let normal_style =
                            druid_xml::simple_style::Styler {
                                 padding : (None, None),
                                 margin : (None, None),
                                 font_size : (None, None),
                                 width : (None, None),
                                 height : (None, None),
                                 text_color : (None, None),
                                 background_color : (None, None),
                                 border : (None, None),
                            };
                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                            None,
                            None,
                            None,
                            None,
                            ];
                            let custom_widget = druid::WidgetExt::padding( custom_widget, druid_xml::widget::theme::PADDING );
                            
                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, custom_widget )
                    };
                    flex.add_child( child );
                    println!("3");
                    let child = {
                            let custom_widget = {
                                    let mut flex = druid::widget::Flex::column();
                                    let child = {
                                            let label = druid_xml::widget::DXLabel::new("⚽");
                                            let normal_style =
                                            druid_xml::simple_style::Styler {
                                                 padding : (None, None),
                                                 margin : (None, None),
                                                 font_size : (None, None),
                                                 width : (None, None),
                                                 height : (None, None),
                                                 text_color : (None, None),
                                                 background_color : (None, None),
                                                 border : (None, None),
                                            };
                                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                            None,
                                            None,
                                            None,
                                            None,
                                            ];
                                            let label = druid::WidgetExt::padding( label, druid_xml::widget::theme::PADDING );
                                            
                                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, label )
                                    };
                                    flex.add_child( child );
                                    let child = {
                                            let label = druid_xml::widget::DXLabel::new("Play");
                                            let normal_style =
                                            druid_xml::simple_style::Styler {
                                                 padding : (None, None),
                                                 margin : (None, None),
                                                 font_size : (None, None),
                                                 width : (None, None),
                                                 height : (None, None),
                                                 text_color : (None, None),
                                                 background_color : (None, None),
                                                 border : (None, None),
                                            };
                                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                            None,
                                            None,
                                            None,
                                            None,
                                            ];
                                            let label = druid::WidgetExt::padding( label, druid_xml::widget::theme::PADDING );
                                            
                                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, label )
                                    };
                                    flex.add_child( child );
                                    let normal_style =
                                    druid_xml::simple_style::Styler {
                                         padding : (None, None),
                                         margin : (None, None),
                                         font_size : (None, None),
                                         width : (None, None),
                                         height : (None, None),
                                         text_color : (None, None),
                                         background_color : (None, None),
                                         border : (None, None),
                                    };
                                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                    None,
                                    None,
                                    None,
                                    None,
                                    ];
                                    let flex = druid::WidgetExt::padding( flex, druid_xml::widget::theme::PADDING );
                                    
                                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, flex )
                            };
                            let normal_style =
                            druid_xml::simple_style::Styler {
                                 padding : (None, None),
                                 margin : (None, None),
                                 font_size : (None, None),
                                 width : (Some(40f64), None),
                                 height : (None, None),
                                 text_color : (None, None),
                                 background_color : (None, None),
                                 border : (None, None),
                            };
                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                            None,
                            None,
                            None,
                            None,
                            ];
                            let custom_widget = druid::WidgetExt::fix_width(custom_widget, 40f64);
                            let custom_widget = druid::WidgetExt::padding( custom_widget, druid_xml::widget::theme::PADDING );
                            
                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, custom_widget )
                    };
                    flex.add_child( child );
                    let normal_style =
                    druid_xml::simple_style::Styler {
                         padding : (None, None),
                         margin : (None, None),
                         font_size : (None, None),
                         width : (None, None),
                         height : (None, None),
                         text_color : (None, None),
                         background_color : (None, None),
                         border : (None, None),
                    };
                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                    None,
                    None,
                    None,
                    None,
                    ];
                    let flex = druid::WidgetExt::padding( flex, druid_xml::widget::theme::PADDING );
                    
                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, flex )
            };
            let normal_style =
            druid_xml::simple_style::Styler {
                 padding : (None, None),
                 margin : (None, None),
                 font_size : (None, None),
                 width : (None, None),
                 height : (None, None),
                 text_color : (None, None),
                 background_color : (None, None),
                 border : (None, None),
            };
            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
            None,
            None,
            None,
            None,
            ];
            let custom_widget = druid::WidgetExt::padding( custom_widget, druid_xml::widget::theme::PADDING );
            
            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, custom_widget )
    };
    flex.add_child( child );
    flex.add_default_spacer( );
    let child = {
            let one = {
                    let one = {
                            let custom_widget = {
                                    let mut flex = druid::widget::Flex::column();
                                    let child = {
                                            let label = druid_xml::widget::DXLabel::new("Some1");
                                            let button = druid_xml::widget::DXButton::from_label(label);
                                            let normal_style =
                                            druid_xml::simple_style::Styler {
                                                 padding : (None, None),
                                                 margin : (None, None),
                                                 font_size : (None, None),
                                                 width : (None, None),
                                                 height : (None, None),
                                                 text_color : (None, None),
                                                 background_color : (None, None),
                                                 border : (None, None),
                                            };
                                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                            None,
                                            None,
                                            None,
                                            None,
                                            ];
                                            let button = druid::WidgetExt::padding( button, druid_xml::widget::theme::PADDING );
                                            
                                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, button )
                                    };
                                    flex.add_child( child );
                                    let child = {
                                            let label = druid_xml::widget::DXLabel::new("Some2");
                                            let button = druid_xml::widget::DXButton::from_label(label);
                                            let normal_style =
                                            druid_xml::simple_style::Styler {
                                                 padding : (None, None),
                                                 margin : (None, None),
                                                 font_size : (None, None),
                                                 width : (None, None),
                                                 height : (None, None),
                                                 text_color : (None, None),
                                                 background_color : (None, None),
                                                 border : (None, None),
                                            };
                                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                            None,
                                            None,
                                            None,
                                            None,
                                            ];
                                            let button = druid::WidgetExt::padding( button, druid_xml::widget::theme::PADDING );
                                            
                                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, button )
                                    };
                                    flex.add_child( child );
                                    let normal_style =
                                    druid_xml::simple_style::Styler {
                                         padding : (None, None),
                                         margin : (None, None),
                                         font_size : (None, None),
                                         width : (None, None),
                                         height : (None, None),
                                         text_color : (None, None),
                                         background_color : (None, None),
                                         border : (None, None),
                                    };
                                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                    None,
                                    None,
                                    None,
                                    None,
                                    ];
                                    let flex = druid::WidgetExt::padding( flex, druid_xml::widget::theme::PADDING );
                                    
                                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, flex )
                            };
                            let normal_style =
                            druid_xml::simple_style::Styler {
                                 padding : (None, None),
                                 margin : (None, None),
                                 font_size : (None, None),
                                 width : (None, None),
                                 height : (None, None),
                                 text_color : (None, None),
                                 background_color : (None, None),
                                 border : (None, None),
                            };
                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                            None,
                            None,
                            None,
                            None,
                            ];
                            let custom_widget = druid::WidgetExt::padding( custom_widget, druid_xml::widget::theme::PADDING );
                            
                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, custom_widget )
                    };                      let two = {
                            let custom_widget = {
                                    let mut flex = druid::widget::Flex::column();
                                    let child = {
                                            let mut flex = druid::widget::Flex::row();
                                            let child = {
                                                    let label = druid_xml::widget::DXLabel::new("Name");
                                                    let normal_style =
                                                    druid_xml::simple_style::Styler {
                                                         padding : (None, None),
                                                         margin : (None, None),
                                                         font_size : (None, None),
                                                         width : (None, None),
                                                         height : (None, None),
                                                         text_color : (None, None),
                                                         background_color : (None, None),
                                                         border : (None, None),
                                                    };
                                                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    ];
                                                    let label = druid::WidgetExt::padding( label, druid_xml::widget::theme::PADDING );
                                                         
                                                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, label )
                                            };
                                            println!("4");
                                            flex.add_child( child );
                                            let child = {
                                                    let textbox = druid::widget::TextBox::new();
                                                    let normal_style =
                                                    druid_xml::simple_style::Styler {
                                                         padding : (None, None),
                                                         margin : (None, None),
                                                         font_size : (None, None),
                                                         width : (None, None),
                                                         height : (None, None),
                                                         text_color : (None, None),
                                                         background_color : (None, None),
                                                         border : (None, None),
                                                    };
                                                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    ];
                                                    let textbox = druid::WidgetExt::lens(textbox, MyApplication::name);
                                                    let textbox = druid::WidgetExt::padding( textbox, druid_xml::widget::theme::PADDING );
                                                     
                                                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, textbox )
                                            };
                                            flex.add_flex_child(child, 1f64);
                                            let normal_style =
                                            druid_xml::simple_style::Styler {
                                                 padding : (None, None),
                                                 margin : (None, None),
                                                 font_size : (None, None),
                                                 width : (None, None),
                                                 height : (None, None),
                                                 text_color : (None, None),
                                                 background_color : (None, None),
                                                 border : (None, None),
                                            };
                                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                            None,
                                            None,
                                            None,
                                            None,
                                            ];
                                            let flex = druid::WidgetExt::padding( flex, druid_xml::widget::theme::PADDING );
                                            
                                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, flex )
                                    };
                                    flex.add_child( child );
                                    let child = {
                                            let mut flex = druid::widget::Flex::row();
                                            let child = {
                                                    let label = druid_xml::widget::DXLabel::new("Address");
                                                    let normal_style =
                                                    druid_xml::simple_style::Styler {
                                                         padding : (None, None),
                                                         margin : (None, None),
                                                         font_size : (None, None),
                                                         width : (None, None),
                                                         height : (None, None),
                                                         text_color : (None, None),
                                                         background_color : (None, None),
                                                         border : (None, None),
                                                    };
                                                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    ];
                                                    let label = druid::WidgetExt::padding( label, druid_xml::widget::theme::PADDING );
                                                         
                                                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, label )
                                            };
                                            flex.add_child( child );
                                            let child = {
                                                    let textbox = druid::widget::TextBox::new();
                                                    let normal_style =
                                                    druid_xml::simple_style::Styler {
                                                         padding : (None, None),
                                                         margin : (None, None),
                                                         font_size : (None, None),
                                                         width : (None, None),
                                                         height : (None, None),
                                                         text_color : (None, None),
                                                         background_color : (None, None),
                                                         border : (None, None),
                                                    };
                                                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    ];
                                                    let textbox = druid::WidgetExt::lens(textbox, MyApplication::address);
                                                    let textbox = druid::WidgetExt::padding( textbox, druid_xml::widget::theme::PADDING );
                                                     
                                                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, textbox )
                                            };
                                            flex.add_flex_child(child, 1f64);
                                            let normal_style =
                                            druid_xml::simple_style::Styler {
                                                 padding : (None, None),
                                                 margin : (None, None),
                                                 font_size : (None, None),
                                                 width : (None, None),
                                                 height : (None, None),
                                                 text_color : (None, None),
                                                 background_color : (None, None),
                                                 border : (None, None),
                                            };
                                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                            None,
                                            None,
                                            None,
                                            None,
                                            ];
                                            let flex = druid::WidgetExt::padding( flex, druid_xml::widget::theme::PADDING );
                                            
                                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, flex )
                                    };
                                    flex.add_child( child );
                                    let child = {
                                            let mut flex = druid::widget::Flex::row();
                                            let child = {
                                                    let label = druid_xml::widget::DXLabel::new("Hot?");
                                                    let normal_style =
                                                    druid_xml::simple_style::Styler {
                                                         padding : (None, None),
                                                         margin : (None, None),
                                                         font_size : (None, None),
                                                         width : (None, None),
                                                         height : (None, None),
                                                         text_color : (None, None),
                                                         background_color : (None, None),
                                                         border : (None, None),
                                                    };
                                                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    ];
                                                    let label = druid::WidgetExt::padding( label, druid_xml::widget::theme::PADDING );
                                                         
                                                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, label )
                                            };
                                            flex.add_child( child );
                                            println!("5");
                                            let child = {
                                                    let switch = druid::widget::Switch::new();
                                                    let normal_style =
                                                    druid_xml::simple_style::Styler {
                                                         padding : (None, None),
                                                         margin : (None, None),
                                                         font_size : (None, None),
                                                         width : (None, None),
                                                         height : (None, None),
                                                         text_color : (None, None),
                                                         background_color : (None, None),
                                                         border : (None, None),
                                                    };
                                                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    ];
                                                    let switch = druid::WidgetExt::lens(switch, MyApplication::hot);
                                                    let switch = druid::WidgetExt::padding( switch, druid_xml::widget::theme::PADDING );
                                                       
                                                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, switch )
                                            };
                                            flex.add_flex_child(child, 1f64);
                                            let normal_style =
                                            druid_xml::simple_style::Styler {
                                                 padding : (None, None),
                                                 margin : (None, None),
                                                 font_size : (None, None),
                                                 width : (None, None),
                                                 height : (None, None),
                                                 text_color : (None, None),
                                                 background_color : (None, None),
                                                 border : (None, None),
                                            };
                                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                            None,
                                            None,
                                            None,
                                            None,
                                            ];
                                            let flex = druid::WidgetExt::padding( flex, druid_xml::widget::theme::PADDING );
                                            
                                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, flex )
                                    };
                                    flex.add_child( child );
                                    let child = {
                                            let mut flex = druid::widget::Flex::row();
                                            let child = {
                                                    let label = druid_xml::widget::DXLabel::new("Human?");
                                                    let normal_style =
                                                    druid_xml::simple_style::Styler {
                                                         padding : (None, None),
                                                         margin : (None, None),
                                                         font_size : (None, None),
                                                         width : (None, None),
                                                         height : (None, None),
                                                         text_color : (None, None),
                                                         background_color : (None, None),
                                                         border : (None, None),
                                                    };
                                                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    ];
                                                    let label = druid::WidgetExt::padding( label, druid_xml::widget::theme::PADDING );
                                                         
                                                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, label )
                                            };
                                            flex.add_child( child );
                                            let child = {
                                                    let checkbox = druid::widget::Checkbox::new("Yes");
                                                    let normal_style =
                                                    druid_xml::simple_style::Styler {
                                                         padding : (None, None),
                                                         margin : (None, None),
                                                         font_size : (None, None),
                                                         width : (None, None),
                                                         height : (None, None),
                                                         text_color : (None, None),
                                                         background_color : (None, None),
                                                         border : (None, None),
                                                    };
                                                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    ];
                                                    let checkbox = druid::WidgetExt::lens(checkbox, MyApplication::human);
                                                    let checkbox = druid::WidgetExt::padding( checkbox, druid_xml::widget::theme::PADDING );        
                                                    
                                                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, checkbox )
                                            };
                                            flex.add_flex_child(child, 1f64);
                                            let normal_style =
                                            druid_xml::simple_style::Styler {
                                                 padding : (None, None),
                                                 margin : (None, None),
                                                 font_size : (None, None),
                                                 width : (None, None),
                                                 height : (None, None),
                                                 text_color : (None, None),
                                                 background_color : (None, None),
                                                 border : (None, None),
                                            };
                                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                            None,
                                            None,
                                            None,
                                            None,
                                            ];
                                            let flex = druid::WidgetExt::padding( flex, druid_xml::widget::theme::PADDING );
                                            
                                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, flex )
                                    };
                                    flex.add_child( child );
                                    let normal_style =
                                    druid_xml::simple_style::Styler {
                                         padding : (None, None),
                                         margin : (None, None),
                                         font_size : (None, None),
                                         width : (None, None),
                                         height : (None, None),
                                         text_color : (None, None),
                                         background_color : (None, None),
                                         border : (None, None),
                                    };
                                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                                    None,
                                    None,
                                    None,
                                    None,
                                    ];
                                    let flex = druid::WidgetExt::padding( flex, druid_xml::widget::theme::PADDING );
                                    
                                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, flex )
                            };
                            let normal_style =
                            druid_xml::simple_style::Styler {
                                 padding : (None, None),
                                 margin : (None, None),
                                 font_size : (None, None),
                                 width : (None, None),
                                 height : (None, None),
                                 text_color : (None, None),
                                 background_color : (None, None),
                                 border : (None, None),
                            };
                            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                            None,
                            None,
                            None,
                            None,
                            ];
                            let custom_widget = druid::WidgetExt::padding( custom_widget, druid_xml::widget::theme::PADDING );
                            
                            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, custom_widget )
                    };                      let mut split = druid::widget::Split::columns(one, two);
                    split = split.split_point(0.3f64);
                    split = split.bar_size(3f64);
                    split = split.draggable(true);
                    split = split.solid_bar(true);
                    let normal_style =
                    druid_xml::simple_style::Styler {
                         padding : (None, None),
                         margin : (None, None),
                         font_size : (None, None),
                         width : (None, None),
                         height : (Some(200f64), None),
                         text_color : (None, None),
                         background_color : (None, None),
                         border : (Some(druid_xml::simple_style::BorderStyle::new(1f64, 0f64,druid::Color::rgb8(128,128,128))), None),
                    };
                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                    None,
                    None,
                    None,
                    None,
                    ];
                    let split = druid::WidgetExt::fix_height(split, 200f64);
                    let split = druid::WidgetExt::padding( split, druid_xml::widget::theme::PADDING );
                    
                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, split )
            };              let two = {
                    let label = druid_xml::widget::DXLabel::new("Status");
                    let normal_style =
                    druid_xml::simple_style::Styler {
                         padding : (None, None),
                         margin : (None, None),
                         font_size : (None, None),
                         width : (None, None),
                         height : (None, None),
                         text_color : (None, None),
                         background_color : (None, None),
                         border : (None, None),
                    };
                    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
                    None,
                    None,
                    None,
                    None,
                    ];
                    let label = druid::WidgetExt::padding( label, druid_xml::widget::theme::PADDING );
                    
                    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, label )
            };              let mut split = druid::widget::Split::rows(one, two);
            split = split.split_point(0.9f64);
            let normal_style =
            druid_xml::simple_style::Styler {
                 padding : (None, None),
                 margin : (None, None),
                 font_size : (None, None),
                 width : (None, None),
                 height : (None, None),
                 text_color : (None, None),
                 background_color : (None, None),
                 border : (None, None),
            };
            let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
            None,
            None,
            None,
            None,
            ];
            let split = druid::WidgetExt::padding( split, druid_xml::widget::theme::PADDING );
            
            druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, split )
    };
    flex.add_flex_child(child, 1f64);
    let normal_style =
    druid_xml::simple_style::Styler {
         padding : (None, None),
         margin : (None, None),
         font_size : (None, None),
         width : (None, None),
         height : (None, None),
         text_color : (None, None),
         background_color : (None, None),
         border : (None, None),
    };
    let pseudo_styles:[Option<druid_xml::simple_style::PseudoStyle>;4] = [
    None,
    None,
    None,
    None,
    ];
    let flex = druid::WidgetExt::padding( flex, druid_xml::widget::theme::PADDING );
    
    druid_xml::widget::SimpleStyleWidget::new(normal_style, pseudo_styles, flex )
}


fn main() {
    
    let window = WindowDesc::new(build_main() )
    .window_size((640., 480.))
    .resizable(false)
    .title( "Basic demo" );
AppLauncher::with_window(window)
    .launch( MyApplication::default() )
    .expect("launch failed");
}