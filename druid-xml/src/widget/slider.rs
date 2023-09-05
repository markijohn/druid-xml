use std::ops::{DerefMut, Deref};

///Label wrapper for dynamic style(text-size, color)

use druid::{widget::{Label, LabelText, Axis, LineBreaking, Checkbox, Slider}, Widget, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, LayoutCtx, BoxConstraints, PaintCtx, UpdateCtx, Size};
use serde_json::{Value, Number};

use crate::qwidget::value::JSValue;
use super::theme;

pub struct DXSlider {
	val : f64,
    origin : Slider
}

impl DXSlider {
    pub fn new() -> Self {
        Self {
			val : 0.,
            origin : Slider::new()
        }
    }

    pub fn with_range(mut self, min:f64, max:f64) -> Self {
        self.origin = self.origin.with_range(min, max);
        self
    }
}


impl Widget<JSValue> for DXSlider {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, data: &mut JSValue, _env: &Env) {
        self.origin.event(_ctx, _event, &mut self.val, _env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &JSValue, env: &Env) {
		self.origin.lifecycle(ctx, event, &self.val, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &JSValue, data: &JSValue, env: &Env) {
        if let ( Value::Number(old), Value::Number(new) ) = (_old_data.deref(), data.deref()) {
            self.origin.update(ctx, &old.as_f64().unwrap(), &new.as_f64().unwrap(), env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &JSValue, env: &Env) -> Size {
        if let Value::Number(_val) = _data.deref() {
            self.origin.layout(ctx, bc, &_val.as_f64().unwrap(), env)    
        } else {
            bc.max()
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &JSValue, env: &Env) {
        if let Value::Number(val) = _data.deref() {
            self.origin.paint(ctx, &val.as_f64().unwrap(), env)
        }
    }

    fn compute_max_intrinsic(
        &mut self,
        axis: Axis,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &JSValue,
        env: &Env,
    ) -> f64 {
        if let Value::Number(val) = _data.deref() {
            self.origin.compute_max_intrinsic(axis, ctx, bc, &val.as_f64().unwrap(), env)
        } else {
            0.
        }
    }
}