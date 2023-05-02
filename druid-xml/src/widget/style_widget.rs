use crate::simple_style::{Styler, Style, PseudoStyle, Pseudo};
use druid::kurbo::{Point, Rect, Size, RoundedRect};
use druid::widget::Axis;
use druid::{BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    RenderContext, UpdateCtx, Widget, WidgetPod, MouseButton};
use super::theme;

/// A StyleWidget for `hover` and `animation` effect
/// This widget changed `Env` and support `Padding` and `Conatainer`
/// Recommend pseudo class order is `focus` -> `hover` -> `active` but it's not mandatory
/// Styles declared later take precedence. (like CSS)
pub struct SimpleStyleWidget<T,W> {
	normal_style : Styler,
	styles : [Option<PseudoStyle>;4],

	last_focus : bool,
	last_hover : bool,
	last_active : bool,
	last_disabled : bool,

	has_focus_style : bool,
	has_hover_style : bool,
	has_active_style : bool,
	has_disabled_style : bool,

	style_updated : u64,
	start_style : Style,
	end_style : Style,
	base_style : Style,
	curr_style : Style,
	last_point : Point,
	inner_size : Rect,
	inner : WidgetPod<T,W>
}

impl<T,W:Widget<T>> SimpleStyleWidget<T,W> {
    pub fn new(normal_style:Styler, styles:[Option<PseudoStyle>;4], inner: W) -> SimpleStyleWidget<T,W> {
		let padding = normal_style.get_padding().unwrap_or_default();
		let margin = normal_style.get_margin().unwrap_or_default();
		let font_size = normal_style.get_font_size().unwrap_or( theme::DEFAULT_FONT_SIZE );
		let width = normal_style.get_width();
		let height = normal_style.get_height();
		let text_color = normal_style.get_text_color().unwrap_or( theme::DEFAULT_TEXT_COLOR );
		let background_color = normal_style.get_background_color().unwrap_or( theme::DEFAULT_BACKGROUND_COLOR );
		let border = normal_style.get_border().unwrap_or( theme::default_border() );
		let start_style = Style {
			padding,
			margin,
			font_size,
			width,
			height,
			text_color,
			background_color,
			border,
		};
		let end_style = start_style.clone();
		let base_style = start_style.clone();
		let curr_style = start_style.clone();

		let mut has_focus_style = false;
		let mut has_hover_style = false;
		let mut has_active_style = false;
		let mut has_disabled_style = false;
		for s in styles.iter() {
			if let Some(s) = s {
				match s.pseudo {
					Pseudo::Focus => has_focus_style = true,
					Pseudo::Hover => has_hover_style = true,
					Pseudo::Active => has_active_style = true,
					Pseudo::Disabled => has_disabled_style = true,
    			}
			}
		}

		SimpleStyleWidget {
			normal_style,
			styles,
			last_focus : false,
			last_hover : false,
			last_active : false,
			last_disabled : false,
			has_focus_style,
			has_hover_style,
			has_active_style,
			has_disabled_style,
			style_updated : theme::STYLE_UPDATED_LAYOUT,
			start_style,
			end_style,
			base_style,
			curr_style,
			last_point : Point::default(),
			inner_size : Rect::new(0., 0., 0., 0.),
			inner : WidgetPod::new(inner)
        }
    }


}

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
		
		let mut on_style_trigger = false;
		let has_focus = ctx.has_focus() && is_inner;
		let is_hover = ctx.is_hot()  && is_inner;
		let is_active = ctx.is_active();
		let is_disabled = ctx.is_disabled();
		let has_state = has_focus | is_hover | is_active | is_disabled;

		if self.has_focus_style && self.last_focus != has_focus {
			self.last_focus = has_focus;
			on_style_trigger = true;
		}

		if self.has_hover_style && self.last_hover != is_hover {
			self.last_hover = is_hover;
			on_style_trigger = true;
		}

		if self.has_active_style && self.last_active != is_active {
			self.last_active = is_active;
			on_style_trigger = true;
		}

		if self.has_disabled_style && self.last_disabled != is_hover {
			self.last_disabled = is_disabled;
			on_style_trigger = true;
		}

		if on_style_trigger {
			ctx.request_anim_frame();

			//set start style to goal style
			self.start_style = self.end_style.clone();
			self.base_style = self.curr_style.clone();

			
			if has_state {
				for e in self.styles.iter_mut() {
					if let Some( e) = e.as_mut() {
						let target = match e.pseudo {
							Pseudo::Focus => has_focus,
							Pseudo::Hover => is_hover,
							Pseudo::Active => is_active,
							Pseudo::Disabled => is_disabled,
						};
						if target {
							e.style.set_state_from_style(&self.start_style, &self.end_style, &self.base_style);
						}
					}
				}
			}

			//make new target style
			self.end_style = self.normal_style.composite_styles( self.styles.iter_mut()
				.filter( | e|
					if let Some( e) = e {
						match e.pseudo {
							Pseudo::Focus => has_focus,
							Pseudo::Hover => is_hover,
							Pseudo::Active => is_active,
							Pseudo::Disabled => is_disabled,
						}
					} else {
						false
					}
				)
				.map( |e| &e.as_ref().unwrap().style )
				.filter( |_e| {
					true
				})
			);

			//analysis progress state
			self.normal_style.set_state_from_style(&self.start_style, &self.end_style, &self.base_style);
			// println!("Start Style {:#?}", self.start_style);
			// println!("End Style {:#?}", self.end_style);
		}
		
		
		match event {
			Event::AnimFrame(e) => {
				let (mut request_layout, mut request_paint, mut request_anim) = 
				//self.base_style.transit(*e as _, &self.end_style, &mut self.normal_style, None, &mut self.curr_style);
				self.start_style.transit(*e as _, &self.end_style, &mut self.normal_style, None, &mut self.curr_style);
				if has_state {
					for ps in self.styles.as_mut() {
						if let Some(ps) = ps {
							let matched = match ps.pseudo {
								Pseudo::Focus => has_focus,
								Pseudo::Hover => is_hover,
								Pseudo::Active => is_active,
								Pseudo::Disabled => is_disabled,
							};
	
							if !matched {
								continue
							}
	
							//TODO : optimization
							//There are several optimization points. 
							//If the transition of the styler has been performed while performing the iteration backwards, 
							//the rest does not need to be performed. But currently it does all the transition processing while doing forward iteration.
							//let result = self.base_style.transit(*e as _, &self.end_style, &mut ps.style, Some(&mut self.normal_style), &mut self.curr_style);
							let result = self.start_style.transit(*e as _, &self.end_style, &mut ps.style, Some(&mut self.normal_style), &mut self.curr_style);
							request_layout |= result.0;
							request_paint |= result.1;
							request_anim |= result.2;
							// println!("{:?} layout:{} paint:{} anim:{}", ps.pseudo, request_layout, request_paint, request_anim);
						}
					}
					
				}
				
				
				if request_layout {
					ctx.request_layout();
				}
				if request_paint {
					ctx.request_paint();
				}
				if request_anim || *e == 0 {
					ctx.request_anim_frame();
				}
			}
			_ => ()
		}
		
		
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
		let mut wrapped_env = env.clone();
		wrapped_env.set( super::theme::PADDING, self.curr_style.padding );
		let env = &wrapped_env;
		let (mt,mr,mb,ml) = (self.curr_style.margin.y0, self.curr_style.margin.x1, self.curr_style.margin.y1, self.curr_style.margin.x0);
		let child_bc = bc.shrink( (ml+mr, mt+mb) );
		let size = self.inner.layout(ctx, &child_bc, data, &wrapped_padding_env(env, self.style_updated,&self.curr_style));
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
		let (mt,mr,mb,ml) = (self.curr_style.margin.y0, self.curr_style.margin.x1, self.curr_style.margin.y1, self.curr_style.margin.x0);
		let size = ctx.size();
		if self.curr_style.border.width > 0. {
			let inner_grow = self.curr_style.border.width - 1.;
			let rr = RoundedRect::new(ml+inner_grow, mt+inner_grow, size.width-inner_grow, size.height-inner_grow, self.curr_style.border.radius);
			ctx.fill(rr, &self.curr_style.background_color);
			ctx.stroke_styled(rr, &self.curr_style.border.color, self.curr_style.border.width, &druid::piet::StrokeStyle::default());
		} else {
			let r = Rect::new(ml, mt, size.width-mr, size.height-mb);
			ctx.fill(r, &self.curr_style.background_color);
		}

        self.inner.paint(ctx, data, &wrapped_padding_env(env, self.style_updated,&self.curr_style));
    }

	fn compute_max_intrinsic(
        &mut self,
        axis: Axis,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> f64 {
		let margin = self.curr_style.margin;

		let child_bc = bc.shrink( (margin.x0+margin.x1, margin.y0+margin.y1) );
        self
            .inner
			.widget_mut()
            .compute_max_intrinsic(axis, ctx, &child_bc, data, env)
    }
}
