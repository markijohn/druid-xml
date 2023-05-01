#![allow(arithmetic_overflow)]

use std::{rc::Rc, ops::{Deref, DerefMut}, time::Duration};

use druid::{Size, Insets, Color, Rect, piet::StrokeStyle};

use crate::{curve::AnimationCurve};

#[derive(Debug,Clone,Copy)]
pub enum JumpTerm {
    JumpStart, //Denotes a left-continuous function, so that the first jump happens when the animation begins
    JumpEnd, //Denotes a right-continuous function, so that the last jump happens when the animation ends
    JumpNone, //There is no jump on either end. Instead, holding at both the 0% mark and the 100% mark, each for 1/n of the duration
    JumpBoth, //Includes pauses at both the 0% and 100% marks, effectively adding a step during the animation iteration
    Start, //Same as jump-start
    End //Same as jump-end
}

#[derive(Debug,Clone,Copy)]
pub enum TimingFunction {
    Ease, //Equal to cubic-bezier(0.25, 0.1, 0.25, 1.0), the default value, increases in velocity towards the middle of the animation, slowing back down at the end
    EaseIn, //Equal to cubic-bezier(0.42, 0, 1.0, 1.0), starts off slowly, with the speed of the transition of the animating property increasing until complete
    EaseOut, //Equal to cubic-bezier(0, 0, 0.58, 1.0), starts quickly, slowing down the animation continues
    EaseInOut, //Equal to cubic-bezier(0.42, 0, 0.58, 1.0), with the animating properties slowly transitioning, speeding up, and then slowing down again
    Linear, //Equal to cubic-bezier(0.0, 0.0, 1.0, 1.0), animates at an even speed
    StepStart, //Equal to steps(1, jump-start)
    StepEnd, //Equal to steps(1, jump-end)
    CubicBezier{p1:f64, p2:f64, p3:f64, p4:f64}, //An author defined cubic-bezier curve, where the p1 and p3 values must be in the range of 0 to 1

    //Displays an animation iteration along n stops along the transition, 
    //displaying each stop for equal lengths of time. For example, if n is 5, there are 5 steps. 
    //Whether the animation holds temporarily at 0%, 20%, 40%, 60% and 80%, on the 20%, 40%, 60%, 80% and 100%, or makes 5 stops between the 0% and 100% along the animation, 
    //or makes 5 stops including the 0% and 100% marks (on the 0%, 25%, 50%, 75%, and 100%) depends on which of the following jump terms is used jumpterm
    Steps{n:f64, jumpterm:JumpTerm}
}

impl TimingFunction {
    pub fn translate(&self, t:f64) -> f64 {
        match self {
            TimingFunction::Ease => AnimationCurve::cubic(0.25, 0.1, 0.25, 1.0).translate(t),
            TimingFunction::EaseIn => AnimationCurve::EASE_IN.translate(t),
            TimingFunction::EaseOut => AnimationCurve::EASE_OUT.translate(t),
            TimingFunction::EaseInOut => AnimationCurve::EASE_IN_OUT.translate(t),
            TimingFunction::Linear => t,
            TimingFunction::StepStart => todo!(),
            TimingFunction::StepEnd => todo!(),
            TimingFunction::CubicBezier { p1, p2, p3, p4 } => AnimationCurve::cubic(*p1, *p2, *p3, *p4).translate(t),
            TimingFunction::Steps { n, jumpterm } => todo!(),
        }
    }
}

#[derive(Debug,Clone)]
pub enum Direction {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse
}

#[derive(Debug,Clone)]
pub struct Animation {
    pub delay : i64, //delay for start
    pub direction : Direction, //when animation is end how to start
    pub duration : i64, //animation time in one cycle. actually this is the like animation speed (nanosecond)
    pub iteration : f64, //how many repeat animation
    pub name : f64, //animation progression state
    pub timing_function : TimingFunction, //timinig function
    pub fill_mode : f64, //how to fill when animation start/end
}

#[derive(Debug,Clone)]
pub struct AnimationState {
    elapsed : i64,
    anim : Animation
}

impl From<Animation> for AnimationState {
    fn from(value: Animation) -> Self {
        Self {
            elapsed : 0,
            anim : value
        }
    }
}

impl AnimationState {
    pub fn transit<T:Transit>(&mut self,src:T, target:T, interval:i64) -> (bool,T) {
        let old_elapsed = self.elapsed;
        //println!("elapsed interval duration {} {} {}", self.elapsed, interval, self.anim.duration);

        self.elapsed += interval;
        let has_more = 
        if self.elapsed <= 0 {
            self.elapsed = 0;
            if old_elapsed == self.elapsed {
                return (false, src)
            }
            false
        } else if self.elapsed >= self.anim.duration {
            self.elapsed = self.anim.duration;
            if old_elapsed == self.elapsed {
                return (false, target)
            }
            false
        } else {
            true
        };

        //let alpha = self.anim.timing_function.translate( self.elapsed as f64 / self.anim.duration as f64 );
        let alpha = self.elapsed as f64 / self.anim.duration as f64;
        // println!("alpha {} {} {} {}", self.elapsed, self.anim.duration, interval, alpha);

        (has_more, src.transit(target, alpha))
    }
}

#[derive(Clone,Debug)]
pub struct StyleQueryResult<T> {
    is_animated : bool,
    data : Option<T>
}

impl <T> StyleQueryResult<T> {
    pub fn new(is_animated:bool, data:Option<T>) -> Self {
        Self {
            is_animated,
            data
        }
    }

    pub fn some( is_animated:bool, data:T ) -> Self {
        Self {
            is_animated,
            data : Some(data)
        }
    }

    pub fn none( is_animated:bool ) -> Self {
        Self {
            is_animated,
            data : None
        }
    }

    pub fn into(self) -> (bool,Option<T>) {
        (self.is_animated, self.data)
    }

    pub fn has_next_animation(&self) -> bool {
        self.is_animated
    }
}

pub trait Transit {
    /// `forward_dir` flag is linear forward
    /// `target` is the goal of transit
    /// `duration` is animation time
    /// `interval` how to elapsed time
    /// (bool,Self) first bool is reach the end. Self is calculate value
    fn transit(self, target:Self, alpha:f64) -> Self;

    fn alpha(self, target:Self, status:Self) -> f64;
}

// impl Transit for f64 {
//     fn transit(self, target:Self, alpha:f64) -> Self {
//         let diff = target - self;
//         self + diff * alpha
//     }
//     fn alpha(self, target:Self, status:Self) -> Self {
//         let alpha = if target == self {
//             //avoid zero divide and don't need animation
//             1.
//         } else {
//             let max = target.max(self);
//             let min = target.min(self);
//             let n_alpha = (status.max(min) - status.min(min)) / (max - min);
//             let n_alpha = if n_alpha < 0. {
//                 0.
//             } else if n_alpha > 1. {
//                 1.
//             } else {
//                 n_alpha
//             };
//             //the direction
//             if target < self {
//                 1. - n_alpha
//             } else {
//                 n_alpha
//             }
//         };
//         //alpha
//         alpha.min(1.0).max(0.0)
//     }
// }

impl Transit for f64 {
    fn transit(self, target:Self, alpha:f64) -> Self {     
        let diff = target - self;
        self + diff * alpha
    }
    fn alpha(self, target:Self, status:Self) -> Self {
        let alpha = if target == self {
            //avoid zero divide and don't need animation
            1.
        } else {
            ((status - self) / (target - self)).max(0.0).min(1.0)
        };
        alpha
    }
}

// impl Transit for f64 {
//     fn transit(self, target: Self, alpha: f64) -> Self {
//         let diff = target - self;
//         self + diff * alpha
//     }

//     fn alpha(self, target: Self, status: Self) -> Self {
//         if target == self {
//             // If the target and the source are equal,
//             // there is no need for animation.
//             1.0
//         } else {
//             let (min, max) = if self < target {
//                 (self, target)
//             } else {
//                 (target, self)
//             };

//             let diff = max - min;
//             let clamped_status = status.max(min).min(max);
//             let n_alpha = (clamped_status - min) / diff;
//             //n_alpha.max(0.0).min(1.0)
//             n_alpha.max(1.0).min(0.0)
//         }
//     }
// }


impl Transit for u8 {
    fn transit(self, target: Self, alpha: f64) -> Self {
        let diff = (target as i16 - self as i16) as f64;
        let value = (self as f64 + diff * alpha).round();
        value as _
    }

    fn alpha(self, target: Self, status: Self) -> f64 {
        let max = target.max(self);
        let min = target.min(self);
        let n_alpha = (status.max(min) - status.min(min)) as f64 / (max - min) as f64;
        let n_alpha = n_alpha.max(0.0).min(1.0);
        if target < self {
            1. - n_alpha
        } else {
            n_alpha
        }
    }
}

// impl Transit for Insets {
//     fn transit(self, target:Self, alpha:f64) -> Self {
//         let diff_x0 = target.x0 - self.x0;
//         let diff_y0 = target.y0 - self.y0;
//         let diff_x1 = target.x1 - self.x1;
//         let diff_y1 = target.y1 - self.y1;
//         // println!("alpha  {}",alpha);
//         // println!("diff  {} {} {} {}",diff_x0, diff_y0, diff_x1, diff_y1);
//         Self { 
//             x0: self.x0 + diff_x0 * alpha, 
//             y0: self.y0 + diff_y0 * alpha, 
//             x1: self.x1 + diff_x1 * alpha, 
//             y1: self.y1 + diff_y1 * alpha 
//         }
//     }
//     fn alpha(self, target:Self, status:Self) -> f64 {
//         self.x0.alpha(target.x0, status.x0)
//         .min( self.y0.alpha(target.y0, status.y0) )
//         .min( self.x1.alpha(target.x1, status.x1) )
//         .min( self.y1.alpha(target.y1, status.y1) )
//     }
// }

impl Transit for Insets {
    fn transit(self, target: Self, alpha: f64) -> Self {
        let diff_x0 = target.x0 - self.x0;
        let diff_y0 = target.y0 - self.y0;
        let diff_x1 = target.x1 - self.x1;
        let diff_y1 = target.y1 - self.y1;
        // println!("alpha  {}",alpha);
        // println!("diff  {} {} {} {}",diff_x0, diff_y0, diff_x1, diff_y1);
        Self { 
            x0: self.x0 + diff_x0 * alpha, 
            y0: self.y0 + diff_y0 * alpha, 
            x1: self.x1 + diff_x1 * alpha, 
            y1: self.y1 + diff_y1 * alpha 
        }
    }

    // fn transit(self, target: Self, alpha: f64) -> Self {
    //     let x0 = self.x0.transit(target.x0, alpha);
    //     let y0 = self.y0.transit(target.y0, alpha);
    //     let x1 = self.x1.transit(target.x1, alpha);
    //     let y1 = self.y1.transit(target.y1, alpha);
    //     Insets { x0, y0, x1, y1 }
    // }

    fn alpha(self, target: Self, status: Self) -> f64 {
        // Calculate alpha values for each edge
        let alpha_x0 = self.x0.alpha(target.x0, status.x0);
        let alpha_y0 = self.y0.alpha(target.y0, status.y0);
        let alpha_x1 = self.x1.alpha(target.x1, status.x1);
        let alpha_y1 = self.y1.alpha(target.y1, status.y1);

        // Use the minimum alpha value to ensure that all edges are
        // interpolated at the same pace
        let alpha = alpha_x0.min(alpha_y0).min(alpha_x1).min(alpha_y1);
        
        // Clamp alpha to [0, 1] range
        let a = alpha.max(0.0).min(1.0);
        println!("{a}");
        a
    }
}

impl Transit for Color {
    fn transit(self, target:Self, alpha:f64) -> Self {
        let self_into = self.as_rgba8();
        let target_into = target.as_rgba8();
        let diff_r = ((target_into.0 as i16 - self_into.0 as i16) as f64 * alpha) as i16;
        let diff_g = ((target_into.1 as i16 - self_into.1 as i16) as f64 * alpha) as i16;
        let diff_b = ((target_into.2 as i16 - self_into.2 as i16) as f64 * alpha) as i16;
        let diff_a = ((target_into.3 as i16 - self_into.3 as i16) as f64 * alpha) as i16;
        Color::rgba8( 
        (self_into.0 as i16+diff_r) as _, 
        (self_into.1 as i16+diff_g) as _,
        (self_into.2 as i16+diff_b) as _,
        (self_into.3 as i16+diff_a) as _
        )
    }

    fn alpha(self, target:Self, status:Self) -> f64 {
        let (sr,sg,sb,sa) = self.as_rgba8();
        let (tr,tg,tb,ta) = target.as_rgba8();
        let (str,stg,stb,sta) = status.as_rgba8();

        // let min_diff = str.abs_diff( tr ).abs_diff( sr )
        // .min( stg.abs_diff( tg ).abs_diff( sg ) )
        // .min( stb.abs_diff( tb ).abs_diff( sb ) )
        // .min( sta.abs_diff( ta ).abs_diff( sa ) );

        // println!("mindiff col {} {:?} {:?} {:?}", min_diff, self, target, status);
        // min_diff as f64 / 255.

        // sr.alpha(tr,str)
        // .min( sg.alpha(tg, stg) )
        // .min( sb.alpha(tb, stb) )
        // .min( sa.alpha(ta, sta) )
        let max_alpha = sr.alpha(tr, str)
        .max(sg.alpha(tg, stg))
        .max(sb.alpha(tb, stb))
        .max(sa.alpha(ta, sta));
        
        max_alpha
    }
}

impl Transit for BorderStyle {
    fn transit(self, target:Self, alpha:f64) -> Self {
        BorderStyle { 
            width: self.width.transit(target.width, alpha),
            radius: self.radius.transit(target.radius, alpha),
            color: self.color.transit(target.color, alpha)
        }
    }

    fn alpha(self, target:Self, status:Self) -> f64 {
        self.width.alpha(target.width, status.width)
        .min( self.radius.alpha(target.radius, status.radius))
        .min( self.color.alpha(target.color, status.color) )
    }
}


#[derive(Debug,Clone,Copy)]
pub struct BorderStyle {
    pub width: f64,
    pub radius : f64,
    pub color: Color,
}

impl BorderStyle {
    pub fn new(width:f64, radius:f64, color:impl Into<Color>) -> Self {
        Self { width, radius , color : color.into()}
    }
}


impl Default for BorderStyle {
    fn default() -> Self {
        Self { width: 1f64, radius:0f64, color: Color::rgb8(0,0,0) }
    }
}

#[derive(Debug,Clone,Copy)]
pub enum Pseudo {
	Focus,
	Hover,
	Active,
	Disabled
}

pub struct PseudoStyle {
	pub pseudo : Pseudo,
	pub style : Styler
}

impl PseudoStyle {
	pub fn hover(src:Styler) -> Self {
		Self {pseudo:Pseudo::Hover, style:src}
	}

	pub fn focus(src:Styler) -> Self {
		Self {pseudo:Pseudo::Focus, style:src}
	}

	pub fn active(src:Styler) -> Self {
		Self {pseudo:Pseudo::Active, style:src}
	}

	pub fn disabled(src:Styler) -> Self {
		Self {pseudo:Pseudo::Disabled, style:src}
	}
}

#[derive(Debug)]
pub struct Styler {
    pub padding : (Option<Insets>,Option<AnimationState>),
    pub margin : (Option<Insets>,Option<AnimationState>),
    pub font_size : (Option<f64>,Option<AnimationState>),
    pub width : (Option<f64>,Option<AnimationState>),
    pub height : (Option<f64>,Option<AnimationState>),
    pub text_color : (Option<Color>,Option<AnimationState>),
    pub background_color : (Option<Color>,Option<AnimationState>),
    pub border : (Option<BorderStyle>,Option<AnimationState>),
}

#[derive(Clone,Debug)]
pub struct Style {
	pub padding : Insets,
    pub margin : Insets,
    pub font_size : f64,
    pub width : Option<f64>,
    pub height : Option<f64>,
    pub text_color : Color,
    pub background_color : Color,
    pub border : BorderStyle,
}

impl Style {
    pub fn composite_transit(&self, elapsed:i64, target:&mut Styler, default_target:&mut Styler, out:&mut Style) -> (bool,bool,bool) {
        let mut layout_updated = false;
        let mut paint_updated = false;
        let mut has_next_anim = false;

        macro_rules! composite {
            ($item:ident) => { {
                let style = target.$item.0.as_mut().or( default_target.$item.0.as_mut() );
                let anim = target.$item.1.as_mut().or( default_target.$item.1.as_mut() );
                match (style, anim) {
                    ( Some(target_style), Some(target_anim) ) => {
                        let transit = target_anim.transit( self.$item, target_style.clone(), elapsed);
                        out.$item = transit.1.into();
                        (true, transit.0)
                    }
                    ( Some(target_style), None) => {
                        out.$item = target_style.clone().into();
                        (true, false)
                    }
                    _ => (false, false)
                }
            } }
        }

        let result = composite!( padding );
        layout_updated |= result.0;
        paint_updated |= result.0;
        has_next_anim |= result.1;

        let result = composite!( margin );
        layout_updated |= result.0;
        paint_updated |= result.0;
        has_next_anim |= result.1;

        let result = composite!( font_size );
        layout_updated |= result.0;
        paint_updated |= result.0;
        has_next_anim |= result.1;

        // let result = transit_style!( width );
        // layout_updated |= result.0;
        // paint_updated |= result.0;
        // has_next_anim |= result.1;

        // let result = transit_style!( height );
        // layout_updated |= result.0;
        // paint_updated |= result.0;
        // has_next_anim |= result.1;

        let result = composite!( text_color );
        paint_updated |= result.0;
        has_next_anim |= result.1;

        let result = composite!( background_color );
        paint_updated |= result.0;
        has_next_anim |= result.1;

        let result = composite!( border );
        paint_updated |= result.0;
        has_next_anim |= result.1;

        (layout_updated, paint_updated, has_next_anim)
    }

    pub fn transit(&self, elapsed:i64, target_style:&Style, target:&mut Styler, mut default_styler:Option<&mut Styler>, out:&mut Style) -> (bool,bool,bool) {
        let mut layout_updated = false;
        let mut paint_updated = false;
        let mut has_next_anim = false;

        macro_rules! transit_style {
            ($item:ident) => {
                match &mut target.$item {
                    ( _, Some(target_anim) ) => {
                        let transit = target_anim.transit( self.$item, target_style.$item.clone(), elapsed);
                        out.$item = transit.1.into();
                        // println!("myanim {} my:{:?} target:{:?} transited:{:?}", stringify!($item), self.$item, target_style.$item, transit);
                        (true, transit.0)
                    }
                    ( _, None) => {
                        if let Some( default_styler ) = default_styler.as_mut() {
                            if let (_,Some(target_anim)) = &mut default_styler.$item {
                                let transit = target_anim.transit( self.$item, target_style.$item.clone(), elapsed);
                                out.$item = transit.1.into();
                                // println!("alter anim {} {:?}", stringify!($item), transit);
                                (true, transit.0)
                            } else {
                                // println!("what");
                                out.$item = target_style.$item.clone().into();
                                (true, false)
                            }
                        } else {
                            out.$item = target_style.$item.clone().into();
                            (true, false)
                        }
                    }
                    _ => (false, false)
                }
            };
            ($item:ident) => {
                match &mut target.$item {
                    ( Some(target_style), Some(target_anim) ) => {
                        let transit = target_anim.transit( self.$item, target_style.clone(), elapsed);
                        out.$item = transit.1.into();
                        (true, transit.0)
                    }
                    ( Some(target_style), None) => {
                        out.$item = target_style.clone().into();
                        (true, false)
                    }
                    _ => (false, false)
                }
            }
        }

        let result = transit_style!( padding );
        layout_updated |= result.0;
        paint_updated |= result.0;
        has_next_anim |= result.1;

        let result = transit_style!( margin );
        layout_updated |= result.0;
        paint_updated |= result.0;
        has_next_anim |= result.1;

        let result = transit_style!( font_size );
        // println!("{elapsed} {} {} {:?}", out.font_size, self.font_size, target.font_size);
        layout_updated |= result.0;
        paint_updated |= result.0;
        has_next_anim |= result.1;

        // let result = transit_style!( width );
        // layout_updated |= result.0;
        // paint_updated |= result.0;
        // has_next_anim |= result.1;

        // let result = transit_style!( height );
        // layout_updated |= result.0;
        // paint_updated |= result.0;
        // has_next_anim |= result.1;

        let result = transit_style!( text_color );
        paint_updated |= result.0;
        has_next_anim |= result.1;

        let result = transit_style!( background_color );
        paint_updated |= result.0;
        has_next_anim |= result.1;

        let result = transit_style!( border );
        paint_updated |= result.0;
        has_next_anim |= result.1;

        (layout_updated, paint_updated, has_next_anim)
    }
}

impl Styler {
    pub fn composite_styles<'a, I:Iterator<Item=&'a Styler>>(&self, iter:I) -> Style {
        macro_rules! composite {
            ($styler:ident, $item:ident) => {
                if let (Some(style),_) = $styler.$item {
                    style
                } else {
                    $item
                }
            }
        }
        let mut padding = self.get_padding().unwrap_or_default();
		let mut margin = self.get_margin().unwrap_or_default();
		let mut font_size = self.get_font_size().unwrap_or( 14. );
		let mut width = self.get_width();
		let mut height = self.get_height();
		let mut text_color = self.get_text_color().unwrap_or( Color::rgba8(0, 0, 0, 255) );
		let mut background_color = self.get_background_color().unwrap_or( Color::rgba8(0, 0, 0, 0) );
		let mut border = self.get_border().unwrap_or( BorderStyle::new(0., 0., Color::rgba8(0,0,0,0)) );
        for style in iter {
            padding = composite!( style, padding );
            margin = composite!( style, margin );
            font_size = composite!( style, font_size );
            // width = composite!( style, width );
            // height = composite!( style, height );
            text_color = composite!( style, text_color );
            background_color = composite!( style, background_color );
            border = composite!( style, border );
        }
        Style {
			padding,
			margin,
			font_size,
			width,
			height,
			text_color,
			background_color,
			border,
		}
    }

    pub fn set_state_from_style(&mut self, start:&Style, end:&Style, curr:&Style) {
        macro_rules! set_anim_state {
            ($item:ident) => {
                if let (_,Some(ref mut anim)) = self.$item {
                    
                    let alpha = start.$item.alpha( end.$item, curr.$item );
                    anim.elapsed = (anim.anim.duration as f64 * alpha) as i64;
                    // println!("{} {:?}", stringify!($item), start.$item.transit(end.$item, alpha) );
                    // println!("{} start:{:?} end:{:?} status:{:?} => {} {}", stringify!($item), start.$item, end.$item, curr.$item, alpha, anim.elapsed);
                }
            }
        }
        set_anim_state!(padding);
        set_anim_state!(margin);
        set_anim_state!(font_size);
        // set_anim_state!(width);
        // set_anim_state!(height);
        set_anim_state!(text_color);
        set_anim_state!(background_color);
        set_anim_state!(border);
    }

    pub fn get_padding(&self) -> Option<Insets> {
        self.padding.0
    }

    pub fn get_margin(&self) -> Option<Insets> {
        self.margin.0
    }

    pub fn get_font_size(&self) -> Option<f64> {
        self.font_size.0
    }

    pub fn get_width(&self) -> Option<f64> {
        self.width.0
    }

    pub fn get_height(&self) -> Option<f64> {
        self.height.0
    }

    pub fn get_text_color(&self) -> Option<Color> {
        self.text_color.0
    }

    pub fn get_background_color(&self) -> Option<Color> {
        self.background_color.0
    }

    pub fn get_border(&self) -> Option<BorderStyle> {
        self.border.0.clone()
    }

    pub fn transit(&mut self, elapsed:i64, styler:&mut Styler, start:&Style, build:&mut Style) -> (bool,bool,bool) {
        let mut layout_updated = false;
        let mut paint_updated = false;
        let mut has_next_anim = false;

        macro_rules! transit_style {
            ($item:ident) => {
                match (&mut self.$item, &mut styler.$item) {
                    ( (Some(my_style), Some(my_anim)), (Some(target_style), _) ) => {
                        let transit = my_anim.transit(my_style.clone(), target_style.clone(), elapsed);
                        build.$item = transit.1.into();
                        (true, transit.0)
                    }
                    ( (Some(_), None), (Some(target_style), None) ) => {
                        build.$item = target_style.clone().into();
                        (true, false)
                    }
                    _ => (false, false)
                }
            }
        }

        let result = transit_style!( padding );
        layout_updated |= result.0;
        paint_updated |= result.0;
        has_next_anim |= result.1;

        let result = transit_style!( margin );
        layout_updated |= result.0;
        paint_updated |= result.0;
        has_next_anim |= result.1;

        let result = transit_style!( font_size );
        layout_updated |= result.0;
        paint_updated |= result.0;
        has_next_anim |= result.1;

        let result = transit_style!( width );
        layout_updated |= result.0;
        paint_updated |= result.0;
        has_next_anim |= result.1;

        let result = transit_style!( height );
        layout_updated |= result.0;
        paint_updated |= result.0;
        has_next_anim |= result.1;

        let result = transit_style!( text_color );
        paint_updated |= result.0;
        has_next_anim |= result.1;

        let result = transit_style!( background_color );
        paint_updated |= result.0;
        has_next_anim |= result.1;

        let result = transit_style!( border );
        paint_updated |= result.0;
        has_next_anim |= result.1;

        (layout_updated, paint_updated, has_next_anim)
    }

}



#[cfg(test)]
mod test {
    use druid::Insets;

    use crate::simple_style::{Styler, Animation, Direction, TimingFunction, AnimationState};

    // #[test]
    // fn calc_test() {
    //     let anim = Animation { delay: 0., direction: Direction::Alternate, duration: 2000_000_000, iteration: 1., name: 1., timing_function: TimingFunction::Linear, fill_mode: 1. };
    //     let anim_state = AnimationState::from( anim );
    //     let mut styler = Styler {
    //         padding: ( Some( Insets { x0: 10., y0: 10., x1: 20., y1: 20. } ), Some(anim_state.clone()) ),
    //         margin: (None,None),
    //         font_size: ( Some(12.), Some(anim_state.clone())),
    //         width: (None,None),
    //         height: (None,None),
    //         text_color: (None,None),
    //         background_color: (None,None),
    //         border: (None,None),
    //     };

    //     println!("Get Initial : {:?}", styler.get_padding());

    //     //animation 50%
    //     let target = Some( Insets { x0: 20., y0: 20., x1: 40., y1: 40. } );
    //     let transit = styler.get_padding_with_anim( 1000_000_000, target);
    //     println!("+50%(=50%) progress forward : {:?}",  transit);
    //     assert_eq!( transit.into(), (true,Some(Insets::new(15., 15., 30., 30.))) );

    //     //animation 50% (with keep state)
    //     let transit = styler.get_padding_with_anim( 1000_000_000, target);
    //     println!("+50%(=100%) progress forward : {:?}",  transit);
    //     assert_eq!( transit.into(), (false,Some(Insets::new(20., 20., 40., 40.))) );

    //     //animation overflowing
    //     let transit = styler.get_padding_with_anim( 1000_000_000, target);
    //     println!("+50%(=150% but keeped 100%) progress forward : {:?}",  transit);
    //     assert_eq!( transit.into(), (false,Some(Insets::new(20., 20., 40., 40.))) );

    //     //backward 50% (current status is 100%)
    //     let transit = styler.get_padding_with_anim( -1000_000_000, target);
    //     println!("-50%(will be 50%) progress forward : {:?}",  transit);
    //     assert_eq!( transit.into(), (true,Some(Insets::new(15., 15., 30., 30.))) );

    //     let target = Some( 24. );
    //     let transit = styler.get_font_size_with_anim( 1000_000_000, target);
    //     println!("+50%(=50%) progress forward : {:?}",  transit);
    //     //assert_eq!( transit.into(), (true,Some(Insets::new(15., 15., 30., 30.))) );
    // }
}