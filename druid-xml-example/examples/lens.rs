#[macro_use]
extern crate druid_xml_macro;

use druid::{AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc,
};

use druid::widget::{MainAxisAlignment, CrossAxisAlignment, Flex, Label, Button, Switch, TextBox, Painter};

#[derive(Clone, Debug, Data, Lens)]
struct MyAppState {
    name : String,
    flag : bool
}

pub fn main() {
    druid_xml!(
        r#"
        <flex fn="build_main" lens="MyAppState">
            <label>Hello Druid!</label>
            <!-- 'input type=text' same as 'textbox' -->
            <input type="text" lens="MyAppState::name" placeholder=""/>
            <switch lens="MyAppState::flag"></switch>
            <button>OK</button>
        </flex>
        "#
    );
    
    let window = WindowDesc::new(build_main)
        .window_size((223., 300.))
        .resizable(false)
        .title(
            LocalizedString::new("demo-title").with_placeholder("Basic lens demo"),
        );
    let state = MyAppState { name : "".to_owned(), flag : false };
    AppLauncher::with_window(window)
        .launch( state )
        .expect("launch failed");
}