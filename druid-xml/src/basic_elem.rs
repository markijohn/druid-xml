use crate::ElementSource;




pub trait ElementSource {
	fn new(attrs:Attributes) -> Self where Self:Sized;

	fn tag(&self) -> &'static str;

	fn append_child(&mut self) -> Result<(), DruidXMLError> {
		Err( DruidXMLError::ChildlessElement )
	}

	fn write_source(&self, src:&mut String);
}

struct ButtonElement;
impl ElementSource for ButtonElement {
	
}