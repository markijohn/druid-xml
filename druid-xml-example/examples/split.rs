#[macro_use]
extern crate druid_xml_macro;

use druid::{AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc,
};

use druid::widget::{MainAxisAlignment, CrossAxisAlignment, Flex, Label, Split, Button, Switch, TextBox, Painter};


pub fn main() {
    druid_xml!(
        r#"
        <style>
        label {font-size:1.5em}
        .one { color:yellow }
        .two { color:blue }
        </style>
        <split fn="build_main" style="padding:10px;" lens="()" draggable="true">
            <label class="one">Split One</label>
            <label class="two">Split Two</label>
        </split>
        "#
    );
    
    let window = WindowDesc::new(build_main)
        .window_size((223., 300.))
        .resizable(false)
        .title(
            LocalizedString::new("calc-demo-window-title").with_placeholder("Basic Demo"),
        );
    AppLauncher::with_window(window)
        .launch( ()  )
        .expect("launch failed");
}