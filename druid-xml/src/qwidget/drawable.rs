use std::borrow::Cow;
use std::collections::BTreeMap;
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
    }

    pub fn calc(&self, size:f64) -> f64 {
        match self.op {
            CalcOp::Add => self.rel*size + self.abs,
            CalcOp::Multiply => self.rel * size * self.abs,
            CalcOp::Divide => self.abs / ( self.rel * size )
        }
    }
}

#[derive(Clone)]
pub enum Number {
    Abs(u64), //absolute position
    Rel(f64), //relative position
    SimpleCalc(SimpleCalc),
    Calc(String) //https://developer.mozilla.org/ko/docs/Web/CSS/calc
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

    pub fn calc(&self, vars:&mut CalcVars) -> f64 {
        match self {
            Number::Abs(v) => *v as _,
            Number::Rel(v) => *v * vars.base_size(),
            Number::SimpleCalc(calc) => calc.calc( vars.base_size() ),
            Number::Calc(v) => fasteval2::ez_eval(v, vars).unwrap()
        }
    }
}

impl FromStr for Number {
    type Err = InvalidNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let v = if s.starts_with("calc(") && s.ends_with(')') {
            Self::Calc( s[5 .. s.len()-1].to_string() )
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

#[derive(Clone)]
pub struct QPoint {
    x : Number,
    y : Number
}

impl QPoint {
    pub fn from(x:&str, y:&str) -> Result<Self,InvalidNumberError> {
        Ok( Self { x:Number::from_str(x)?, y:Number::from_str(y)? } )
    }

    pub fn calc(&self, map:&mut CalcVars) -> (f64,f64) {
        ( self.x.calc( map) , self.y.calc( map) )
    }
}


pub struct CalcVars {
    width : f64,
    height : f64,
    time : f64,
    is_width_base : bool,
    buf : String
}

impl CalcVars {
    pub fn new(width:f64, height:f64, time:f64, is_width_base:bool) -> Self {
        Self {
            width, height, time, is_width_base, buf : String::new()
        }
    }

    pub fn set_width_base(&mut self, flag:bool) {
        self.is_width_base = flag;
    }

    pub fn width_base(&mut self) -> &mut Self {
        self.is_width_base = true;
        self
    }

    pub fn height_base(&mut self) -> &mut Self {
        self.is_width_base = false;
        self
    }

    pub fn base_size(&self) -> f64 {
        if self.is_width_base {
            self.width
        } else {
            self.height
        }
    }
}

impl fasteval2::evalns::EvalNamespace for CalcVars {
    //more advanced function : https://github.com/izihawa/fasteval2/blob/master/examples/advanced-vars.rs
    fn lookup(&mut self, name: &str, args: Vec<f64>, keybuf: &mut String) -> Option<f64> {
        let mydata: [f64; 3] = [11.1, 22.2, 33.3];
        match name {
            // Custom constants/variables:
            "width" => Some(self.width),
            "height" => Some(self.height),
            "time" => Some(self.time),
            "size" => Some(self.base_size()),

            // Custom function:
            "sum" => Some(args.into_iter().fold(0.0, |s, f| s + f)),

            _ => {
                if name.starts_with("__inner_perc_") {
                    Some( name[ "__inner_perc_".len() .. ].parse::<f64>().unwrap() * self.base_size() )
                } else {
                    None
                }
            }

            _ => None,
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
    Line{ start:Option<QPoint>, end:QPoint, style:Option<BorderStyle> }
}

pub struct DrawableStack(Vec<Drawable>);

impl DrawableStack {
    pub fn new(vec:Vec<Drawable>) -> Self {
        Self( vec )
    }

    pub fn draw(&self, ctx:&mut PaintCtx) {
        let mut last_point = (0., 0.);
        let mut last_style = Default::default();
        let bounds = ctx.size().to_rect();
        let width = bounds.width();
        let height = bounds.height();
        let time = 0f64;
        let mut cvar = CalcVars::new(width, height, time, false);

        for d in self.0.iter() {
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
                    let rect = RoundedRect::new( left.calc(cvar.width_base()), top.calc(cvar.height_base()), right.calc(cvar.width_base()), bottom.calc(cvar.height_base()), round );
                    draw!(fill, rect, border);
                },
                Drawable::Circle { center, radius, border, fill } => {
                    let circle = Circle::new( center.calc(&mut cvar), *radius);
                    draw!(fill, circle, border);
                },
                Drawable::Ellipse { center, border, elli, fill } => {
                    let elli = Ellipse::new( center.calc(&mut cvar), elli.radii(), elli.rotation());
                    draw!(fill, elli, border);
                },
                Drawable::Arc{ pos, border, arc} => { 
                    ctx.stroke_styled(arc, &border.color, border.width, &border.style);
                }
                Drawable::Line { start, end, style } => {
                    let start = start.as_ref().map( |e| e.calc( &mut cvar ) ).unwrap_or( last_point );
                    let end = end.calc( &mut cvar );
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
    fn simple_calc_test() {
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

    #[test]
    fn test() {
        let mut cb = |name: &str, args: Vec<f64>| -> Option<f64> {
            let mydata: [f64; 3] = [11.1, 22.2, 33.3];
            match name {
                // Custom constants/variables:
                "x" => Some(3.0),
                "y" => Some(4.0),
                "100%" => Some(100.0),
    
                // Custom function:
                "sum" => Some(args.into_iter().fold(0.0, |s, f| s + f)),
    
                // Custom array-like objects:
                // The `args.get...` code is the same as:
                //     mydata[args[0] as usize]
                // ...but it won't panic if either index is out-of-bounds.
                "data" => args.get(0).and_then(|f| mydata.get(*f as usize).copied()),
    
                // A wildcard to handle all undefined names:
                _ => None,
            }
        };
    
        let val = fasteval2::ez_eval("sum(x^2, y^2)^0.5 + data[0] ", &mut cb).unwrap();
        //                           |   |                   |
        //                           |   |                   square-brackets act like parenthesis
        //                           |   variables are like custom functions with zero args
        //                           custom function
    
        assert_eq!(val, 16.1);
    }
}