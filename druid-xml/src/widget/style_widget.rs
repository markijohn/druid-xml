use crate::simple_style::{Styler, BorderStyle};
use druid::kurbo::{Insets, Point, Rect, Size, RoundedRect};
use druid::widget::Axis;
use druid::{BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    RenderContext, UpdateCtx, Widget, WidgetPod,
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
pub struct SimpleStyleWidget<T,W> {
	normal_style : Styler,
	pseduo_styles : [Option<PseudoStyle>;3],

	style : Style,

	last_point : Point,
	last_hover : bool,
	last_focus : bool,
	last_active : bool,
	inner_size : Rect,
	inner : WidgetPod<T,W>
}

impl<T,W:Widget<T>> SimpleStyleWidget<T,W> {
    pub fn new(normal_style:Styler, pseduo_styles:[Option<PseudoStyle>;3], inner: W) -> SimpleStyleWidget<T,W> {
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

			last_point : Point::new(0., 0.),
			last_hover : false,
			last_focus : false,
			last_active : false,
			inner_size : Rect::new(0., 0., 0., 0.),
			inner : WidgetPod::new(inner)
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

fn wrapped_padding_env(env:&Env, style:&Style) -> Env {
	let mut wrapped_env = env.clone();
	wrapped_env.set( super::theme::PADDING, style.padding.unwrap_or_default());
	wrapped_env.set( super::theme::FONT_SIZE, style.font_size.unwrap_or(14.));
	wrapped_env.set( super::theme::COLOR, style.text_color.unwrap_or(Color::rgb8(255,255,255)));
	wrapped_env
}

impl<T:Data, W:Widget<T>> Widget<T> for SimpleStyleWidget<T,W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
		match event {
			Event::MouseMove(e) => {
				self.last_point = e.pos;
			}
			Event::MouseDown(e) => {
				self.last_point = e.pos;
			}
			Event::MouseUp(e) => {
				self.last_point = e.pos;
			}
			Event::Wheel(e) => {
				self.last_point = e.pos;
			}
			_ => ()
		};

		let is_inner = self.inner_size.contains(self.last_point);
		let has_focus = ctx.has_focus() && is_inner;
		let is_active = ctx.is_active() && is_inner;
		let is_hover = ctx.is_hot()  && is_inner;

		if self.last_hover != is_hover {
			self.last_hover = is_hover;
			ctx.request_anim_frame();
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
		
		
		self.inner.event(ctx, event, data, &wrapped_padding_env(env, &self.style));
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, &wrapped_padding_env(env, &self.style));
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, data, &wrapped_padding_env(env, &self.style));
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
		let mut wrapped_env = env.clone();
		if let Some(padding) = self.style.padding {
			wrapped_env.set( super::theme::PADDING, padding );
		} else {
			wrapped_env.set( super::theme::PADDING, Insets::default() );
		};
		let env = &wrapped_env;
		let (mt,mr,mb,ml) = if let Some(margin) = self.style.margin {
			(margin.y0, margin.x1, margin.y1, margin.x0)
		} else {
			(0., 0., 0., 0.)
		};
		let child_bc = bc.shrink( (ml+mr, mt+mb) );
		let size = self.inner.layout(ctx, &child_bc, data, &wrapped_padding_env(env, &self.style));
		let origin = Point::new(ml, mt);
		self.inner.set_origin(ctx, origin);

		let my_size = Size::new(
			size.width + ml+mr,
			size.height + mt+mb
		);
		self.inner_size = Rect::new(ml, mt, my_size.width-mr, my_size.height-mb);
		let my_insets = self.inner.compute_parent_paint_insets(my_size);
        ctx.set_paint_insets(my_insets);
        let baseline_offset = self.inner.baseline_offset();
        if baseline_offset > 0f64 {
            ctx.set_baseline_offset(baseline_offset + mt);
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
        self.inner.paint(ctx, data, &wrapped_padding_env(env, &self.style));
    }

	fn compute_max_intrinsic(
        &mut self,
        axis: Axis,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> f64 {
		let (mt,mr,mb,ml) = if let Some(margin) = self.style.margin {
			(margin.y0, margin.x1, margin.y1, margin.x0)
		} else {
			(0., 0., 0., 0.)
		};

		let child_bc = bc.shrink( (ml+mr, mt+mb) );
        self
            .inner
			.widget_mut()
            .compute_max_intrinsic(axis, ctx, &child_bc, data, env)
    }
}
