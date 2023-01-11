#[macro_use]
extern crate druid_xml_macro;

use druid::{AppLauncher, LocalizedString, WindowDesc, Data, Lens};

pub fn main() {
    #[derive(Default, Clone, Lens, Data)]
    struct MyApplication {
        name : String,
        address : String,
        hot : bool,
        human : bool
    }

    druid_xml!(
        r#"
<style>
  #title {font-size:13px; color:dodgerblue; padding:10px;}
  [fn=icon] { border:1px solid white; width:60px }
  [fn=icon] .icon { font-size:25px; color:cyan }
  [fn=icon] .desc { font-size:10px }
  [fn=split_right] label { width:80px; }
  [fn=split_right] flex { padding:2 2 }
</style> 

<!-- icon -->
<flex fn="icon" direction="column" style="">
  <label class="icon">${ico}</label>
  <label class="desc">${desc}</label>
</flex>

<!-- icon bar -->
<flex fn="my_icon_bar"  must_fill_main_axis="true" axis_alignment="spaceevenly">
    <icon fn="icon" ico="⌚" desc="Time"/>
    <icon fn="icon" ico="⌛" desc="Count"/>
    <icon fn="icon" ico="✅" desc="Todo"/>
    <icon fn="icon" ico="⚽" desc="Play" style="width:40px"/>
</flex>

<!-- split left -->
<flex fn="split_left" direction="column">
  <button>Some1</button>
  <button>Some2</button>
</flex>

<!-- split right -->
<flex fn="split_right" direction="column">
  <flex><label>Name</label> <textbox flex="1" lens="MyApplication::name"/></flex>
  <flex><label>Address</label> <textbox flex="1" lens="MyApplication::address"/></flex>
  <flex><label>Hot?</label> <switch flex="1" lens="MyApplication::hot"/></flex>
  <flex><label>Human?</label> <checkbox flex="1" lens="MyApplication::human">Yes</checkbox></flex>
</flex>  

<!-- main frame -->
<flex direction="column" fn="build_main" lens="MyApplication">
  <flex>
    <label flex="1" id="title">MyApplication</label>
  </flex>
  
  <my_icon_bar/>
  
  <spacer/>
  
  <split direction="row" flex="1" split_point="0.9">
    <split direction="column" flex="1" bar_size="3" solid_bar="true" draggable="true" style="height:200px; border:1px solid gray;" split_point="0.3" >
      <split_left/>
      <split_right/>
    </split>
    <label>Status</label>
  </split>
</flex>
        "#
    );
    
    
    let window = WindowDesc::new(build_main)
        .window_size((320., 480.))
        .resizable(false)
        .title(
            LocalizedString::new("basic-demo").with_placeholder("Basic Demo"),
        );
    AppLauncher::with_window(window)
        .launch( MyApplication::default() )
        .expect("launch failed");
}