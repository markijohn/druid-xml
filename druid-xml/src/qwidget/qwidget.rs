

pub struct QManage {
    //stylers
    //json_value
}


pub struct Query {
    
}

///! Queriable widget
pub struct QWidget {
    localname : Cow<'a,str>,
    style : Rc<StyleSheet>,
    parent : Rc<RefCell<QWidget>>,
    origin : Option<Box<dyn Widget<Value>>>,
    attribute : Attributes,
    //id : unique index
    //paint_stack : smallvec<Drawable>,?
    //attributes?
    //localname?
    //pseudostate
    //value?
    //childs
}

impl <T> Widget<T> for Rc<RefCell<Widget<T>>> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.borrow_mut().event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.borrow_mut().lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.borrow_mut().update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.borrow_mut().layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.deref_mut().paint(ctx, data, env);
    }

    fn id(&self) -> Option<WidgetId> {
        self.borrow().id()
    }

    fn type_name(&self) -> &'static str {
        self.borrow().type_name()
    }
}

pub struct QueryChain {
    //queried : Vec<&'a QWidget>
}

impl QueryChain {
    pub fn q(q:Into<Query>) -> QueryChain {
        todo!()
    }

    pub fn q(&self, q:Into<Query>) -> QueryChain {
        todo!()
    }

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

    pub fn size(&self) -> ! {
        todo!()
    }

    pub fn text(&self) -> &str {
        todo!()
    }

    pub fn toggle_class(&self) -> bool {
        todo!()
    }

    pub fn val(&self) -> ! {
        //json value
        todo!()
    }

    pub fn width(&self) -> usize {
        todo!()
    }

    pub fn len(&self) -> usize {
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