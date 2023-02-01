use std::collections::VecDeque;
use std::rc::Rc;

use crate::simple_style::{Animation, Styler, BorderStyle};
use druid::commands::SCROLL_TO_VIEW;
use druid::kurbo::{Affine, Insets, Point, Rect, Shape, Size};
use druid::{
    ArcStr, BoxConstraints, Color, Command, Cursor, Data, Env, Event, EventCtx, InternalEvent,
    InternalLifeCycle, LayoutCtx, LifeCycle, LifeCycleCtx, Notification, PaintCtx, Region,
    RenderContext, Target, TextLayout, UpdateCtx, Widget, WidgetId, WindowId, WidgetPod,
};


/// A StyleWidget for `hover` and `animation` effect
/// This widget changed `Env` and support `Padding` and `Conatainer`
pub struct SimpleStyleWidget<T, W> {
	normal_style : Styler,
	hover_style : Option<Styler>,
	active_style : Option<Styler>,
	focus_style : Option<Styler>,

    padding : Option<Insets>,
    margin : Option<Insets>,
    font_size : Option<f64>,
    width : Option<f64>,
    height : Option<f64>,
    text_color : Option<Color>,
    background_color : Option<Color>,
    border : Option<BorderStyle>,

	last_hover : bool,
	last_focus : bool,
	last_active : bool,
	inner : WidgetPod<T,W>
}

impl<T, W: Widget<T>> SimpleStyleWidget<T, W> {
    pub fn new(normal_style:Styler, hover_style:Option<Styler>, active_style:Option<Styler>, focus_style:Option<Styler>, inner: W) -> SimpleStyleWidget<T, W> {
		let padding = normal_style.get_padding();
		let margin = normal_style.get_margin();
		let font_size = normal_style.get_font_size();
		let width = normal_style.get_width();
		let height = normal_style.get_height();
		let text_color = normal_style.get_text_color();
		let background_color = normal_style.get_background_color();
		let border = normal_style.get_border();
		SimpleStyleWidget {
			normal_style,
			hover_style,
			active_style,
			focus_style,

			padding,
			margin,
			font_size,
			width,
			height,
			text_color,
			background_color,
			border,

			last_hover : false,
			last_focus : false,
			last_active : false,
			inner : WidgetPod::new(inner)
        }
    }
}


impl<T:Data, W: Widget<T>> Widget<T> for SimpleStyleWidget<T, W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.inner.event(ctx, event, data, env);
		
		let has_focus = self.inner.has_focus();
		let has_active = self.inner.has_active();
		let is_active = self.inner.is_active();
		let is_hover = self.inner.is_hot();
		
		match event {
			Event::AnimFrame(e) => {
				let result = self.normal_style.get_padding_with_anim(is_hover, e, self.padding, self.hover_style.map(|e| e.padding));
				if result.has_next_animation() {
					ctx.request_anim_frame();
				}

			}
			_ => {
				if self.last_hover != is_hover {
					ctx.request_anim_frame();
					self.last_hover = is_hover;
				}
			}
		}
		//check hot active focus
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
		//paint current status
        self.inner.paint(ctx, data, env);
    }
}