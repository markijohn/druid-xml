use druid::{widget::{Split, Axis}, Widget, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, LayoutCtx, BoxConstraints, PaintCtx, UpdateCtx, Size};

use crate::qwidget::{value::JSValue, qwidget::QWidget};


pub struct DXSplit {
	origin : Split<JSValue>
}

impl DXSplit {
	pub fn rows(child1:QWidget, child2:QWidget) -> Self {
		Self {
			origin : Split::rows(child1,child2)
		}
	}

	pub fn columns(child1:QWidget, child2:QWidget) -> Self {
		Self {
			origin : Split::columns(child1,child2)
		}
	}
}


impl Widget<JSValue> for DXSplit {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, data: &mut JSValue, _env: &Env) {
		self.origin.event(_ctx, _event, data, _env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &JSValue, env: &Env) {
		self.origin.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &JSValue, data: &JSValue, env: &Env) {
		self.origin.update(ctx, _old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &JSValue, env: &Env) -> Size {
        self.origin.layout(ctx, bc, _data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &JSValue, env: &Env) {
        self.origin.paint(ctx, _data, env)
    }

    fn compute_max_intrinsic(
        &mut self,
        axis: Axis,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &JSValue,
        env: &Env,
    ) -> f64 {
        self.origin.compute_max_intrinsic(axis, ctx, bc, _data, env)
    }
}