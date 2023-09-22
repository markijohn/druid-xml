



use druid::widget::{Flex, TextBox};
use druid::{AppLauncher, WindowDesc, widget::{Button} };
use druid::{WidgetExt, Color, Insets};

use druid_xml::simple_style::{BorderStyle, Styler, AnimationState, Animation, Direction, TimingFunction, PseudoStyle};
use druid_xml::widget::{DXButton, DXLabel};
use druid_xml::widget::style_widget::{SimpleStyleWidget};

use druid_xml::widget::theme;

// what is diffrent margin and padding?
// padding catching the origin widget event and propagation but margin area will ignored
// padding area include as background paint but margin is not

fn main() {
	let simple_linear_anim = Some(AnimationState::from( Animation{ delay: 0, direction: Direction::Normal, duration: 1_000_000_000, iteration: 1., name: 1., timing_function: TimingFunction::Linear, fill_mode: 0. } ));
	let simple_linear_anim_half = Some(AnimationState::from( Animation{ delay: 0, direction: Direction::Normal, duration: 500_000_000, iteration: 1., name: 1., timing_function: TimingFunction::Linear, fill_mode: 0. } ));

	let normal_style = Styler {
		margin: (Some(Insets::new(5., 0., 0., 0.)), simple_linear_anim.clone() ),
		padding : (None,None),
		font_size: (None,None),
		width: (None,None),
		height: (None,None),
		text_color: (None,None),
		background_color: (Some(Color::rgb8(255,0,0)), simple_linear_anim.clone() ),
		border: (None,None),
	};
	let pseudo_styles = [
		Some(PseudoStyle::hover( Styler {
			margin: (Some(Insets::new(25., 0., 0., 0.)), None ),
			padding: (None,None),
			font_size: (None,None),
			width: (None,None),
			height: (None,None),
			text_color: (None,None),
			background_color: (Some(Color::rgb8(0,0,255)),None),
			border: (None,None),
		})),
		None,
		None,
		None
	];
	//let origin = DXLabel::new("Margin Animation").padding(druid_xml::widget::theme::PADDING ).on_click( |ctx, data,env| println!("Clicked2"));
	let origin = TextBox::new().padding(druid_xml::widget::theme::PADDING ).on_click( |_ctx, _data,_env| println!("Clicked2"));
	let margin_style_widget = SimpleStyleWidget::new(normal_style, pseudo_styles, origin );

	let normal_style = Styler {
		padding: (Some(Insets::new(55., 15., 15., 5.)), simple_linear_anim.clone() ),
		margin : (Some(Insets::new(0., 10., 0., 0.)), None ),
		font_size: (Some(14.), simple_linear_anim_half ),
		width: (None,None),
		height: (None,None),
		text_color: (Some(Color::rgb8(0,0,255)), simple_linear_anim.clone() ),
		background_color: (Some(Color::rgb8(255,0,0)), simple_linear_anim.clone() ),
		border: (None,None),
	};
	let pseudo_styles = [
		Some(PseudoStyle::hover( Styler {
			padding: (Some(Insets::new(15., 5., 55., 15.)), None ),
			margin: (None,None),
			font_size: (Some(24.), None ),
			width: (None,None),
			height: (None,None),
			text_color: (Some(Color::rgb8(255,0,0)),None),
			background_color: (Some(Color::rgb8(0,0,255)),None),
			border: (None,None),
		})),
		None,
		None,
		None
	];
	let origin = DXLabel::new("Padding Animation").padding(theme::PADDING ).on_click( |_ctx, _data,_env| println!("Clicked"));
	let padding_style_widget = SimpleStyleWidget::new(normal_style, pseudo_styles, origin );

	let normal_style = Styler {
		padding: (Some(Insets::new(20., 20., 20., 20.)), None ),
		margin : (None, None ),
		font_size: (Some(14.), None ),
		width: (None,None),
		height: (None,None),
		text_color: (None,None),
		background_color: (Some(Color::rgb8(255,0,0)), None ),
		border: (Some(BorderStyle { width:2., radius:1., color:Color::rgb8(255,255,255) }),None),
	};
	let pseudo_styles = [
		
		Some(PseudoStyle::hover( Styler {
			padding: (None, None ),
			margin: (None,None),
			font_size: (None, None ),
			width: (None,None),
			height: (None,None),
			text_color: (None,None),
			background_color: (Some(Color::rgb8(0,0,255)),None),
			border: (Some(BorderStyle { width:10., radius:5., color:Color::rgb8(0,0,255) }), simple_linear_anim),
		})),
		Some(PseudoStyle::active( Styler {
			padding: (None, None ),
			margin: (None,None),
			font_size: (None, None ),
			width: (None,None),
			height: (None,None),
			text_color: (None,None),
			background_color: (Some(Color::rgb8(0,255,0)),None),
			border: (None,None),
		})),
		None,
		None
	];
	let origin = DXButton::new("None Animation").padding(theme::PADDING ).on_click( |_ctx, _data,_env| println!("Clicked"));
	let none_anim_widget = SimpleStyleWidget::new(normal_style, pseudo_styles, origin );

	let flex = Flex::column()
	.with_child(margin_style_widget)
	.with_child(padding_style_widget )
	.with_child(none_anim_widget)
	.with_child( Button::new("OK").border( Color::rgb8(0,0,0), 3.) )
	;

	let window = WindowDesc::new( flex )
		.window_size((640., 480.))
		.resizable(true)
		.title( "DrawableStack demo" );
	AppLauncher::with_window(window)
		.launch( String::new() )
		.expect("launch failed");
}