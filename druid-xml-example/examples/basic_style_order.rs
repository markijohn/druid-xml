#[macro_use]
extern crate druid_xml_macro;

use druid::{AppLauncher, LocalizedString, WindowDesc};

pub fn main() {
    druid_xml!(
        r#"
        <style>
        flex {background-color:white}
        label { color:black; font-size:16px }
        .my_label { color:olive; font-size:1.6em }
        #my_special_label { color:cyan; font-soze:24px }
        </style>
        
        <flex direction="column" fn="build_main" lens="()">
            <!-- specific element style 1st order -->
            <label class="my_label" style="color:blue; font-size:1.6em">Specific Style</label>
        
            <!-- #id define is 2st order -->
            <label class="my_label" id="my_special_label">ID style</label>
        
            <!-- class style -->
            <label class="my_label">Class style</label>
          
            <!-- global style -->
            <label>Global style</label>
            
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
        .launch( () )
        .expect("launch failed");
}