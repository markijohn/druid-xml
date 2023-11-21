
use druid::{AppLauncher,WindowDesc,Widget};
use druid_xml::qwidget::value::JSValue;

fn build_main() -> Box<dyn Widget<JSValue>> {
    // druid_xml::dynamic::generate_widget(
    //     r#"
    //     <!-- The top-level element must have a `fn` `lens` attribute. -->
    //     <!-- `fn` is generated function name. -->
    //     <!-- `lens` is druid `Lens` type. -->
    //     <flex direction="column" fn="build_main" lens="()">
    //       <flex>
    //           <label flex="1">Hello Druid!</label>
    //           <button flex="1">OK</button>
    //       </flex>
    //       <label>Second</label>
    //     </flex>
    //     "#
    // ).unwrap()

    druid_xml::dynamic::generate_widget(
        r#"
        <style>
        
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

        #inner {width:200px; background-color:white}
        #inner label {color:black}

        button { font-size:24px }
        </style>

        <flex direction="column" fn="build_main" lens="()">
            <label class="anim_margin">Margin animation (Linear)</label>
            <label class="anim_padding">Padding (Ease)</label>
            <label class="anim_font_size">Font-size (EaseIn)</label>
            <label class="anim_backcol">Background-color (EaseInOut)</label>
            <button> Hello? </button>
        </flex>
        "#
    ).unwrap()
}

fn main() {
    let window = WindowDesc::new(build_main())
        .window_size((600., 500.))
        .resizable(true)
        .title("Dynamic demo");
    AppLauncher::with_window(window)
        .launch( JSValue::default() )
        .expect("launch failed");
}