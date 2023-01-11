#[macro_use]
extern crate druid_xml_macro;

use druid::{AppLauncher, LocalizedString, WindowDesc, Data, Lens};

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
    
    let window = WindowDesc::new(build_main)
        .window_size((223., 300.))
        .resizable(false)
        .title(
            LocalizedString::new("basic-demo").with_placeholder("Basic Demo"),
        );
    AppLauncher::with_window(window)
        .launch( LoginInfo { id : "".to_owned(), pwd:"".to_owned(), remember:true } )
        .expect("launch failed");
}