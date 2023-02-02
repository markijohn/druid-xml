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

enum Pseudo {
	Focus,
	Hover,
	Active
}

struct Style {
	padding : Option<Insets>,
    margin : Option<Insets>,
    font_size : Option<f64>,
    width : Option<f64>,
    height : Option<f64>,
    text_color : Option<Color>,
    background_color : Option<Color>,
    border : Option<BorderStyle>,
}

pub struct PseudoStyle {
	pseudo : Pseudo,
	style : Styler
}

impl PseudoStyle {
	pub fn hover(src:Styler) -> Self {
		Self {pseudo:Pseudo::Hover, style:src}
	}

	pub fn focus(src:Styler) -> Self {
		Self {pseudo:Pseudo::Hover, style:src}
	}

	pub fn active(src:Styler) -> Self {
		Self {pseudo:Pseudo::Hover, style:src}
	}
}

/// A StyleWidget for `hover` and `animation` effect
/// This widget changed `Env` and support `Padding` and `Conatainer`
/// Recommend pseudo class order is `focus` -> `hover` -> `active` but it's not mandatory
pub struct SimpleStyleWidget<T, W> {
	normal_style : Styler,
	pseduo_styles : [Option<PseudoStyle>;3],

	style : Style,

	last_hover : bool,
	last_focus : bool,
	last_active : bool,
	inner : WidgetPod<T,W>
}

impl<T, W: Widget<T>> SimpleStyleWidget<T, W> {
    pub fn new(normal_style:Styler, pseduo_styles:[Option<PseudoStyle>;3], inner: W) -> SimpleStyleWidget<T, W> {
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
			pseduo_styles,

			style : Style {
				padding,
				margin,
				font_size,
				width,
				height,
				text_color,
				background_color,
				border,
			},

			last_hover : false,
			last_focus : false,
			last_active : false,
			inner : WidgetPod::new(inner)
        }
    }


}

/// return (layout, paint, anim, ResultStyle)
fn check_style(e:i64, src:&mut Styler, target:&Styler) -> Option<(bool,bool,bool,Style)> {
	let mut need_layout = false;
	let mut need_paint = false;
	let mut need_anim = false;
	
	let target_padding = target.get_padding();
	let result = src.get_padding_with_anim( e, target_padding );
	let (has_next_anim,padding) = result.into();
	need_anim |= has_next_anim;
	
	if has_next_anim {
		need_anim = true;
	}
	if padding.is_some() {
		need_layout = true;
		need_paint = true;
	}

	//equal with padding
	let target_margin = target.get_margin();
	let result = src.get_margin_with_anim( e, target_margin );
	let (has_next_anim,margin) = result.into();
	need_anim |= has_next_anim;

	if has_next_anim {
		need_anim = true;
	}
	if margin.is_some() {
		need_layout = true;
		need_paint = true;
	}

	todo!()
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
				for ps in self.pseduo_styles.as_mut() {
					if let Some(ps) = ps {
						let elapsed = *e as i64;
						let is_neg = match ps.pseudo {
							Pseudo::Focus => has_focus,
							Pseudo::Hover => is_hover,
							Pseudo::Active => is_active,
						};
						let elapsed = if is_neg {
							-elapsed
						} else {
							elapsed
						};

						if let Some( (refresh_layout, repaint, anim, changed_style) ) = check_style(elapsed, &mut self.normal_style, &ps.style) {
							if refresh_layout {
								ctx.request_layout();
							}
							if repaint {
								ctx.request_paint();
							}
							if anim {
								ctx.request_anim_frame();
							}
							self.style = changed_style;
							break;
						}
					}
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