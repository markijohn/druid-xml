use druid::{AppLauncher, WindowDesc, WidgetExt, widget::LensWrap};
use druid_xml::widget::tree_label::TreeLabel;

fn build_main() -> impl druid::Widget<String> {
    let mut flex = druid::widget::Flex::<String>::column();
    let child = {
            let mut flex = druid::widget::Flex::<String>::row();
            let child = {
				TreeLabel::<String>::new()
            };
            flex.add_flex_child(child, 1f64);
			
            let child = {
                    let mut label = druid::widget::Label::new("OK");
                    let button = druid::widget::Button::from_label(label);
                    
                    (|btn:druid::widget::Button<_>| {
                            btn.on_click( |_,_,_| {
                                    println!("On clicked");
                            })
                    }) (button)
            };
            flex.add_flex_child(child, 1f64);
            flex
        
    };
    flex.add_child( child );
    let child = {
            
            druid::widget::Label::new("Second")
    };
    flex.add_child( child );
    flex
}

pub fn main() {
    let window = WindowDesc::new(build_main() )
        .window_size((640., 480.))
        .resizable(false)
        .title( "Basic demo" );
    AppLauncher::with_window(window)
        .launch( "MyTreeLabel".to_string() )
        .expect("launch failed");
}