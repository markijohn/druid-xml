///Label wrapper for dynamic style(text-size, color)

use druid::{widget::{Label, LabelText, Axis, LineBreaking}, Data, Widget, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, LayoutCtx, BoxConstraints, PaintCtx, UpdateCtx, Size};

use super::theme;

pub struct DXLabel<T> {
    origin : Label<T>
}

impl <T:Data> DXLabel<T> {
    pub fn new(text: impl Into<LabelText<T>>) -> Self {
        Self {
            origin : Label::new(text)
        }
    }

    pub fn set_line_break_mode(&mut self, mode:LineBreaking ) {
        self.origin.set_line_break_mode(mode);
    }
}


impl<T: Data> Widget<T> for DXLabel<T> {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.origin.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.origin.update(ctx, _old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, env: &Env) -> Size {
        if env.get(theme::STYLE_UPDATED) == theme::STYLE_UPDATED_LAYOUT {
            self.origin.set_text_size(env.get(theme::FONT_SIZE));
            self.origin.set_text_color(env.get(theme::COLOR));
        }
        self.origin.layout(ctx, bc, _data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        if env.get(theme::STYLE_UPDATED) == theme::STYLE_UPDATED_PAINT {
            self.origin.set_text_color(env.get(theme::COLOR));
        }
        self.origin.paint(ctx, _data, env)
    }

    fn compute_max_intrinsic(
        &mut self,
        axis: Axis,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &T,
        env: &Env,
    ) -> f64 {
        self.origin.compute_max_intrinsic(axis, ctx, bc, _data, env)
    }
}