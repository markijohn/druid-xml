use std::str::FromStr;



use druid::widget::Flex;
use druid::{AppLauncher, WindowDesc, widget::Label};
use druid::{WidgetExt, Color};
use druid_xml::qwidget::drawable::*;
use druid_xml::simple_style::BorderStyle;

fn main() {
	let _border_color = Color::rgb8(64,64,64);
	let col_red = Color::rgb8(128,0,0);
	let col_blue = Color::rgb8(0,0,128);
	let col_white = Color::rgb8(255,255,255);

	let raw_drawable = Label::new( "TEXT" ).with_text_size(28.).padding( (100., 100.) );
	let draw_stack = DrawableStack::new( vec![

		//Rounded fill
		Drawable::Rect {
			top:Number::Abs(0.),
			right:Number::Rel(1f64),
			bottom:Number::Rel(1f64),
			left:Number::from_str("calc(100% - 20)").unwrap(),
			border: Some(BorderStyle::new(
				//StrokeStyle{ line_join: Default::default(), line_cap: Default::default(), dash_pattern: Default::default(), dash_offset: 0.5 } ,
				2f64,
				0.,
				Color::rgb8(0,0,0)
			)), 
			round: None, 
			fill: FillMethod::Solid(col_red) }

		,Drawable::Circle { 
			center: QVec2::from("35", "15").unwrap(), 
			radius: Number::Abs(15.), 
			border: Some(BorderStyle::new(
				//Default::default(), 
				3. , 0., col_blue ) ), 
			fill: FillMethod::Solid( col_white ) }

		,Drawable::Ellipse {
			center:QVec2::from("50%","15").unwrap(),
			border:Some(BorderStyle::new(
				//Default::default(),
				3., 0., col_blue)),
			fill:FillMethod::Solid(col_white), 
			radi: QVec2::from("13","15").unwrap(), 
			x_rot: Number::Abs(0.7) }
	] );

	

	let raw_drawable = raw_drawable.background( draw_stack.to_background() );

	let flex = Flex::row().with_child( raw_drawable );

	let window = WindowDesc::new( flex )
		.window_size((640., 480.))
		.resizable(true)
		.title( "DrawableStack demo" );
	AppLauncher::with_window(window)
		.launch( () )
		.expect("launch failed");
}