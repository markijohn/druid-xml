use std::{borrow::{Cow, BorrowMut, Borrow}, rc::Rc, cell::UnsafeCell, ops::{Deref, DerefMut}};

use druid::{Widget, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, PaintCtx, WidgetId, Size};
use serde_json::Value;

//pub type QWidget = Rc<UnsafeCell<QWidgetRaw>>;

#[derive(Clone)]
pub struct JSValue(Value);

impl druid::Data for JSValue {
    fn same(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Deref for JSValue {
    type Target=Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JSValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct QWidget(Rc<UnsafeCell<QWidgetRaw>>);

///! Queriable widget
pub struct QWidgetRaw {
    localname : Rc<String>,
    classes : Vec<Rc<String>>,
    parent : Option< QWidget >,
    origin : Option<Box<dyn Widget<Value>>>,
    //attribute : Attributes,
    childs : Vec< QWidget >
}

pub trait Queriable {
    fn find(&self, q:&str) -> QueryChain;
    fn q(&self, q:&str) -> QueryChain;
    fn root(&self) -> QueryChain;
}

impl Queriable for QWidget {
    fn find(&self, q:&str) -> QueryChain {
        //find in self
        QueryChain { queried : vec![ QWidget(self.0.clone()) ] }
    }

    fn q(&self, q:&str) -> QueryChain {
        //find in root
        QueryChain { queried : vec![ QWidget(self.0.clone()) ] }
    }

    fn root(&self) -> QueryChain {
        let mut parent = self.0.clone();
        
        loop {
            if let Some(p_parent) = unsafe { (&*parent.get()).parent.as_ref() } {
                parent = p_parent.0.clone();
            } else {
                break
            }
        }
        QueryChain { queried : vec![ QWidget(parent) ] }
    }
}

impl Widget<JSValue> for QWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut JSValue, env: &Env) {
        self.borrow_mut().event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &JSValue, env: &Env) {
        self.borrow_mut().lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &JSValue, data: &JSValue, env: &Env) {
        self.borrow_mut().update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &JSValue, env: &Env) -> Size {
        self.borrow_mut().layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &JSValue, env: &Env) {
        self.borrow_mut().paint(ctx, data, env);
    }

    fn id(&self) -> Option<WidgetId> {
        self.borrow().id()
    }

    fn type_name(&self) -> &'static str {
        self.borrow().type_name()
    }
}

pub struct QueryChain {
    queried : Vec<QWidget>
}


impl QueryChain {
    pub fn set_class(&self, cls:&str) -> QueryChain {
        todo!()
    }

    pub fn has_class(&self, cls:&str) -> bool {
        todo!()
    }

    pub fn remove_class(&self, cls:&str) -> QueryChain {
        todo!()
    }

    pub fn trigger_class(&self, cls:&str) -> QueryChain {
        todo!()
    }

    pub fn empty(&self) -> QueryChain {
        todo!()
    }

    pub fn click(&self) -> QueryChain {
        todo!()
    }

    pub fn dblclick(&self) -> QueryChain {
        todo!()
    }

    pub fn focus(&self) -> QueryChain {
        todo!()
    }

    pub fn height(&self) -> usize {
        todo!()
    }

    pub fn hide(&self) -> usize {
        todo!()
    }

    pub fn keydown(&self) -> usize {
        todo!()
    }

    pub fn keypress(&self) -> usize {
        todo!()
    }

    pub fn keyup(&self) -> usize {
        todo!()
    }

    pub fn mousedown(&self) -> usize {
        todo!()
    }

    pub fn mouseenter(&self) -> usize {
        todo!()
    }

    pub fn mouseleave(&self) -> usize {
        todo!()
    }

    pub fn mousemove(&self) -> usize {
        todo!()
    }

    pub fn mouseout(&self) -> usize {
        todo!()
    }

    pub fn mouseover(&self) -> usize {
        todo!()
    }

    pub fn mouseup(&self) -> usize {
        todo!()
    }

    pub fn show(&self) -> usize {
        todo!()
    }

    pub fn size(&self) -> Size {
        todo!()
    }
    
    pub fn text(&self) -> &str {
        todo!()
    }

    pub fn toggle_class(&self) -> bool {
        todo!()
    }

    pub fn val(&self) -> Option<&Value> {
        //json value
        todo!()
    }

}

pub enum Drawable {
    Oval,
    Circle,
    Rect,
    RoundedRect,
    Line,
    Image,
    Text
}

pub struct OvalParam {
    //color, fill_color, radius, fill
}

pub struct CircleParam {
    //color, fill_color, radius, color
}

pub struct RectParam {
    //color, fill_color, width, height
}

pub struct RoundedRectParam {
    //color, fill_color, round, width, height
}

pub struct LineParam {
    //color, sx,sy,ex,ey
}

pub struct BezierParam {
    //color, sx,sy,ex,ey
}

pub struct ImageParam {
    //src, object-fit, image-rendering
}

pub struct TextParam {
    //color, text, font-size
}


#[cfg(test)]
mod test {
    #[test]
    fn test() {
        println!("Qwidget");
    }
}