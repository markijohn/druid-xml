

pub struct QManage {
    //stylers
    //json_value
}


pub struct Query {
    
}

///! Queriable widget
pub struct QWidget {
    //id : unique index
    //paint_stack : smallvec<Drawable>,
    //attributes
    //localname
    //pseudostate
    //value
    //childs
}

pub struct QueryChain {
    //queried : Vec<&'a QWidget>
}

impl QueryChain {
    pub fn q(q:Into<Query>) -> QueryChain {
        todo!()
    }
}

impl QWidget {
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