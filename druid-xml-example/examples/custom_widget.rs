#[macro_use]
extern crate druid_xml_macro;

use druid::{AppLauncher, LocalizedString, WindowDesc, Data, Lens};

pub fn main() {
    #[derive(Clone, Lens, Data)]
    struct MyAppState {
        name : String
    }

    druid_xml!(
        r#"
        <style>
        .wrap_border { padding:10; border:5px solid cyan; width:200px; height:50px; }
        </style>
        
        <flex fn="my_custom">
          <label>Label</label>
          <textbox lens="MyAppState::name"/>
        </flex>
        
        <flex fn="my_custom_param">
          <label>${name}</label>
          <textbox placeholder="${placeholder}" lens="MyAppState::name"/>
        </flex>
        
        <flex direction="column" fn="build_main" lens="MyAppState" must_fill_main_axis="true" axis_alignment="spaceevenly">
          <!-- map custom widget -->
          <my_custom/>
        
          <!-- custom widget with style -->
          <my_custom class="wrap_border"/>
          
          <!-- custom widget with parameter -->
          <my_custom_param name="MyName" placeholder="Input here..."/>
        </flex>
        "#
    );
    
    let window = WindowDesc::new(build_main)
        .window_size((223., 300.))
        .resizable(false)
        .title(
            LocalizedString::new("basic-demo").with_placeholder("Basic Demo"),
        );
    AppLauncher::with_window(window)
        .launch( MyAppState { name : "".to_owned() } )
        .expect("launch failed");
}