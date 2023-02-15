

#[macro_use]
extern crate druid_xml_macro;

use druid::{AppLauncher, WindowDesc, WidgetExt};

fn main() {
    druid_xml!(
        r#"
        <style>
        /* class define */
        label { background-color:rgb(0,0,255); font-size:24px; border:3px solid white; border-radius:5px; padding:10px; margin:0 20 0 0 }
        .anim_margin { transition:2s margin linear }
        .anim_margin:hover { margin:100 20 0 0 }
        
        .anim_padding { transition:2s padding ease }
        .anim_padding:hover { padding:100 10 10 10 }
        
        .anim_font_size { transition:2s font-size ease-in }
        .anim_font_size:hover { font-size:52px }
        
        .anim_backcol { transition:2s background-color ease-in-out }
        .anim_backcol:hover { background-color:rgb(255,200,200) }

        .two {color:green}

        /* id define */
        #inner {width:200px; background-color:white}
        #inner label {color:black}

        /* global tag define */
        button { font-size:24px }
        </style>

        <flex direction="column" fn="build_main" lens="()">
            <label class="anim_margin">Margin animation (Linear)</label>
            <label class="anim_padding">Padding (Ease)</label>
            <label class="anim_font_size">Font-size (EaseIn)</label>
            <label class="anim_backcol">Background-color (EaseInOut)</label>
        </flex>
        "#
    );

    let window = WindowDesc::new(build_main() )
    .window_size((640., 480.))
    .resizable(false)
    .title( "Basic demo" );
AppLauncher::with_window(window)
    .launch( () )
    .expect("launch failed");
}