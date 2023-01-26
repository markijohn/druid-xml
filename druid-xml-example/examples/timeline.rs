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
        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        //
        // bx.max() returns the maximum size of the widget. Be careful
        // using this, since always make sure the widget is bounded.
        // If bx.max() is used in a scrolling widget things will probably
        // not work correctly.
        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(300.0, 100.0);
            bc.constrain(size)
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Timeline, env: &Env) {
        let size = ctx.size();
        ctx.stroke(Rect::new(0., 0., size.width, size.height), &Color::rgb8(0,0,200), 1.);
    }
}

pub fn main() {
    let mut events = Vector::new();
    events.push_back( TimeEvent{time:1.2, event:"OK".to_string()} );
    let window = WindowDesc::new(TimelineBar )
        .window_size((640., 480.))
        .resizable(true)
        .title( "Basic demo" );
    AppLauncher::with_window(window)
        .launch( Timeline { head : Some( Target{id:0, events} )} )
        .expect("launch failed");
}