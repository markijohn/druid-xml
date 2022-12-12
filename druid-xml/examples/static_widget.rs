

/// This code not used in this example but how generation actual code from macro syntax 
#[allow(dead_code)]
fn will_generation_code() -> Box<dyn Widget> {

}



pub fn main() {
	let my_widget = druid_xml! {
		<flex direction="row">
			<label>Login..</label>

			<!-- you can remove direction="col" attribute because that default value is "col" and also other all default value is ignorable -->
			<flex direction="col" cross_alignment="" main_alignment="" fill_major_axis="true">
				<textbox value="Default Value" placeholder="Input here"/>
				<textbox id="pwd" placeholder="Your password"/>
			</flex>

			<flex>
				<button>OK</button>
				<button>CANCEL</button>
			</flex>
		</flex>

		,
		label:hover { color:#333333 }
		button {color:black, background-color:white}
		textbox {color:black, background-color:gray}
		#pwd {color:white, background-color:black}
		,
		
		onclick : {
			"#pwd" => {

			}
		},
		onchange : {
			"textbox" => {

			}
		},
		onfocus : {
			"label" => {
				self.set_font_size( .20 );
			}
		},
		onblur : {
			"label" => {
				self.set_font_size( .10 );
			}
		}
	};


	let (userid,pwd) = druid_xml_map! {
		<flex>
			<flex direction="col" cross_alignment="" main_alignment="" fill_major_axis="true">
				<textbox id="userid" value="Default Value" placeholder="Input here"/>
				<textbox id="pwd" value="Default Value" placeholder="Your password"/>
			</flex>

			<flex direction="col">
				<button>OK</button>
				<button>CANCEL</button>
			</flex>
		</flex>

		,
		button {color:black, background-color:white}
		textbox {color:black, background-color:gray}
		#pwd {color:white, background-color:black}
	};
}