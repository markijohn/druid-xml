#[macro_use]
extern crate druid_xml_macro;

use druid::{AppLauncher, WindowDesc};

#[allow(unused)]
fn build_main_actual() -> impl druid::Widget<()> {
    let mut flex = druid::widget::Flex::column();
    let child = {
            let mut flex = druid::widget::Flex::row();
            let child = {
                    let mut label = druid::widget::Label::new("Hello Druid!");
                    label
            };
            flex.add_flex_child(child, 1f64);
            let child = {
                    let mut label = druid::widget::Label::new("OK");
                    let button = druid::widget::Button::from_label(label);
                    let button = (|btn:druid::widget::Button<()>| {
                            btn.on_click( |_,_,_| {
                                    println!("On clicked");
                            })
                    }) (button);
                    button
            };
            flex.add_flex_child(child, 1f64);
            flex
        
    };
    flex.add_child( child );
    let child = {
            let mut label = druid::widget::Label::new("Second");
            label
    };
    flex.add_child( child );
    flex
}

pub fn main() {
    druid_xml!(
        r#"
        <!-- The top-level element must have a `fn` `lens` element. -->
        <!-- `fn` is generated function name. -->
        <!-- `lens` is druid `Lens` type. -->
        <flex direction="column" fn="build_main" lens="()">
          <flex>
              <label flex="1">Hello Druid!</label>
              <button id="my_btn" flex="1">OK</button>
          </flex>
          <label>Second</label>
        </flex>
        "#,
        "#my_btn" => {
            widget.on_click( |_,_,_| {
                println!("On clicked");
            })
        }
    );
    
    let window = WindowDesc::new(build_main() )
        .window_size((640., 480.))
        .resizable(false)
        .title( "Basic demo" );
    AppLauncher::with_window(window)
        .launch( () )
        .expect("launch failed");
}