#[macro_use]
extern crate druid_xml_macro;

use druid::{AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc,
};

use druid::widget::{MainAxisAlignment, CrossAxisAlignment, Flex, Label, Button, Switch, TextBox, Painter};

#[derive(Clone, Debug, Data, Lens)]
struct AppState {
    text : String,
    flag : bool
}

pub fn main() {
    druid_xml!(
        r#"
        <style>
        flex[fn="build_main"] { background-color:white; padding:20px; }
        label { color: blue; }
        #xml_label { color : darkgray; }
        .cls_btn { width:200px; height:50 }
        </style>

        <flex fn="build_main" lens="AppState" direction="column" axis_alignment="center">
            <label flex="1">Lens DEMO</label>
            <spacer/>
            <textbox placeholder="Input here..." flex="1" lens="AppState::text"/>
            <switch lens="AppState::flag"/>
        </flex>
        "#
    );
    
    let window = WindowDesc::new(build_main)
        .window_size((223., 300.))
        .resizable(false)
        .title(
            LocalizedString::new("calc-demo-window-title").with_placeholder("Basic Demo"),
        );
    let state = AppState { text : "".to_owned(), flag : false };
    AppLauncher::with_window(window)
        .launch( state )
        .expect("launch failed");
}