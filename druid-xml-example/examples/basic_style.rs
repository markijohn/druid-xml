#[macro_use]
extern crate druid_xml_macro;

use druid::{AppLauncher, WindowDesc};

pub fn main() {
    druid_xml!(
        r#"
        <style>
        /* class define */
        .one {color:darkgray}
        .two {color:green}

        /* id define */
        #inner {width:200px; background-color:white}
        #inner label {color:black}

        /* global tag define */
        button { font-size:24px }
        </style>

        <flex direction="column" fn="build_main" lens="()">
            <label class="one">One</label>
            <label class="one">Two</label>
            <label class="two">Three</label>
            <label class="two">Four</label>
            <flex id="inner">
                <label>First</label>
                <label>Second</label>
            </flex>
            <button>Button</button>
        </flex>
        "#
    );
    
    let window = WindowDesc::new(build_main() )
        .window_size((320., 300.))
        .resizable(false)
        .title("Style demo");
    AppLauncher::with_window(window)
        .launch( () )
        .expect("launch failed");
}