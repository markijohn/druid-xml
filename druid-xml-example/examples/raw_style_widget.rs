use std::str::FromStr;

use druid::kurbo::Ellipse;
use druid::piet::StrokeStyle;
use druid::widget::Flex;
use druid::{AppLauncher, WindowDesc, widget::{Label,Button} };
use druid::{WidgetExt, Color, Vec2, Insets};
use druid_xml::qwidget::drawable::*;
use druid_xml::simple_style::{BorderStyle, Styler, AnimationState, Animation, Direction, TimingFunction};
use druid_xml::widget::style_widget::{SimpleStyleWidget, PseudoStyle};

// what is diffrent margin and padding?
// padding catching the origin widget event and propagation but margin area will ignored
// padding area include as background paint but margin is not

fn main() {
	let normal_style = Styler {
		margin: (Some(Insets::new(5., 0., 0., 0.)), Some(AnimationState::from( Animation{ delay: 0., direction: Direction::Normal, duration: 1000_000_000, iteration: 1., name: 1., timing_function: TimingFunction::Linear, fill_mode: 0. } )) ),
		padding : (None,None),
		font_size: (None,None),
		width: (None,None),
		height: (None,None),
		text_color: (None,None),
		background_color: (Some(Color::rgb8(255,0,0)), Some(AnimationState::from( Animation{ delay: 0., direction: Direction::Normal, duration: 1000_000_000, iteration: 1., name: 1., timing_function: TimingFunction::Linear, fill_mode: 0. } )) ),
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
		None
	];
	let margin_style_widget = SimpleStyleWidget::new(normal_style, pseudo_styles, Label::new("Margin Animation").with_text_size(24.) );

	let normal_style = Styler {
		padding: (Some(Insets::new(55., 15., 15., 5.)), Some(AnimationState::from( Animation{ delay: 0., direction: Direction::Normal, duration: 1000_000_000, iteration: 1., name: 1., timing_function: TimingFunction::Linear, fill_mode: 0. } )) ),
		margin : (Some(Insets::new(0., 10., 0., 0.)), None ),
		font_size: (None,None),
		width: (None,None),
		height: (None,None),
		text_color: (None,None),
		background_color: (Some(Color::rgb8(255,0,0)), Some(AnimationState::from( Animation{ delay: 0., direction: Direction::Normal, duration: 500_000_000, iteration: 1., name: 1., timing_function: TimingFunction::Linear, fill_mode: 0. } )) ),
		border: (None,None),
	};
	let pseudo_styles = [
		Some(PseudoStyle::hover( Styler {
			padding: (Some(Insets::new(15., 5., 55., 15.)), None ),
			margin: (None,None),
			font_size: (None,None),
			width: (None,None),
			height: (None,None),
			text_color: (None,None),
			background_color: (Some(Color::rgb8(0,0,255)),None),
			border: (None,None),
		})),
		None,
		None
	];
	let padding_style_widget = SimpleStyleWidget::new(normal_style, pseudo_styles, Label::new("Padding Animation").with_text_size(24.) );

	let flex = Flex::column()
	.with_child(margin_style_widget)
	.with_child(padding_style_widget)
	.with_child( Button::new("OK").border( Color::rgb8(0,0,0), 3.) )
	;

	let window = WindowDesc::new( flex )
		.window_size((640., 480.))
		.resizable(true)
		.title( "DrawableStack demo" );
	AppLauncher::with_window(window)
		.launch( () )
		.expect("launch failed");
}