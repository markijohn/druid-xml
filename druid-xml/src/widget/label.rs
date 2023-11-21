use std::ops::{DerefMut, Deref};

///Label wrapper for dynamic style(text-size, color)

use druid::{widget::{Label, LabelText, Axis, LineBreaking}, Data, Widget, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, LayoutCtx, BoxConstraints, PaintCtx, UpdateCtx, Size, ArcStr};
use serde_json::Value;

use crate::qwidget::value::JSValue;
use super::theme;

pub struct DXLabel {
    text : String,
    origin : Label<String>
}

impl DXLabel {
    pub fn new(text: impl Into<LabelText<String>>) -> Self {
        Self {
            text : "Text...".to_string(),
            origin : Label::new(text)
        }
    }

    pub fn set_line_break_mode(&mut self, mode:LineBreaking ) {
        self.origin.set_line_break_mode(mode);
    }
}


impl Widget<JSValue> for DXLabel {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, data: &mut JSValue, _env: &Env) {
        // if let Value::String(val) = data.deref_mut() {
            self.origin.event(_ctx, _event, &mut self.text, _env);
        // }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &JSValue, env: &Env) {
        // if let Value::String(val) = data.deref() {
            self.origin.lifecycle(ctx, event, &self.text, env);
        // }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &JSValue, data: &JSValue, env: &Env) {
        // if let ( Value::String(old), Value::String(new) ) = (_old_data.deref(), data.deref()) {
            self.origin.update(ctx, &"OldData".to_owned(), &self.text, env);
        // }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &JSValue, env: &Env) -> Size {
        // if let Value::String(_val) = _data.deref() {
            if env.get(theme::STYLE_UPDATED) == theme::STYLE_UPDATED_LAYOUT {
                self.origin.set_text_size(env.get(theme::FONT_SIZE));
                self.origin.set_text_color(env.get(theme::COLOR));
            }
            self.origin.layout(ctx, bc, &self.text, env)    
        // } else {
        //     bc.max()
        // }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &JSValue, env: &Env) {
        // if let Value::String(val) = _data.deref() {
            if env.get(theme::STYLE_UPDATED) == theme::STYLE_UPDATED_PAINT {
                self.origin.set_text_color(env.get(theme::COLOR));
            }
            self.origin.paint(ctx, &self.text, env)
        // }
    }

    fn compute_max_intrinsic(
        &mut self,
        axis: Axis,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &JSValue,
        env: &Env,
    ) -> f64 {
        if let Value::String(val) = _data.deref() {
            self.origin.compute_max_intrinsic(axis, ctx, bc, val, env)
        } else {
            0.
        }
    }
}