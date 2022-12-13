

/// This code not used in this example but how generation actual code from macro syntax 
#[allow(dead_code)]
fn will_generation_code() -> Box<dyn Widget> {

}



pub fn main() {
	let native_custom_widget = |attrs| {

	};

	struct MyData {
		id : String,
		pwd : String
	}

	fn login(myData:&mut MyData) {

	}

	fn exit() {
		Application::exit();
	}

	let widget = build_widget!( MyData, 
		{
			<style>
			label:hover { color:#333333 }
			button {color:black, background-color:white}
			textbox {color:black, background-color:gray}
			#pwd {color:white, background-color:black}
			</style>

			<widget name=icon>
				<flex direction="row">
					<label style="font-size:25px">${icon_text}</label>
					<label style="font-size:10px">${title}</label>
				</flex>
			</widget>

			<flex direction="row">
				<label>Login..</label>

				<widget name=native_custom_widget title="GO"/>
				<widget name=native_custom_widget title="MAIN"/>
				<widget name=native_custom_widget title="NO"/>
				<icon title="Exit" icon="★" onclick="exit"/>

				<!-- you can remove direction="col" attribute because that default value is "col" and also other all default value is ignorable -->
				<flex direction="col" cross_alignment="" main_alignment="" fill_major_axis="true">
					<label>ID</label><textbox class="normal" lens="id" value="Default Value" placeholder="Input here"/>
					<label>PWD</label><textbox lens="pwd" placeholder="Your password"/>
				</flex>

				<flex>
					<button onclick="login">OK</button>
					<button style="background-color:red; color:white">CANCEL</button>
				</flex>
			</flex>
		},

		"#pwd" => |widget, _data| widget.on_click( || println!("Clicked button")
		
	);
	
	let widget = build_widget!( "my_design.xml" );
}