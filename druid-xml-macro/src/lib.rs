extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use syn::{parse_macro_input, Result, Token};
use syn::parse::{Parse, ParseStream};


#[proc_macro]
pub fn druid_xml( input:TokenStream ) -> TokenStream {
	
	struct DruidXML {
		mlens : syn::Ident,
		sep1 : Token![,],
		xml_src : syn::LitStr,
		sep2 : Option<Token![,]>,
		maps : Vec<EventMap>,
	}

	//like syn::Arm
	struct EventMap {
		query : syn::LitStr,
		sep : Token![=>],
		bindfn : syn::ExprClosure,
		sepe : Option<Token![,]>,
	}

	impl Parse for EventMap {
		fn parse(input: ParseStream) -> Result<Self> {
			Ok( EventMap {
				query : input.parse()?,
				sep : input.parse()?,
				bindfn : input.parse()?,
				sepe : input.parse()?
			})
		}
	}

	impl Parse for DruidXML {
		fn parse(input: ParseStream) -> Result<Self> {
			let mlens:syn::Ident = input.parse()?;
			let sep1 = input.parse()?;
			let xml_src:syn::LitStr = input.parse()?;
			let sep2 = input.parse()?;
			let mut maps:Vec<EventMap> = vec![];
			loop {
				if let Ok(ev) = input.parse::<EventMap>() {
					maps.push( ev );
				} else {
					break
				}
			}
			Ok( DruidXML { mlens, sep1, xml_src, sep2, maps } )
		}
	}

	let druid_xml = parse_macro_input!(input as DruidXML);
	
	TokenStream::new()
}
