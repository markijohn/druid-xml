use std::borrow::Cow;

use druid::{Widget, Data};

pub struct ImageInfo {
	width : usize,
	height : usize
}


pub enum Status {
	Idle,
	Ready(ImageInfo),
	Progress(f64),
	Closed
}

pub trait StreamHandler {
	fn read_stream(&self, buffer:&mut [u8]) -> std::io::Result<usize>;
}

pub struct FileStreamHandler {
	
}

pub struct LazyImage<'a,S:StreamHandler> {
	src : Cow<'a,str>,
	status : Status,
	handle : S
}

impl <'a, T:Data, S:StreamHandler> Widget<T> for LazyImage<'a, S> {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut T, env: &druid::Env) {
        todo!()
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &T, env: &druid::Env) {
        todo!()
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &T, data: &T, env: &druid::Env) {
        todo!()
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &T, env: &druid::Env) -> druid::Size {
        todo!()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &druid::Env) {
        todo!()
    }
}

