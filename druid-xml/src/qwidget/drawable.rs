use std::borrow::Cow;
use std::collections::BTreeMap;
use druid::{RoundedRectRadii, Vec2};
use druid::{Point,Size,Insets,Color,LinearGradient,RadialGradient, Widget, PaintCtx, Env, widget::BackgroundBrush, RenderContext};
use druid::kurbo::{Line, Circle, Arc, RoundedRect, Ellipse, Shape};
use druid::piet::{StrokeStyle, IntoBrush};
use fasteval2::{Instruction, Slab, Evaler, Compiler};
use std::str::FromStr;

use crate::simple_style::BorderStyle;



pub struct Calculator {
    src : String,
    cached : (Instruction,Slab),
}

impl Calculator {
    pub fn new(src:String) -> Result<Self, InvalidNumberError> {
        let mut slab = fasteval2::Slab::new();
        let compiled = fasteval2::Parser::new()
            .parse(&src, &mut slab.ps)
            .map_err( |_| InvalidNumberError )?
            .from(&slab.ps).compile(&slab.ps, &mut slab.cs, &mut fasteval2::EmptyNamespace);
        Ok( Self { src, cached : (compiled, slab) } )
    }

    pub fn calc(&self, cvar:&mut CalcVars) -> f64 {
        let (inst,slab) = &self.cached;
        inst.eval(&slab, cvar).unwrap()
    }
}

impl Clone for Calculator {
    fn clone(&self) -> Self {
        Self::new( self.src.clone() ).unwrap()
    }
}


#[derive(Clone)]
pub enum Number {
    Abs(f64), //absolute position
    Rel(f64), //relative position
    Calc( Calculator ) //https://developer.mozilla.org/ko/docs/Web/CSS/calc
}

#[derive(Clone,Debug)]
pub struct InvalidNumberError;

impl Number {
    pub fn calc(&self, vars:&mut CalcVars) -> f64 {
        match self {
            Number::Abs(v) => *v,
            Number::Rel(v) => *v * vars.base_size(),
            Number::Calc(v) => v.calc(vars)
        }
    }
}

impl FromStr for Number {
    type Err = InvalidNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use std::fmt::Write;
        let s = s.trim();
        let v = if s.starts_with("calc(") && s.ends_with(')') {
            let splits = s[5 .. s.len()-1].split( '%');
            let mut expr_str:String = String::new();
            splits.for_each( |e| {
                if e.ends_with("%") 
                && e.len() > 1 
                && e.chars().rev().skip(1).next().unwrap().is_numeric() {
                    let s = e[..e.len()-1].rfind( |c:char| !c.is_numeric() ).unwrap_or(0);
                    write!(&mut expr_str, "__size_perc_{}", &e[s .. e.len()-1].parse::<f64>().unwrap() / 100f64 ).unwrap();
                } else {
                    expr_str.push_str( e );
                }
            });
            Self::Calc( Calculator::new( expr_str )? )
        } else if s.find('.').is_some() {
            Self::Rel( s.parse::<f64>().map_err( |_| InvalidNumberError )? )
        } else if s.ends_with('%') {
            Self::Rel( s[..s.len()-1].parse::<f64>().map_err( |_| InvalidNumberError )? / 100f64 )
        } else {
            Self::Abs( s.parse::<f64>().map_err( |_| InvalidNumberError )?  )
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
pub struct QVec2 {
    x : Number,
    y : Number
}

impl QVec2 {
    pub fn from(x:&str, y:&str) -> Result<Self,InvalidNumberError> {
        Ok( Self { x:Number::from_str(x)?, y:Number::from_str(y)? } )
    }

    pub fn calc(&self, map:&mut CalcVars) -> (f64,f64) {
        ( self.x.calc( map) , self.y.calc( map) )
    }

    pub fn to_vec2(&self, map:&mut CalcVars) -> Vec2 {
        Vec2::new( self.x.calc( map) , self.y.calc( map) )
    }
}


pub struct CalcVars {
    width : f64,
    height : f64,
    time : f64,
    is_width_base : bool
}

impl CalcVars {
    pub fn new(width:f64, height:f64, time:f64, is_width_base:bool) -> Self {
        Self {
            width, height, time, is_width_base
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
    fn lookup(&mut self, name: &str, _args: Vec<f64>, _keybuf: &mut String) -> Option<f64> {
        match name {
            // Custom constants/variables:
            "width" => Some(self.width),
            "height" => Some(self.height),
            "time" => Some(self.time),
            "size" => Some(self.base_size()),

            // Custom function:
            // "sum" => Some(args.into_iter().fold(0.0, |s, f| s + f)),

            _ => {
                if name.starts_with("__size_perc_") {
                    Some( name[ "__size_perc_".len() .. ].parse::<f64>().unwrap() * self.base_size() )
                } else {
                    None
                }
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

    Circle{ center:QVec2, radius:Number, border:Option<BorderStyle>, fill:FillMethod},

    Ellipse{ center:QVec2, radi:QVec2, x_rot:Number, border:Option<BorderStyle>, fill:FillMethod },

    Arc { center:QVec2, radi:QVec2, start_angle:Number, sweep_angle:Number, x_rot:Number, border:BorderStyle},

    //Absolute line
    //start : absolute start point
    //end : absolute end point
    Line{ start:Option<QVec2>, end:QVec2, style:Option<BorderStyle> }
}

pub struct DrawableStack(Vec<Drawable>);

impl DrawableStack {
    pub fn new(vec:Vec<Drawable>) -> Self {
        Self( vec )
    }

    pub fn draw(&self, time:f64, ctx:&mut PaintCtx) {
        let mut last_point = (0., 0.);
        let mut last_style = Default::default();
        let bounds = ctx.size().to_rect();
        let width = bounds.width();
        let height = bounds.height();
        let mut cvar = CalcVars::new(width, height, time, false);
        
        fn draw_fill_bordered(ctx:&mut PaintCtx, fill:&FillMethod, shape:&impl Shape, border:&Option<BorderStyle>) {
            match fill {
                FillMethod::None => (),
                FillMethod::Solid(brush) => { ctx.fill(shape, brush); },
                FillMethod::LinearGradient(brush) => { ctx.fill(shape, brush); },
                FillMethod::RadialGradient(brush) => { ctx.fill(shape, brush); },
            }
            if let Some(border) = border.as_ref() {
                ctx.stroke_styled(shape, &border.color, border.width, &StrokeStyle::default());
            }
        }

        for d in self.0.iter() {
            match d {
                Drawable::Rect { top, right, bottom, left, border, round, fill  } => {
                    let round = round.unwrap_or( Default::default() );
                    let rect = RoundedRect::new( left.calc(cvar.width_base()), top.calc(cvar.height_base()), right.calc(cvar.width_base()), bottom.calc(cvar.height_base()), round );
                    draw_fill_bordered(ctx, fill, &rect, border);
                },
                Drawable::Circle { center, radius, border, fill } => {
                    let mcvar = &mut cvar;
                    let circle = Circle::new( center.calc(mcvar), radius.calc(mcvar));
                    draw_fill_bordered(ctx, fill, &circle, border);
                },
                Drawable::Ellipse { center, border, fill, radi, x_rot } => {
                    let mcvar = &mut cvar;
                    let elli = Ellipse::new( center.calc(mcvar), radi.to_vec2(mcvar), x_rot.calc(mcvar));
                    draw_fill_bordered(ctx, fill, &elli, border);
                },
                Drawable::Arc{ center, radi, start_angle, sweep_angle, x_rot, border } => { 
                    let mcvar = &mut cvar;
                    let center = center.calc(mcvar);
                    let arc = Arc { center: Point { x: center.0, y: center.0 }, radii: radi.to_vec2(mcvar), start_angle: start_angle.calc(mcvar), sweep_angle: sweep_angle.calc(mcvar), x_rotation: x_rot.calc(mcvar) };
                    ctx.stroke_styled(arc, &border.color, border.width, &StrokeStyle::default());
                }
                Drawable::Line { start, end, style } => {
                    let start = start.as_ref().map( |e| e.calc( &mut cvar ) ).unwrap_or( last_point );
                    let end = end.calc( &mut cvar );
                    last_point = end;
                    let style = style.as_ref().unwrap_or( &last_style );
                    //let style = style.as_ref().unwrap_or( &def_stroke );
                    ctx.stroke_styled( Line::new( start, end ), &style.color, style.width, &StrokeStyle::default() );
                },
            }
        }
    }

    pub fn to_background<T>(self) -> BackgroundBrush<T> {
        let painter_fn = Box::new( 
            move |ctx:&mut druid::PaintCtx, t:&T, env:&druid::Env| {
                self.draw(0. , ctx);
            }
        );
        BackgroundBrush::Painter( druid::widget::Painter::<T>::new(painter_fn) )
    }
}

