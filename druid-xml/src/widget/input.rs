use std::ops::{DerefMut, Deref};

use druid::{Widget, widget::TextBox};
use serde_json::Value;

use crate::qwidget::value::JSValue;

pub struct DXTextBox {
	origin : TextBox<String>
}

impl DXTextBox {
	pub fn multiline() -> DXTextBox {
		DXTextBox {
			origin : TextBox::multiline()
		}
	}

	pub fn new() -> DXTextBox {
		DXTextBox { origin: TextBox::new() }
	}
}

impl Widget<JSValue> for DXTextBox {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut JSValue, env: &druid::Env) {
		if let Value::String( s) = data.deref_mut() {
			self.origin.event( ctx, event, s, env);
		}
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &JSValue, env: &druid::Env) {
		if let Value::String(s) = data.deref() {
			self.origin.lifecycle(ctx, event, &s, env);
		}
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &JSValue, data: &JSValue, env: &druid::Env) {
        if let (Value::String(_s1), Value::String(_s2)) = (old_data.deref(),data.deref()) {
			self.origin.update(ctx, _s1, _s2, env);
		}
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &JSValue, env: &druid::Env) -> druid::Size {
		if let Value::String(s) = data.deref() {
			self.origin.layout(ctx, bc, &s, env)
		} else {
			bc.max()
		}
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &JSValue, env: &druid::Env) {
		if let Value::String(s) = data.deref() {
			self.origin.paint(ctx,  &s, env)
		}
    }
}