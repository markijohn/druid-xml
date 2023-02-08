use crate::simple_style::{Styler, BorderStyle, Style};
use druid::kurbo::{Insets, Point, Rect, Size, RoundedRect};
use druid::widget::Axis;
use druid::{BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    RenderContext, UpdateCtx, Widget, WidgetPod, MouseButton};
use super::theme;

enum Pseudo {
	Focus,
	Hover,
	Active,
	Disabled
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
		Self {pseudo:Pseudo::Focus, style:src}
	}

	pub fn active(src:Styler) -> Self {
		Self {pseudo:Pseudo::Active, style:src}
	}

	pub fn disabled(src:Styler) -> Self {
		Self {pseudo:Pseudo::Disabled, style:src}
	}
}

/// A StyleWidget for `hover` and `animation` effect
/// This widget changed `Env` and support `Padding` and `Conatainer`
/// Recommend pseudo class order is `focus` -> `hover` -> `active` but it's not mandatory
/// Forced order is 'normal(none pseudo)' -> disabled -> {user defined styles..}
pub struct SimpleStyleWidget<T,W> {
	normal_style : Styler,
	styles : [Option<PseudoStyle>;5],
	style_updated : u64,
	style : Style,
	last_point : Point,
	inner_size : Rect,
	inner : WidgetPod<T,W>
}

impl<T,W:Widget<T>> SimpleStyleWidget<T,W> {
    pub fn new(normal_style:Styler, styles:[Option<PseudoStyle>;5], inner: W) -> SimpleStyleWidget<T,W> {
		let padding = normal_style.get_padding().unwrap_or_default();
		let margin = normal_style.get_margin().unwrap_or_default();
		let font_size = normal_style.get_font_size().unwrap_or( 14. );
		let width = normal_style.get_width();
		let height = normal_style.get_height();
		let text_color = normal_style.get_text_color().unwrap_or( Color::rgba8(0, 0, 0, 255) );
		let background_color = normal_style.get_background_color().unwrap_or( Color::rgba8(0, 0, 0, 0) );
		let border = normal_style.get_border().unwrap_or_default();
		SimpleStyleWidget {
			normal_style,
			styles,
			style_updated : theme::STYLE_UPDATED_NONE,
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
			last_point : Point::default(),
			inner_size : Rect::new(0., 0., 0., 0.),
			inner : WidgetPod::new(inner)
        }
    }


}

/// return (layout, paint, anim, ResultStyle)
// fn check_style(e:i64, src:&mut Styler, target:&Styler) -> Option<(bool,bool,bool,Style)> {
// 	let mut need_anim = false;
	
// 	//padding
// 	let target_padding = target.get_padding();
// 	let result = src.get_padding_with_anim( e, target_padding );
// 	let (has_next_anim,padding) = result.into();
// 	need_anim |= has_next_anim;

// 	//margin. same procedure padding
// 	let target_margin = target.get_margin();
// 	let result = src.get_margin_with_anim( e, target_margin );
// 	let (has_next_anim,margin) = result.into();
// 	need_anim |= has_next_anim;

// 	//font-size
// 	let target_font_size = target.get_font_size();
// 	let result = src.get_font_size_with_anim( e, target_font_size );
// 	let (has_next_anim,font_size) = result.into();
// 	need_anim |= has_next_anim;

// 	//width
// 	let target_width = target.get_width();
// 	let result = src.get_width_with_anim( e, target_width );
// 	let (has_next_anim,width) = result.into();
// 	need_anim |= has_next_anim;

// 	//height
// 	let target_height = target.get_height();
// 	let result = src.get_height_with_anim( e, target_height );
// 	let (has_next_anim,height) = result.into();
// 	need_anim |= has_next_anim;

// 	//text-color
// 	let target_text_color = target.get_text_color();
// 	let result = src.get_text_color_with_anim( e, target_text_color );
// 	let (has_next_anim,text_color) = result.into();
// 	need_anim |= has_next_anim;

// 	//background-color
// 	let target_background_color = target.get_background_color();
// 	let result = src.get_background_color_with_anim( e, target_background_color );
// 	let (has_next_anim,background_color) = result.into();
// 	need_anim |= has_next_anim;
	
// 	//border
// 	let target_border = target.get_border();
// 	let result = src.get_border_with_anim( e, target_border );
// 	let (has_next_anim,border) = result.into();
// 	need_anim |= has_next_anim;

// 	let need_layout = 
// 	padding.is_some() 
// 	| margin.is_some()
// 	| font_size.is_some()
// 	| width.is_some()
// 	| height.is_some();

// 	let need_paint = 
// 	need_layout
// 	| text_color.is_some()
// 	| background_color.is_some()
// 	| border.is_some();
	

// 	if need_layout | need_paint {
// 		Some( (need_layout,need_paint,need_anim, Style {
// 			padding,
// 			margin,
// 			font_size,
// 			width,
// 			height,
// 			text_color,
// 			background_color,
// 			border,
// 		} ) )
// 	} else {
// 		None
// 	}
// }

fn wrapped_padding_env(env:&Env, style_updated:u64, style:&Style) -> Env {
	let mut wrapped_env = env.clone();
	wrapped_env.set( theme::STYLE_UPDATED, style_updated );
	wrapped_env.set( theme::PADDING, style.padding );
	wrapped_env.set( theme::FONT_SIZE, style.font_size );
	wrapped_env.set( theme::COLOR, style.text_color );
	wrapped_env
}

impl<T:Data, W:Widget<T>> Widget<T> for SimpleStyleWidget<T,W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
		
		let is_inner = self.inner_size.contains(self.last_point);
		if is_inner {
			//Do not call WidgetPod.event
			self.inner.widget_mut().event(ctx, event, data, env);
		}
		match event {
			Event::MouseMove(e) => {
				self.last_point = e.pos;
			}
			Event::MouseDown(e) => {
				if e.button == MouseButton::Left && !ctx.is_disabled() {
                    ctx.set_active(true);
                }
				self.last_point = e.pos;
			}
			Event::MouseUp(e) => {
				if ctx.is_active() && e.button == MouseButton::Left {
					ctx.set_active(false);
                }
				self.last_point = e.pos;
			}
			Event::Wheel(e) => {
				self.last_point = e.pos;
			}
			_ => ()
		};
		
		let has_focus = ctx.has_focus() && is_inner;
		let is_hover = ctx.is_hot()  && is_inner;
		let is_active = ctx.is_active();
		let is_disabled = ctx.is_disabled();
		
		match event {
			Event::AnimFrame(e) => {
				for ps in self.styles.as_mut() {
					if let Some(ps) = ps {
						let neg = match ps.pseudo {
							Pseudo::Normal => true,
							Pseudo::Focus => !has_focus,
							Pseudo::Hover => !is_hover,
							Pseudo::Active => !is_active,
							Pseudo::Disabled => !is_disabled,
						};

						let elapsed = if neg {
							*e as i64 * -1
						} else {
							*e as i64
						};

						let (request_layout, request_paint, request_anim) = self.style.transit( elapsed, ps.style );
						if request_layout {
							ctx.request_layout();
						}
						if request_paint {
							ctx.request_paint();
						}
						if request_anim {
							ctx.request_anim_frame();
						}
					}
				}
			}
			_ => {
				if event.is_pointer_event() {
					ctx.request_anim_frame();
				}
			}
		}
		
		
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
		let mut wrapped_env = env.clone();
		wrapped_env.set( super::theme::PADDING, self.style.padding );
		let env = &wrapped_env;
		let (mt,mr,mb,ml) = (self.style.margin.y0, self.style.margin.x1, self.style.margin.y1, self.style.margin.x0);
		let child_bc = bc.shrink( (ml+mr, mt+mb) );
		let size = self.inner.layout(ctx, &child_bc, data, &wrapped_padding_env(env, self.style_updated,&self.style));
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
		let (mt,mr,mb,ml) = (self.style.margin.y0, self.style.margin.x1, self.style.margin.y1, self.style.margin.x0);
		let size = ctx.size();

		if self.style.border.width > 0. {
			let rr = RoundedRect::new(ml, mt, size.width, size.height, self.style.border.radius);
			ctx.fill(rr, &self.style.background_color);
			ctx.stroke_styled(rr, &self.style.border.color, self.style.border.width, &druid::piet::StrokeStyle::default());
		} else {
			let r = Rect::new(ml, mt, size.width-mr, size.height-mb);
			ctx.fill(r, &self.style.background_color);
		}

        self.inner.paint(ctx, data, &wrapped_padding_env(env, self.style_updated,&self.style));
    }

	fn compute_max_intrinsic(
        &mut self,
        axis: Axis,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> f64 {
		let margin = self.style.margin;

		let child_bc = bc.shrink( (margin.x0+margin.x1, margin.y0+margin.y1) );
        self
            .inner
			.widget_mut()
            .compute_max_intrinsic(axis, ctx, &child_bc, data, env)
    }
}
