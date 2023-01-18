use std::borrow::Cow;
use druid::RoundedRectRadii;
use druid::{Point,Size,Insets,Color,LinearGradient,RadialGradient, Widget, PaintCtx, Env, widget::BackgroundBrush, RenderContext};
use druid::kurbo::{Line, Circle, Arc, RoundedRect, Ellipse};
use druid::piet::StrokeStyle;
use std::str::FromStr;

#[derive(Clone)]
pub struct BorderStyle {
    style : StrokeStyle,
    width: f64,
    color: Color,
}

impl BorderStyle {
    pub fn new(style:StrokeStyle, width:f64, color:impl Into<Color>) -> Self {
        Self { style, width, color : color.into()}
    }
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self { style: Default::default(), width: 1f64, color: Color::rgb8(0,0,0) }
    }
}

#[derive(Clone,Copy)]
pub enum Number {
    Abs(u64), //absolute position
    Rel(f64) //relative position
}

#[derive(Clone,Debug)]
pub struct InvalidNumberError;

impl Number {
    pub fn as_u64(&self) -> Option<u64> {
        if let Self::Abs(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_u64_or(&self, def:u64) -> u64 {
        if let Self::Abs(v) = self {
            *v
        } else {
            def
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        if let Self::Rel(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_f64_or(&self, def:f64) -> f64 {
        if let Self::Rel(v) = self {
            *v
        } else {
            def
        }
    }

    pub fn calc_rate(&self, size:f64) -> f64 {
        match self {
            Number::Abs(v) => *v as _,
            Number::Rel(v) => size * *v,
        }
    }
}

impl FromStr for Number {
    type Err = InvalidNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = if s.find('.').is_some() {
            Self::Rel( s.parse::<f64>().map_err( |_| InvalidNumberError )? )
        } else if s.ends_with('%') {
            Self::Rel( s[..s.len()-1].parse::<f64>().map_err( |_| InvalidNumberError )? / 100f64 )
        } else {
            Self::Abs( s.parse::<u64>().map_err( |_| InvalidNumberError )?  )
        };
        Ok( v )
    }
}

#[derive(Clone)]
pub enum Round {
    Solid{ radius:f64, width:f64, color:Color },
    Dash{ radius:f64, width:f64, color:Color, offset:f64 }
}

#[derive(Clone)]
pub enum FillMethod {
    None,
    Solid( Color ),
    LinearGradient( LinearGradient ),
    RadialGradient( RadialGradient ),
}

#[derive(Clone)]
pub enum Pos {
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

#[derive(Clone,Copy)]
pub struct QPoint {
    x : Number,
    y : Number
}

impl QPoint {
    pub fn from(x:&str, y:&str) -> Result<Self,InvalidNumberError> {
        Ok( Self { x:Number::from_str(x)?, y:Number::from_str(y)? } )
    }

    pub fn calc_rate(&self, width:f64, height:f64) -> (f64,f64) {
        ( self.x.calc_rate( width) , self.y.calc_rate( height) )
    }
}

#[derive(Clone)]
pub enum PointEnd {
    WidthHeight(QPoint), //width, height
    RightBottom(QPoint) //right, bottom
}

impl PointEnd {
    pub fn calc(&self, start:(f64,f64), width:f64, height:f64) -> (f64,f64) {
        match self {
            Self::WidthHeight(point) => {
                let calc = point.calc_rate(width, height);
                ( start.0+calc.0, start.1+calc.1 )
            },
            Self::RightBottom(point) => {
                let calc = point.calc_rate(width, height);
                ( width-calc.0, height-calc.1 )
            }
        }
    }
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
    //start: top left
    //end: padding : top right bottom left
    //size : x , y
    Rect{ start:QPoint, end:PointEnd, border:Option<BorderStyle>, round:Option<RoundedRectRadii>, fill:FillMethod },

    Circle{ center:QPoint, radius:f64, border:Option<BorderStyle>, fill:FillMethod},

    Ellipse{ center:QPoint, border:Option<BorderStyle>, elli:Ellipse, fill:FillMethod },

    Arc { pos:QPoint, border:BorderStyle, arc:Arc },

    //Absolute line
    //start : absolute start point
    //end : absolute end point
    Line{ start:Option<QPoint>, end:PointEnd, style:Option<BorderStyle> }
}

pub struct DrawableStack(Vec<Drawable>);

impl DrawableStack {
    pub fn new(vec:Vec<Drawable>) -> Self {
        Self( vec )
    }

    pub fn draw(&self, ctx:&mut PaintCtx) {
        let mut last_point = (0., 0.);
        let mut last_style = Default::default();
        for d in self.0.iter() {
            
            let bounds = ctx.size().to_rect();
            
            macro_rules! draw {
                ($fill:ident, $shape:ident, $border:ident) => {
                    match $fill {
                        FillMethod::None => (),
                        FillMethod::Solid(brush) => { ctx.fill($shape, brush); },
                        FillMethod::LinearGradient(brush) => { ctx.fill($shape, brush); },
                        FillMethod::RadialGradient(brush) => { ctx.fill($shape, brush); },
                    }
                    if let Some(border) = $border.as_ref() {
                        ctx.stroke_styled($shape, &border.color, border.width, &border.style);
                    }
                };
            }
            
            match d {
                Drawable::Rect { start, end, border, round, fill  } => {
                    let start = start.calc_rate( bounds.width(), bounds.height() );
                    let end = end.calc( start, bounds.width(), bounds.height() );
                    let round = round.unwrap_or( RoundedRectRadii { top_left: 0., top_right: 0., bottom_right: 0., bottom_left: 0. });
                    let rect = RoundedRect::new( start.0, start.1, end.0, end.1, round );
                    draw!(fill, rect, border);
                },
                Drawable::Circle { center, radius, border, fill } => {
                    let circle = Circle::new( center.calc_rate(bounds.width(), bounds.height()), *radius);
                    draw!(fill, circle, border);
                },
                Drawable::Ellipse { center, border, elli, fill } => {
                    let elli = Ellipse::new( center.calc_rate(bounds.width(), bounds.height()), elli.radii(), elli.rotation());
                    draw!(fill, elli, border);
                },
                Drawable::Arc{ pos, border, arc} => { 
                    ctx.stroke_styled(arc, &border.color, border.width, &border.style);
                }
                Drawable::Line { start, end, style } => {
                    let start = start.map( |e| e.calc_rate( bounds.width(), bounds.height() ) ).unwrap_or( last_point );
                    let end = end.calc( start, bounds.width(), bounds.height() );
                    last_point = end;
                    let style = style.as_ref().unwrap_or( &last_style );
                    //let style = style.as_ref().unwrap_or( &def_stroke );
                    ctx.stroke_styled( Line::new( start, end ), &style.color, style.width, &style.style );
                },
            }
        }
    }

    pub fn to_background<T>(self) -> BackgroundBrush<T> {
        let painter_fn = Box::new( 
            move |ctx:&mut druid::PaintCtx, t:&T, env:&druid::Env| {
                self.draw(ctx);
            }
        );
        BackgroundBrush::Painter( druid::widget::Painter::<T>::new(painter_fn) )
    }
}