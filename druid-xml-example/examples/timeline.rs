use druid::*;
use druid::im::Vector;

#[derive(Clone,Data)]
struct TimeEvent {
    time : f64,
    event : String
}

#[derive(Clone,Data)]
struct Target {
    id : u8,
    events : Vector<TimeEvent>
}

#[derive(Lens,Clone,Data)]
pub struct Timeline {
    head : Option<Target>
}

pub struct TimelineBar;

impl Widget<Timeline> for TimelineBar {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Timeline, env: &Env) {
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Timeline, env: &Env) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Timeline, data: &Timeline, env: &Env) {
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &Timeline, env: &Env) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Timeline, env: &Env) {
        
        ctx.stroke_styled(Rect::new(0., 0., ), brush, width, style)
    }
}

pub fn main() {
    let mut events = Vector::new();
    events.push_back( TimeEvent{time:1.2, event:"OK".to_string()} );
    let window = WindowDesc::new(TimelineBar )
        .window_size((640., 480.))
        .resizable(false)
        .title( "Basic demo" );
    AppLauncher::with_window(window)
        .launch( Timeline { head : Some( Target{id:0, events} )} )
        .expect("launch failed");
}