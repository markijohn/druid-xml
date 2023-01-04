#[macro_use]
extern crate druid_xml_macro;

use druid::{AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc,
};

use druid::widget::{MainAxisAlignment, CrossAxisAlignment, Flex, Label, Button, Switch, TextBox, Painter};

pub fn main() {
    druid_xml!(
        r#"
        <style>
        flex { background-color:white; padding:20px; }
        label { color: blue; }
        #xml_label { color : darkgray; }
        .cls_btn { width:200px; height:50 }
        </style>

        <flex fn="build_main" lens="()" direction="column" axis_alignment="end">
            <label>HI</label>
            <label style="color:yellow">Druid</label>
            <label id="xml_label">XML</label>
            <button class="cls_btn">MyButton</button>
            <button class="cls_btn"/>
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