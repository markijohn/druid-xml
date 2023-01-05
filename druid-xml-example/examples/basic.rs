#[macro_use]
extern crate druid_xml_macro;

use druid::{AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc,
};

use druid::widget::{MainAxisAlignment, CrossAxisAlignment, Flex, Label, Button, Switch, TextBox, Painter};

pub fn main() {
    druid_xml!(
        r#"        
        <!-- The top-level element must have a `fn` `lens` element. -->
        <!-- `fn` is generated function name. -->
        <!-- `lens` is druid `Lens` type. -->
        <flex fn="build_main" lens="()">
            <label>Hello Druid!</label>
            <button>OK</button>
        </flex>
        "#
    );
    
    let window = WindowDesc::new(build_main)
        .window_size((223., 300.))
        .resizable(false)
        .title(
            LocalizedString::new("calc-demo-window-title").with_placeholder("Basic Demo"),
        );
    AppLauncher::with_window(window)
        .launch( () )
        .expect("launch failed");
}