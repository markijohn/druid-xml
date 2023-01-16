use std::borrow::Cow;
use druid::{Point,Size,Insets,Color,LinearGradient,RadialGradient, Widget, PaintCtx, Env, widget::BackgroundBrush, RenderContext};
use druid::kurbo::{Arc, RoundedRect, Ellipse};

struct BorderStyle {
    style : 
    width: f64,
    color: Color,
}

#[derive(Clone)]
pub enum Round {
    Solid{ radius:f64, width:f64, color:Color },
    Dash{ radius:f64, width:f64, color:Color, start:f64, gap:f64 }
}

#[derive(Clone)]
pub enum FillMethod {
    Solid( Color ),
    LinearGradient( LinearGradient ),
    RadialGradient( RadialGradient ),
}

#[derive(Clone)]
pub enum Pos {
    Abs{x:f64, y:f64},
    TopLeft{ x:f64, y:f64 },
    TopCenter { x:f64, y : f64 },
    TopRight { x:f64, y : f64 },
    CenterLeft { x:f64, y : f64 },
    Center { x:f64, y : f64 },
    CenterRight { x:f64, y : f64 },
    BottomLeft { x:f64, y : f64 },
    BottomCenter { x:f64, y : f64 },
    BottomRight { x:f64, y : f64 }
}

///! ex:)
/// <drawable>
///     <rect border="1px solid black" background-color:white>
///     <line start="0,0" base="topleft">
///         <to point="20,20"/>
///         <to point="20,30"/>
///         <start point="30,30"/>
///     </line>
///     
#[derive(Clone)]
pub enum Drawable {
    //radius : 
    //padding : top right bottom left
    //size : x , y
    Rect{ pos:Pos, rect:RoundedRect, fill:FillMethod },

    Ellipse{ pos:Pos, elli:Ellipse, fill:FillMethod },

    //Absolute line
    //start : absolute start point
    //end : absolute end point
    Line{ start:Point, end:Point },

    Text { pos:Pos, text:Cow<'static,str>, font:Option<&'static str>, font_size:Option<f64> }
}

pub struct DrawableStack(Vec<Drawable>);

impl DrawableStack {
    pub fn draw(&self, ctx:&mut PaintCtx) {
        for d in self.0.iter() {
            let bounds = ctx.size().to_rect();
            
            match d {
                Drawable::Rect { pos, rect, fill } => {
                    match fill {
                        FillMethod::Solid(b) => ctx.fill(rect, b),
                        FillMethod::LinearGradient(b) => ctx.fill(rect, b),
                        FillMethod::RadialGradient(b) => ctx.fill(rect, b),
                    }
                },
                Drawable::Ellipse { pos, elli, fill } => {
                    match fill {
                        FillMethod::Solid(b) => ctx.fill(elli, b),
                        FillMethod::LinearGradient(b) => ctx.fill(elli, b),
                        FillMethod::RadialGradient(b) => ctx.fill(elli, b),
                    }
                },
                Drawable::Line { start, end } => todo!(),
                Drawable::Text { text, pos, font, font_size } => todo!(),
            }
        }
    }

    pub fn to_painter<T>(self) -> BackgroundBrush<T> {
        todo!()
    }
}