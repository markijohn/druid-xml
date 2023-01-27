use std::borrow::Cow;
use serde_json::Value;

pub struct Attribute {
	pub name : Cow<'static,str>,
	pub value : Value
}