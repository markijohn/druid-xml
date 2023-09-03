use druid::Widget;

use super::{value::JSValue, qwidget::QWidget};


pub trait ParentableWidget : Widget<JSValue> {
    fn get_childs(&self) -> Option<&[QWidget]>;
}

pub struct QWWidget {
    origin : Box<dyn ParentableWidget>
}

impl Widget<JSValue> for QWWidget {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut JSValue, env: &druid::Env) {
        self.origin.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &JSValue, env: &druid::Env) {
        self.origin.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &JSValue, data: &JSValue, env: &druid::Env) {
        self.origin.update(ctx, old_data, data, env)
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &JSValue, env: &druid::Env) -> druid::Size {
        self.origin.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &JSValue, env: &druid::Env) {
        self.paint(ctx, data, env)
    }
}