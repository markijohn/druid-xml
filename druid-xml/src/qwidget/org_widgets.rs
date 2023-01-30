use std::ops::{DerefMut, Deref};

use druid::Widget;

use druid::widget::{Label,TextBox};
use serde_json::Value;

use super::value::JSValue;

impl Widget<JSValue> for Label<String> {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut JSValue, env: &druid::Env) {
		if let Value::String(s) = data.deref_mut() {
			Label::<String>::event(self, ctx, event, s, env);
		}
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &JSValue, env: &druid::Env) {
		if let Value::String(s) = data.deref() {
			Label::<String>::lifecycle(self, ctx, event, s, env);
		}
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &JSValue, data: &JSValue, env: &druid::Env) {
        if let (Value::String(s1), Value::String(s2)) = (old_data.deref(),data.deref()) {
			Label::<String>::update(self, ctx, old_data, data, env);
		}
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &JSValue, env: &druid::Env) -> druid::Size {
		if let Value::String(s) = data.deref() {
			Label::<String>::layout(self, ctx, bc, s, env)
		} else {
			bc.max()
		}
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &JSValue, env: &druid::Env) {
		if let Value::String(s) = data.deref() {
			Label::<String>::paint(self, ctx,  s, env)
		}
    }
}


impl Widget<JSValue> for TextBox<String> {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut JSValue, env: &druid::Env) {
		if let Value::String(s) = data.deref_mut() {
			TextBox::<String>::event(self, ctx, event, s, env);
		}
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &JSValue, env: &druid::Env) {
		if let Value::String(s) = data.deref() {
			TextBox::<String>::lifecycle(self, ctx, event, s, env);
		}
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &JSValue, data: &JSValue, env: &druid::Env) {
        if let (Value::String(s1), Value::String(s2)) = (old_data.deref(),data.deref()) {
			TextBox::<String>::update(self, ctx, old_data, data, env);
		}
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &JSValue, env: &druid::Env) -> druid::Size {
		if let Value::String(s) = data.deref() {
			TextBox::<String>::layout(self, ctx, bc, s, env)
		} else {
			bc.max()
		}
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &JSValue, env: &druid::Env) {
		if let Value::String(s) = data.deref() {
			TextBox::<String>::paint(self, ctx,  s, env)
		}
    }
}