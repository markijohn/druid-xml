extern crate proc_macro;


use std::collections::HashMap;

use proc_macro::{TokenStream};
use syn::{parse_macro_input, Result, Token};
use syn::parse::{Parse, ParseStream};
use quote::quote;

#[proc_macro]
pub fn druid_xml( input:TokenStream ) -> TokenStream {
	
	struct DruidXML {
		xml_src : syn::LitStr,
		sep2 : Option<Token![,]>,
		maps : Option<Vec<WidgetWrapper>>,
	}

	//like syn::Arm
	// old style : but we want avoid type annotation
	//
	//druid_xml!( r#"
	//",
	//"button" => |btn:Button::<()> | {
	///   println!("On clicked");
	//});
	// struct WidgetWrapper {
	// 	query : syn::LitStr,
	// 	sep : Token![=>],
	// 	bindfn : syn::ExprClosure,
	// 	sepe : Option<Token![,]>,
	// }

	//
	struct WidgetWrapper {
		query : syn::LitStr,
		sep : Token![=>],
		bindfn : syn::ExprBlock,
		sepe : Option<Token![,]>,
	}

	impl Parse for WidgetWrapper {
		fn parse(input: ParseStream) -> Result<Self> {
			Ok( WidgetWrapper {
				query : input.parse()?,
				sep : input.parse()?,
				bindfn : input.parse()?,
				sepe : input.parse()?
			})
		}
	}

	impl Parse for DruidXML {
		fn parse(input: ParseStream) -> Result<Self> {
			let xml_src:syn::LitStr = input.parse()?;
			let sep2 = input.parse()?;
			let mut maps:Vec<WidgetWrapper> = vec![];
			loop {
				if let Ok(ev) = input.parse::<WidgetWrapper>() {
					maps.push( ev );
				} else {
					break
				}
			}
			let maps = if !maps.is_empty() {
				Some(maps)
			} else {
				None
			};
			Ok( DruidXML { xml_src, sep2, maps } )
		}
	}

	let druid_xml = parse_macro_input!(input as DruidXML);
	let mut wrapper_maps = HashMap::new();
	if let Some(wrappers) = druid_xml.maps.as_ref() {
		wrappers.iter().for_each( |e| {
			let w = &e.bindfn;
			wrapper_maps.insert(e.query.value(), quote!(#w).to_string() );
		});
	}
	let ui_code = druid_xml::compile(&druid_xml.xml_src.value(), &wrapper_maps).unwrap();
	ui_code.parse().unwrap()
}
