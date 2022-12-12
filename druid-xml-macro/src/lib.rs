#[macro_use]
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn druid_xml_ui( tks:TokenStream ) -> TokenStream {
	let xml_source = tks.to_string();

	todo!()
}