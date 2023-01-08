
use druid::{AppLauncher, LocalizedString, WindowDesc,Widget};

fn build_main() -> Box<dyn Widget<()>> {
    let map = druid_xml::dynamic::generate_widget(
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
    ).unwrap();
    for (name,w) in map.into_iter() {
        if name == "build_main" {
            return w
        }
    }
    panic!("Can't find build_main")
}

fn main() {
    let window = WindowDesc::new(build_main)
        .window_size((223., 300.))
        .resizable(false)
        .title(
            LocalizedString::new("basic-demo").with_placeholder("Dynamic Basic Demo"),
        );
    AppLauncher::with_window(window)
        .launch( () )
        .expect("launch failed");
}