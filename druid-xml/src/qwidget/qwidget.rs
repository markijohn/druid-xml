use std::{borrow::{Cow, BorrowMut, Borrow}, rc::Rc, cell::UnsafeCell, ops::{Deref, DerefMut}, collections::HashSet, collections::HashMap};

use druid::{Widget, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, PaintCtx, WidgetId, Size, Insets, Point, WidgetPod, widget::Axis};
use serde_json::Value;
use simplecss::{Selector, Element};

use super::{drawable::Drawable, value::JSValue};

#[derive(Debug, Eq, PartialEq, Hash)]
struct CacheItem(Rc<String>);

impl Borrow<str> for CacheItem {
    fn borrow(&self) -> &str {
        &self.0
    }
}

pub struct QWidgetContext<'a> {
    localname_table : HashSet<CacheItem>,
    class_table : HashSet<CacheItem>,
    drawables : HashMap<Selector<'a>,Drawable>
}

impl <'a> QWidgetContext<'a> {
    pub fn new() -> Self {
        Self {
            localname_table : HashSet::new(),
            class_table : HashSet::new(),
            drawables : HashMap::new()
        }
    }

    pub fn get_localname(&mut self, name:&str) -> Rc<String> {
        if let Some(exist) = self.localname_table.get( name ) {
            exist.0.clone()
        } else {
            let rc = Rc::new(name.to_string());
            self.localname_table.insert( CacheItem(rc.clone()) );
            rc
        }
    }

    pub fn get_class(&mut self, name:&str) -> Rc<String> {
        if let Some(exist) = self.class_table.get( name ) {
            exist.0.clone()
        } else {
            let rc = Rc::new(name.to_string());
            self.class_table.insert( CacheItem(rc.clone()) );
            rc
        }
    }

    pub fn draw(ctx:&PaintCtx) {

    }
}


pub trait Queryable {
    fn find(&self, q:&str) -> QueryChain;
    fn q(&self, q:&str) -> QueryChain;
    fn root(&self) -> QueryChain;
}

#[derive(Clone)]
pub struct QWidget(Rc<UnsafeCell<QWidgetRaw>>);

///! Queriable widget
struct QWidgetRaw {
    localname : Rc<String>,
    classes : Vec<Rc<String>>,
    parent : Option< QWidget >,
    origin : WidgetPod<JSValue, Box<dyn Widget<JSValue>>>,
    attribute : HashMap<Cow<'static,str>, JSValue>,
    childs : Vec< QWidget >
}


impl Element for QWidget {
    fn parent_element(&self) -> Option<Self> {
        unsafe { (*self.0.get()).parent.clone() }
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        unsafe { 
            if let Some(parent) = (*self.0.get()).parent.as_ref() {
                if let Some(find) = (*parent.0.get()).childs.iter().skip(1).enumerate().find( |(i,e)| Rc::ptr_eq(&e.0, &self.0)) { //e.0.get() == self.0.get() ) {
                    return Some( (*parent.0.get()).childs[find.0].clone() )
                }
            }
            None
        }
    }

    fn has_local_name(&self, name: &str) -> bool {
        unsafe { (*self.0.get()).localname.as_str() == name }
    }

    fn attribute_matches(&self, local_name: &str, operator: simplecss::AttributeOperator) -> bool {
        unsafe { 
            if let Some(val) = (*self.0.get()).attribute.get(local_name) {
                match val.deref() {
                    Value::Null => operator.matches(""),
                    Value::Bool(true) => {
                        operator.matches("true")
                    },
                    Value::Bool(false) => {
                        operator.matches("true")
                    },
                    Value::Number(v) => operator.matches(&v.to_string()),
                    Value::String(v) => operator.matches(v.as_str()),
                    Value::Array(_) => false,
                    Value::Object(_) => false,
                }
            } else {
                false
            }
        }
    }

    fn pseudo_class_matches(&self, class: simplecss::PseudoClass) -> bool {
        todo!()
    }
}

impl Queryable for QWidget {
    fn find(&self, q:&str) -> QueryChain {
        //find in self
        QueryChain ( vec![ QWidget(self.0.clone()) ] )
    }

    fn q(&self, q:&str) -> QueryChain {
        //find in root
        QueryChain ( vec![ QWidget(self.0.clone()) ] ).q( q )
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
        QueryChain ( vec![ QWidget(parent) ] )
    }
}

impl Widget<JSValue> for QWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut JSValue, env: &Env) {
        let origin = unsafe { &mut (&mut *self.0.get()).origin };
        origin.event(ctx, event, data, env);

        //hover animation check
        if origin.is_hot() {
            //if time is not '1.' then repaint as start direction
        }
        //hover animation progressed check
        else {
            //if time is not '0.' then repaint as end direction
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &JSValue, env: &Env) {
        let origin = unsafe { &mut (&mut *self.0.get()).origin };
        origin.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &JSValue, data: &JSValue, env: &Env) {
        let origin = unsafe { &mut (&mut *self.0.get()).origin };
        origin.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &JSValue, env: &Env) -> Size {
        let qraw = unsafe { (&mut *self.0.get()) };
        let origin = &mut qraw.origin;
        origin.layout(ctx,bc,data,env)

        // let padding = qraw.padding;
        // let hpad = padding.x0 + padding.x1;
        // let vpad = padding.y0 + padding.y1;

        // let child_bc = bc.shrink((hpad, vpad));
        // let size = origin.layout(ctx, &child_bc, data, env);
        // let origin_point = Point::new(padding.x0, padding.y0);
        // origin.set_origin(ctx, origin_point);

        // let my_size = Size::new(size.width + hpad, size.height + vpad);
        // let my_insets = origin.compute_parent_paint_insets(my_size);
        // ctx.set_paint_insets(my_insets);
        // let baseline_offset = origin.baseline_offset();
        // if baseline_offset > 0f64 {
        //     ctx.set_baseline_offset(baseline_offset + padding.y1);
        // }
        // my_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &JSValue, env: &Env) {
        //TODO : DrawableStack
        let origin = unsafe { &mut (&mut *self.0.get()).origin };
        origin.paint(ctx, data, env);
    }

    // fn compute_max_intrinsic(
    //     &mut self,
    //     axis: Axis,
    //     ctx: &mut LayoutCtx,
    //     bc: &BoxConstraints,
    //     data: &JSValue,
    //     env: &Env,
    // ) -> f64 {
    //     let qraw = unsafe { (&mut *self.0.get()) };
    //     let origin = &mut qraw.origin;
    //     let padding = qraw.padding.size();
    //     let child_bc = bc.shrink(padding);
    //     let child_max_intrinsic_width = origin
    //         .widget_mut()
    //         .compute_max_intrinsic(axis, ctx, &child_bc, data, env);
    //     child_max_intrinsic_width + axis.major(padding)
    // }
    
    fn id(&self) -> Option<WidgetId> {
        let origin = unsafe { &mut (&mut *self.0.get()).origin };
        Some( origin.id() )
    }

    fn type_name(&self) -> &'static str {
        "qwidget"
    }
}

pub struct QueryChain(Vec<QWidget>);

impl From<Vec<QWidget>> for QueryChain {
    fn from(value: Vec<QWidget>) -> Self {
        Self(value)
    }
}

impl Deref for QueryChain {
    type Target = [QWidget];

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}


impl QueryChain {
    pub fn q(&self, q:&str) -> QueryChain {
        let mut chain = vec![];
        self.iter().for_each(|e| {
            chain.extend( e.q(q).into_iter().map( |qw| qw.clone() ) );
        });
        QueryChain::from(chain)
    }

    pub fn set_class(&self, cls:&str) -> QueryChain {
        // self.iter().for_each( |e| {
        //     e.set_class( cls );
        // })
        todo!()
    }

    pub fn has_class(&self, cls:&str) -> bool {
        todo!()
    }

    pub fn remove_class(&self, cls:&str) -> QueryChain {
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

    pub fn remove(&self) {
        // self.iter().for_each( |e| {
        //     let addr = e.0.get();
        //     let q_raw = unsafe { &mut *e.0.get() };
        //     if let Some(parent) = q_raw.parent.as_mut() {
        //         let parent_origin = unsafe { (*parent.0.get()).origin };
        //         if let Some(parent_origin) = parent_origin {
        //             parent_origin.
        //         }
        //         let mut childs = unsafe { &mut (&mut *parent.0.get()).childs };
        //     }
        //     if let Some(origin) = unsafe { (&*e.0.get()).origin } {
                
        //     }
        // });
        unimplemented!()
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

    pub fn val(&self, new:Option<JSValue>) -> Option<&JSValue> {
        
        //json value
        todo!()
    }

    pub fn val_one(&self) -> Option<&JSValue> {
        if let Some(q) = self.0.get(0) {
            let qw = unsafe { (&mut *q.0.get()) };
            qw.attribute.get("value")
        } else {
            None
        }
    }

}


#[cfg(test)]
mod test {
    #[test]
    fn test() {
        println!("Qwidget");
    }
}