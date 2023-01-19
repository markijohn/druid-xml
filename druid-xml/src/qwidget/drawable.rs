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
    Rel(f64), //relative position
    Calc(SimpleCalc)
}

#[derive(Clone,Debug)]
pub struct InvalidNumberError;

#[derive(Debug,Clone,Copy)]
enum CalcOp {
    Add,
    Multiply,
    Divide
}

#[derive(Debug,Clone,Copy)]
//https://developer.mozilla.org/ko/docs/Web/CSS/calc
//not support full spec
pub struct SimpleCalc {
    rel : f64,
    op : CalcOp,
    abs : f64,
}

impl SimpleCalc {
    pub fn parse(s:&str) -> Result<Self,InvalidNumberError> {
        let s = s.trim();
        if s.starts_with("calc(") && s.ends_with(")") {
            let mut split = s[5 .. s.rfind(')').unwrap()].split_whitespace();
            let rel = split.next().ok_or_else(|| InvalidNumberError)?;
            let op = split.next().ok_or_else(|| InvalidNumberError)?;
            let abs = split.next().ok_or_else(|| InvalidNumberError)?;

            let (rel,is_rel) = if rel.ends_with('%') {
                (rel[..rel.len()-1].parse::<f64>().map_err( |_| InvalidNumberError )? / 100f64, true)
            } else if rel.find('.').is_some() {
                (rel.parse::<f64>().map_err( |_| InvalidNumberError )?, true)
            } else {
                (rel.parse::<f64>().map_err( |_| InvalidNumberError )?, false)
            };

            
            let (rel,op,abs) = if is_rel {
                let abs = abs.parse::<f64>().map_err( |_| InvalidNumberError )?;
                match op {
                    "+" => (rel, CalcOp::Add, abs),
                    "-" => (rel, CalcOp::Add, -abs),
                    "*" => (rel, CalcOp::Multiply, abs),
                    //"/" => (rel, CalcOp::Multiply, abs / 10f64.powi( (abs.log10().floor()+1f64) as _ ) ),
                    "/" => (rel, CalcOp::Multiply, 1f64 / abs ),
                    _ => return Err(InvalidNumberError)
                }
            } else {
                let _abs = rel;
                let rel = if abs.ends_with('%') {
                    abs[..abs.len()-1].parse::<f64>().map_err( |_| InvalidNumberError )? / 100f64
                } else if abs.find('.').is_some() {
                    abs.parse::<f64>().map_err( |_| InvalidNumberError )?
                } else {
                    return Err(InvalidNumberError)
                };
                match op {
                    "+" => (rel, CalcOp::Add, _abs),
                    "-" => (-rel, CalcOp::Add, _abs),
                    "*" => (rel, CalcOp::Multiply, _abs),
                    "/" => (rel, CalcOp::Divide, _abs ),
                    _ => return Err(InvalidNumberError)
                }
            };
            Ok( Self {rel, op, abs} )
        } else {
            Err(InvalidNumberError)
        }
    }

    pub fn calc(&self, size:f64) -> f64 {
        match self.op {
            CalcOp::Add => self.rel*size + self.abs,
            CalcOp::Multiply => self.rel * size * self.abs,
            CalcOp::Divide => self.abs / ( self.rel * size )
        }
    }
}

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

    pub fn calc(&self, size:f64) -> f64 {
        match self {
            Number::Abs(v) => *v as _,
            Number::Rel(v) => size * *v,
            Number::Calc(v) => v.calc(size)
        }
    }
}

impl FromStr for Number {
    type Err = InvalidNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = if let Ok(calc) = SimpleCalc::parse(s) {
            Self::Calc(calc)
        } else if s.find('.').is_some() {
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
        ( self.x.calc( width) , self.y.calc( height) )
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
    Rect{ top:Number, right:Number, bottom:Number, left:Number, border:Option<BorderStyle>, round:Option<RoundedRectRadii>, fill:FillMethod },

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
            let width = bounds.width();
            let height = bounds.height();
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
                Drawable::Rect { top, right, bottom, left, border, round, fill  } => {
                    let round = round.unwrap_or( Default::default() );
                    let rect = RoundedRect::new( left.calc(width), top.calc(height), right.calc(width), bottom.calc(height), round );
                    draw!(fill, rect, border);
                },
                Drawable::Circle { center, radius, border, fill } => {
                    let circle = Circle::new( center.calc_rate(width, height), *radius);
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

#[cfg(test)]
mod test {
    use super::SimpleCalc;

    #[test]
    fn calc_test() {
        //add
        let calc = SimpleCalc::parse("calc(90% + 20)").unwrap();
        println!("{:?} {}", calc, calc.calc(100.));
        assert!( calc.calc(100.) == 110. );
        //add reverse
        let calc = SimpleCalc::parse("calc(20 + 90%)").unwrap();
        println!("{:?} {}", calc, calc.calc(200.));
        assert!( calc.calc(200.) == 200. );

        //minus 
        let calc = SimpleCalc::parse("calc(100% - 50)").unwrap();
        println!("{:?} {}", calc, calc.calc(200.));
        assert!( calc.calc(200.) == 150. );
        //minus reverse
        let calc = SimpleCalc::parse("calc(200 - 100%)").unwrap();
        println!("{:?} {}", calc, calc.calc(100.));
        assert!( calc.calc(100.) == 100. );

        //multiply
        let calc = SimpleCalc::parse("calc(15% * 20)").unwrap();
        println!("{:?} {}", calc, calc.calc(100.));
        assert!( calc.calc(100.) == 300. );
        //multiply reverse
        let calc = SimpleCalc::parse("calc(20 * 15%)").unwrap();
        println!("{:?} {}", calc, calc.calc(100.));
        assert!( calc.calc(100.) == 300. );

        //divide
        let calc = SimpleCalc::parse("calc(100% / 2)").unwrap();
        println!("{:?} {}", calc, calc.calc(648.));
        assert!( calc.calc(648.) == 324. );
        //divide reverse
        let calc = SimpleCalc::parse("calc(640 / 40%)").unwrap();
        println!("{:?} {}", calc, calc.calc(10.));
        assert!( calc.calc(10.) == 160. );
        
    }
}