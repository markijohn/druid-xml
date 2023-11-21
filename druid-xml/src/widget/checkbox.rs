use std::ops::{DerefMut, Deref};

///Label wrapper for dynamic style(text-size, color)

use druid::{widget::{Label, LabelText, Axis, LineBreaking, Checkbox}, Widget, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, LayoutCtx, BoxConstraints, PaintCtx, UpdateCtx, Size};
use serde_json::Value;

use crate::qwidget::value::JSValue;
use super::theme;

pub struct DXCheckbox {
    value : bool,
    origin : Checkbox
}

impl DXCheckbox {
    pub fn new(text: &str) -> Self {
        Self {
            value : false,
            origin : Checkbox::new( LabelText::from(text) )
        }
    }
}


impl Widget<JSValue> for DXCheckbox {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, data: &mut JSValue, _env: &Env) {
        self.origin.event(_ctx, _event, &mut self.value, _env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &JSValue, env: &Env) {
        self.origin.lifecycle(ctx, event, &self.value, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &JSValue, data: &JSValue, env: &Env) {
        self.origin.update(ctx, &false, &self.value, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &JSValue, env: &Env) -> Size {
        if let Value::Bool(_val) = _data.deref() {
            self.origin.layout(ctx, bc, _val, env)    
        } else {
            bc.max()
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &JSValue, env: &Env) {
        self.origin.paint(ctx, &self.value, env)
    }

    fn compute_max_intrinsic(
        &mut self,
        axis: Axis,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &JSValue,
        env: &Env,
    ) -> f64 {
        if let Value::Bool(val) = _data.deref() {
            self.origin.compute_max_intrinsic(axis, ctx, bc, val, env)
        } else {
            0.
        }
    }
}