#[macro_use]
extern crate druid_xml_macro;

use druid::{AppLauncher, WindowDesc, Data, Lens};

pub fn main() {
    druid_xml!(
        r##"
<style>
[fn=tabmenu] label {font-size:12px; padding:5; color:rgb(154, 160, 166)}
[fn=tabmenu] label:hover {background-color:black}
[fn=tabmenu] label:active {background-color:gray; color:white}
[fn=console] .status { color:rgb(154, 160, 166) }
console_sep:hover { background-color:darkgray }
.console_msg label { font-size:13px }
[fn=console_info] .info { color:white }
[fn=console_error] .error { color:red }
[fn=console_warn] {background-color: rgba(138, 109, 61, 77)}
[fn=console_warn] .warn { color:yellow; }
</style>
 
<label fn="icon">${ico}</label>

<flex fn="tabmenu" must_fill_main_axis="true">
  <icon ico="☝"/>
  <icon ico="▣"/>
  <label>Elements</label>
  <label>Network</label>
  <label>Performance Insights</label>
  <label>Recoder</label>
  <label class="tab_selected">Console</label>
  <label>Performance</label>
  <label>Sources</label>
  <label>Application</label>
  <label>Memory</label>
  <label>Security</label>
</flex>

<flex fn="console_sep">
  <label>▶</label>
  <label>${ico}</label>
  <label flex="1">${sep}</label>
</flex>

<flex fn="console_info" class="console_msg">
  <label style="color:cyan">${count}</label>
  <label flex="1" class="info">${msg}</label>
</flex>

<flex fn="console_error" class="console_msg">
  <label style="color:cyan">${count}</label>
  <label flex="1" class="error">${msg}</label>
</flex>

<flex fn="console_warn" class="console_msg">
  <label style="color:cyan">${count}</label>
  <label flex="1" class="warn">${msg}</label>
</flex>

<flex fn="console" direction="column">
  <split flex="1" direction="column" split_point="0.3" draggable="true">
    <flex direction="column">
      <console_sep ico="▤" sep="All message"/>
      <console_sep ico="⛄" sep="User message" class="tab_selected" />
      <console_sep ico="⛔" sep="errors"/>
      <console_sep ico="⚠" sep="warinigs"/>
    </flex>

    <flex direction="column" must_fill_main_axis="true">
      <console_info count="⑩" msg="TextLayout alignment unsupported on web"/>
      <console_warn count="⑦" msg="DevTools failed to load source map"/>
      <console_error count="④" msg="Runtime unreachable"/>
    </flex>
  </split>
  <flex must_fill_main_axis="true" style="border:1px solid lightgray"> 
    <label class="status">Status message</label>
  </flex>
</flex>

<flex fn="build_main" lens="()" direction="column" must_fill_main_axis="true">
  <tabmenu style="border: 1px solid gray"/>
  <console flex="1"/>
</flex>
        "##
    );
    
    let window = WindowDesc::new(build_main())
        .window_size((800., 500.))
        .resizable(true)
        .title("DevTools sample");
    AppLauncher::with_window(window)
        .launch( () )
        .expect("launch failed");
}