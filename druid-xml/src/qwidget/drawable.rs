use std::borrow::Cow;
use druid::{Point,Size,Insets,Color,LinearGradient,RadialGradient, Widget, PaintCtx, Env, widget::BackgroundBrush, RenderContext};
use druid::kurbo::{Line, Arc, RoundedRect, Ellipse};
use druid::piet::StrokeStyle;


#[derive(Clone)]
pub struct BorderStyle {
    style : StrokeStyle,
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
    Rect{ pos:Pos, border:Option<BorderStyle>, rect:RoundedRect, fill:FillMethod },

    Ellipse{ pos:Pos, border:Option<BorderStyle>, elli:Ellipse, fill:FillMethod },

    Arc { pos:Pos, border:BorderStyle, arc:Arc, col:Color },

    //Absolute line
    //start : absolute start point
    //end : absolute end point
    Line{ pos:Pos, start:Option<Point>, end:Point, width:Option<f64>, col:Option<Color>, style:Option<StrokeStyle> }
}

pub struct DrawableStack(Vec<Drawable>);

impl DrawableStack {
    pub fn draw(&self, ctx:&mut PaintCtx) {
        for d in self.0.iter() {
            let bounds = ctx.size().to_rect();

            macro_rules! draw {
                (border, $shape:ident, $border:ident, $brush:ident) => {
                    ctx.fill($shape, $brush);
                    if let Some(border) = $border.as_ref() {
                        ctx.stroke_styled($shape, $brush, border.width, &border.style);
                    }
                };
            }
            
            match d {
                Drawable::Rect { pos, border, rect, fill } => {
                    match fill {
                        FillMethod::Solid(b) => { draw!(border, rect, border, b); },
                        FillMethod::LinearGradient(b) => { draw!(border, rect, border, b); },
                        FillMethod::RadialGradient(b) => { draw!(border, rect, border, b); },
                    }
                },
                Drawable::Ellipse { pos, border, elli, fill } => {
                    match fill {
                        FillMethod::Solid(b) => { draw!(border, elli, border, b); },
                        FillMethod::LinearGradient(b) => { draw!(border, elli, border, b); },
                        FillMethod::RadialGradient(b) => { draw!(border, elli, border, b); },
                    }
                },
                Drawable::Arc{ pos, border, arc, col } => { 
                    ctx.stroke_styled(arc, col, border.width, &border.style);
                }
                Drawable::Line { pos, start, end, width, col, style } => {
                    let width = width.unwrap_or(1f64);
                    let col = col.unwrap_or( Color::rgb8(0,0,0) );
                    let def_stroke = StrokeStyle::default();
                    let style = style.as_ref().unwrap_or( &def_stroke );
                    ctx.stroke_styled( Line::new( (0.,0.), (10.,10.) ), &col, width, &style );
                },
            }
        }
    }

    pub fn to_background<T>(self) -> BackgroundBrush<T> {
        todo!()
    }
}