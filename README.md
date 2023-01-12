# druid-xml : druid ui builder as xml
* EARLY DEVELOPMENT STAGES

## crates
* [`druid-xml`](https://github.com/markijohn/druid-xml/tree/main/druid-xml) : parse xml and genrate rust code or widget
* [`druid-xml-macro`](https://github.com/markijohn/druid-xml/tree/main/druid-xml-macro) : parse xml and generate rust code
* [`druid-xml-example`](https://github.com/markijohn/druid-xml/tree/main/druid-xml-example) : examples

## Quick Example
* Cargo.toml
```toml
[dependencies]
druid-xml-macro = {git="https://github.com/markijohn/druid-xml.git"}
```

* Rust code
```rust
#[macro_use]
extern crate druid_xml_macro;

use druid::{AppLauncher, WindowDesc, Data, Lens};

pub fn main() {
    #[derive(Clone, Lens, Data)]
    struct LoginInfo {
        id : String,
        pwd : String,
        remember : bool
    }

    druid_xml!(
        r#"
        <style>
        #login_label { color:aqua; font-size:2.1em }
        .idpwd_label { width:100px }
        button {width:100px}
        button.ok {width:100px; color:yellow}
        button.cancel {width:100px; color:red}
        </style>


        <flex direction="column" fn="build_main" lens="LoginInfo" axis_alignment="center">
            <label id="login_label">LOGIN</label>
            <spacer/>

            <!-- ID row -->
            <flex>
                <label class="idpwd_label">ID</label>
                <textbox placeholder="id" lens="LoginInfo::id"/>
            </flex>

            <spacer/>

            <!-- Password row -->
            <flex>
                <label class="idpwd_label">Password</label>
                <textbox placeholder="password"  lens="LoginInfo::pwd"/>
            </flex>

            <spacer/>

            <checkbox lens="LoginInfo::remember">Remeber Me</checkbox>

            <spacer/>

            <!-- Button row -->
            <flex style="padding:0px 20px">
                <button class="ok">OK</button>
                <button class="cancel">CANCEL</button>
            </flex>
        </flex>
        "#
    );
    
    let window = WindowDesc::new(build_main())
        .window_size((500., 380.))
        .resizable(false)
        .title("Login sample");
    AppLauncher::with_window(window)
        .launch( LoginInfo { id : "".to_owned(), pwd:"".to_owned(), remember:true } )
        .expect("launch failed");
}
```
* Result
<img src="media/sample_login.png">


## Try online (include demo)
* [Designer with Demo](https://markijohn.github.io/druid-xml-design/)
* The left panel is the xml code editor, the top right is the real-time wasm reflection panel, and the bottom right is the html rendering. html rendering currently has no meaning, but i plan to make it compatible later.

## TODO
* Query wrap : Specific widget wrapperable
* Animation : CSS `transition`
* Drawable widget : like [`Android Drawable`](https://developer.android.com/guide/topics/resources/drawable-resource)
