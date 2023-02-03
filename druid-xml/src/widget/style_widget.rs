use std::collections::VecDeque;
use std::rc::Rc;

use crate::simple_style::{Animation, Styler, BorderStyle};
use druid::commands::SCROLL_TO_VIEW;
use druid::kurbo::{Affine, Insets, Point, Rect, Shape, Size, RoundedRect};
use druid::widget::Axis;
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
pub struct SimpleStyleWidget<T> {
	normal_style : Styler,
	pseduo_styles : [Option<PseudoStyle>;3],

	style : Style,

	last_hover : bool,
	last_focus : bool,
	last_active : bool,
	//inner : WidgetPod<T,W>
	inner : Box<dyn Widget<T>>
}

impl<T> SimpleStyleWidget<T> {
    pub fn new<W:Widget<T>>(normal_style:Styler, pseduo_styles:[Option<PseudoStyle>;3], inner: W) -> SimpleStyleWidget<T> {
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
			//inner : WidgetPod::new(inner)
			inner : Box::new(inner)
        }
    }


}

/// return (layout, paint, anim, ResultStyle)
fn check_style(e:i64, src:&mut Styler, target:&Styler) -> Option<(bool,bool,bool,Style)> {
	let mut need_layout = false;
	let mut need_paint = false;
	let mut need_anim = false;
	
	//padding
	let target_padding = target.get_padding();
	let result = src.get_padding_with_anim( e, target_padding );
	let (has_next_anim,padding) = result.into();
	need_anim |= has_next_anim;
	if padding.is_some() {
		need_layout = true;
		need_paint = true;
	}

	//margin. same procedure padding
	let target_margin = target.get_margin();
	let result = src.get_margin_with_anim( e, target_margin );
	let (has_next_anim,margin) = result.into();
	need_anim |= has_next_anim;
	if margin.is_some() {
		need_layout = true;
		need_paint = true;
	}

	//font-size
	let target_font_size = target.get_font_size();
	let result = src.get_font_size_with_anim( e, target_font_size );
	let (has_next_anim,font_size) = result.into();
	need_anim |= has_next_anim;
	if font_size.is_some() {
		need_layout = true;
		need_paint = true;
	}

	//width
	let target_width = target.get_width();
	let result = src.get_width_with_anim( e, target_width );
	let (has_next_anim,width) = result.into();
	need_anim |= has_next_anim;
	if width.is_some() {
		need_layout = true;
		need_paint = true;
	}

	//height
	let target_height = target.get_height();
	let result = src.get_height_with_anim( e, target_height );
	let (has_next_anim,height) = result.into();
	need_anim |= has_next_anim;
	if height.is_some() {
		need_layout = true;
		need_paint = true;
	}

	//text-color
	let target_text_color = target.get_text_color();
	let result = src.get_text_color_with_anim( e, target_text_color );
	let (has_next_anim,text_color) = result.into();
	need_anim |= has_next_anim;
	if text_color.is_some() {
		need_paint = true;
	}

	//background-color
	let target_background_color = target.get_background_color();
	let result = src.get_background_color_with_anim( e, target_background_color );
	let (has_next_anim,background_color) = result.into();
	need_anim |= has_next_anim;
	if background_color.is_some() {
		need_paint = true;
	}
	
	//border
	let target_border = target.get_border();
	let result = src.get_border_with_anim( e, target_border );
	let (has_next_anim,border) = result.into();
	need_anim |= has_next_anim;
	if border.is_some() {
		need_paint = true;
	}

	if need_layout | need_paint {
		Some( (need_layout,need_paint,need_anim, Style {
			padding,
			margin,
			font_size,
			width,
			height,
			text_color,
			background_color,
			border,
		} ) )
	} else {
		None
	}
}

impl<T:Data> Widget<T> for SimpleStyleWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
		
        //self.inner.event(ctx, event, data, env);
		
		
		let has_focus = ctx.has_focus();
		let is_active = ctx.is_active();
		let is_hover = ctx.is_hot();

		if self.last_hover != is_hover {
			ctx.request_anim_frame();
			self.last_hover = is_hover;
			return;
		}

		if self.last_focus != has_focus {
			ctx.request_anim_frame();
			self.last_focus = has_focus;
		}

		if self.last_active != is_active {
			ctx.request_anim_frame();
			self.last_active = is_active;
		}
		
		match event {
			Event::AnimFrame(e) => {
				for ps in self.pseduo_styles.as_mut() {
					if let Some(ps) = ps {
						let elapsed = (*e as i64)+1;
						let is_neg = match ps.pseudo {
							Pseudo::Focus => !has_focus,
							Pseudo::Hover => !is_hover,
							Pseudo::Active => !is_active,
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
			_ => ()
		}
		//check hot active focus
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        ///self.inner.update(ctx, data, env);
		self.inner.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
		let (pt,pr,pb,pl) = if let Some(padding) = self.style.padding {
			(padding.y0, padding.x1, padding.y1, padding.x0)
		} else {
			(0., 0., 0., 0.)
		};
		let (mt,mr,mb,ml) = if let Some(margin) = self.style.margin {
			(margin.y0, margin.x1, margin.y1, margin.x0)
		} else {
			(0., 0., 0., 0.)
		};
		let child_bc = bc.shrink( (pl+pr+ml+mr, pt+pb+mt+mb) );
		let size = self.inner.layout(ctx, &child_bc, data, env);
		let origin = Point::new(pl+ml, pt+mt);
		//let origin = Point::new(ml, mt);
		self.inner.set_origin(ctx, origin);

		let my_size = Size::new(
			size.width + pl+pr+ml+mr,
			size.height + pt+pb+mt+mb
		);
		let my_insets = self.inner.compute_parent_paint_insets(my_size);
        ctx.set_paint_insets(my_insets);
        let baseline_offset = self.inner.baseline_offset();
        if baseline_offset > 0f64 {
            ctx.set_baseline_offset(baseline_offset + pt+mt);
        }

		my_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
		let (mt,mr,mb,ml) = if let Some(margin) = self.style.margin {
			(margin.y0, margin.x1, margin.y1, margin.x0)
		} else {
			(0., 0., 0., 0.)
		};
		let size = ctx.size();

		if let Some(border) = &self.style.border {
			let rr = RoundedRect::new(ml, mt, size.width, size.height, border.radius);
			if let Some(background_color) = self.style.background_color {
				ctx.fill(rr, &background_color);
			}
			ctx.stroke_styled(rr, &border.color, border.width, &border.style);
		} else {
			if let Some(background_color) = self.style.background_color {
				let r = Rect::new(ml, mt, size.width-mr, size.height-mb);
				ctx.fill(r, &background_color);
			}
		}
        self.inner.paint(ctx, data, env);
    }

	fn compute_max_intrinsic(
        &mut self,
        axis: Axis,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> f64 {
		let (pt,pr,pb,pl) = if let Some(padding) = self.style.padding {
			(padding.y0, padding.x1, padding.y1, padding.x0)
		} else {
			(0., 0., 0., 0.)
		};
		let (mt,mr,mb,ml) = if let Some(margin) = self.style.margin {
			(margin.y0, margin.x1, margin.y1, margin.x0)
		} else {
			(0., 0., 0., 0.)
		};

		let child_bc = bc.shrink( (pl+pr+ml+mr, pt+pb+mt+mb) );
        self
            .inner
            .widget_mut()
            .compute_max_intrinsic(axis, ctx, &child_bc, data, env)
    }
}