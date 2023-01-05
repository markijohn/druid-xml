#[macro_use]
extern crate druid_xml_macro;

use druid::{AppLauncher, LocalizedString, WindowDesc};

pub fn main() {
    druid_xml!(
        r#"
        <!-- The top-level element must have a `fn` `lens` element. -->
        <!-- `fn` is generated function name. -->
        <!-- `lens` is druid `Lens` type. -->
        <flex direction="column" fn="build_main" lens="()">
          <flex>
              <label flex="1">Hello Druid!</label>
              <button flex="1">OK</button>
          </flex>
          <label>Second</label>
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