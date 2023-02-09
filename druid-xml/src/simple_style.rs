#![allow(arithmetic_overflow)]

use std::{rc::Rc, ops::{Deref, DerefMut}, time::Duration};

use druid::{Size, Insets, Color, Rect, piet::StrokeStyle};

use crate::{curve::AnimationCurve};

#[derive(Clone,Copy)]
pub enum JumpTerm {
    JumpStart, //Denotes a left-continuous function, so that the first jump happens when the animation begins
    JumpEnd, //Denotes a right-continuous function, so that the last jump happens when the animation ends
    JumpNone, //There is no jump on either end. Instead, holding at both the 0% mark and the 100% mark, each for 1/n of the duration
    JumpBoth, //Includes pauses at both the 0% and 100% marks, effectively adding a step during the animation iteration
    Start, //Same as jump-start
    End //Same as jump-end
}

#[derive(Clone,Copy)]
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

#[derive(Clone)]
pub enum Direction {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse
}

#[derive(Clone)]
pub struct Animation {
    pub delay : f64, //delay for start
    pub direction : Direction, //when animation is end how to start
    pub duration : i64, //animation time in one cycle. actually this is the like animation speed (nanosecond)
    pub iteration : f64, //how many repeat animation
    pub name : f64, //animation progression state
    pub timing_function : TimingFunction, //timinig function
    pub fill_mode : f64, //how to fill when animation start/end
}

#[derive(Clone)]
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

        let alpha = self.anim.timing_function.translate( self.elapsed as f64 / self.anim.duration as f64 );
        //println!("alpha {} {} {}", self.elapsed, self.anim.duration, alpha);

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
}

impl Transit for f64 {
    fn transit(self, target:Self, alpha:f64) -> Self {
        let diff = target - self;
        self + diff * alpha
    }

    
}

impl Transit for Insets {
    fn transit(self, target:Self, alpha:f64) -> Self {
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
}

impl Transit for BorderStyle {
    fn transit(self, target:Self, alpha:f64) -> Self {
        BorderStyle { 
            width: self.width.transit(target.width, alpha),
            radius: self.radius.transit(target.radius, alpha),
            color: self.color.transit(target.color, alpha)
        }
    }
}

impl <T:Transit> Transit for Option<T> {
    fn transit(self, target:Self, alpha:f64) -> Self {
        match (self, target) {
            (None, None) => None,
            (None, Some(t)) => Some(t),
            (Some(s), None) => Some(s),
            (Some(s), Some(t)) => Some(s.transit(t,alpha)),
        }
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

#[derive(Clone,Copy)]
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

#[derive(Clone)]
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

        macro_rules! transit_style {
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

    pub fn transit(&self, elapsed:i64, target:&mut Styler, out:&mut Style) -> (bool,bool,bool) {
        let mut layout_updated = false;
        let mut paint_updated = false;
        let mut has_next_anim = false;

        macro_rules! transit_style {
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
		let mut border = self.get_border().unwrap_or_default();
        for style in iter {
            composite!( style, padding );
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

    pub fn set_progress_state(&mut self, state:f64) {
        macro_rules! clear_state {
            ($item:ident) => {
                if let (_,Some(anim)) = self.$item {
                    anim.elapsed = (anim.anim.duration as f64 * state) as _;
                }
            }
        }
        clear_state!(padding);
        clear_state!(margin);
        clear_state!(font_size);
        clear_state!(width);
        clear_state!(height);
        clear_state!(text_color);
        clear_state!(background_color);
        clear_state!(border);
    }

    pub fn apply_style(&self, start:&Style, end:&Style, curr:&mut Style, elapsed:i64, duration:i64) {
        let alpha = elapsed as f64 / duration as f64;
        curr.transit()
    }


    pub fn get_padding(&self) -> Option<Insets> {
        self.padding.0
    }

    pub fn get_padding_with_anim(&mut self, elapsed:i64, target:Option<Insets>) -> StyleQueryResult<Insets> {
        if let (Some(p), anim  ) = &mut self.padding {
            match (anim,target) {
                (_, None) => {
                    StyleQueryResult::some(false, *p)
                }
                (None, Some(target)) => {
                    if elapsed <= 0 {
                        StyleQueryResult::some(false, *p)
                    } else {
                        StyleQueryResult::some(false, target)
                    }
                }
                (Some(anim), Some(target)) => {
                    let transit = anim.transit(*p, target, elapsed);
                    StyleQueryResult::some(transit.0, transit.1)
                }
            }
        } else {
            StyleQueryResult::none(false)
        }
    }

    pub fn get_margin(&self) -> Option<Insets> {
        self.margin.0
    }

    pub fn get_margin_with_anim(&mut self, elapsed:i64, target:Option<Insets>) -> StyleQueryResult<Insets> {
        if let (Some(p), anim  ) = &mut self.margin {
            match (anim,target) {
                (_, None) => {
                    StyleQueryResult::some(false, *p)
                }
                (None, Some(target)) => {
                    if elapsed <= 0 {
                        StyleQueryResult::some(false, *p)
                    } else {
                        StyleQueryResult::some(false, target)
                    }
                }
                (Some(anim), Some(target)) => {
                    let transit = anim.transit(*p, target, elapsed);
                    StyleQueryResult::some(transit.0, transit.1)
                }
            }
        } else {
            StyleQueryResult::none(false)
        }
    }

    pub fn get_font_size(&self) -> Option<f64> {
        self.font_size.0
    }

    pub fn get_font_size_with_anim(&mut self, elapsed:i64, target:Option<f64>) -> StyleQueryResult<f64> {
        if let (Some(p), anim  ) = &mut self.font_size {
            match (anim,target) {
                (_, None) => {
                    StyleQueryResult::some(false, *p)
                }
                (None, Some(target)) => {
                    if elapsed <= 0 {
                        StyleQueryResult::some(false, *p)
                    } else {
                        StyleQueryResult::some(false, target)
                    }
                }
                (Some(anim), Some(target)) => {
                    let transit = anim.transit(*p, target, elapsed);
                    StyleQueryResult::some(transit.0, transit.1)
                }
            }
        } else {
            StyleQueryResult::none(false)
        }
    }

    pub fn get_width(&self) -> Option<f64> {
        self.width.0
    }

    pub fn get_width_with_anim(&mut self,elapsed:i64, target:Option<f64>) -> StyleQueryResult<f64> {
        if let (Some(p), anim  ) = &mut self.width {
            match (anim,target) {
                (_, None) => {
                    StyleQueryResult::some(false, *p)
                }
                (None, Some(target)) => {
                    if elapsed <= 0 {
                        StyleQueryResult::some(false, *p)
                    } else {
                        StyleQueryResult::some(false, target)
                    }
                }
                (Some(anim), Some(target)) => {
                    let transit = anim.transit(*p, target, elapsed);
                    StyleQueryResult::some(transit.0, transit.1)
                }
            }
        } else {
            StyleQueryResult::none(false)
        }
    }

    pub fn get_height(&self) -> Option<f64> {
        self.height.0
    }

    pub fn get_height_with_anim(&mut self,elapsed:i64, target:Option<f64>) -> StyleQueryResult<f64> {
        if let (Some(p), anim  ) = &mut self.height {
            match (anim,target) {
                (_, None) => {
                    StyleQueryResult::some(false, *p)
                }
                (None, Some(target)) => {
                    if elapsed <= 0 {
                        StyleQueryResult::some(false, *p)
                    } else {
                        StyleQueryResult::some(false, target)
                    }
                }
                (Some(anim), Some(target)) => {
                    let transit = anim.transit(*p, target, elapsed);
                    StyleQueryResult::some(transit.0, transit.1)
                }
            }
        } else {
            StyleQueryResult::none(false)
        }
    }

    pub fn get_text_color(&self) -> Option<Color> {
        self.text_color.0
    }

    pub fn get_text_color_with_anim(&mut self, elapsed:i64, target:Option<Color>) -> StyleQueryResult<Color> {
        if let (Some(p), anim  ) = &mut self.text_color {
            match (anim,target) {
                (_, None) => {
                    StyleQueryResult::some(false, *p)
                }
                (None, Some(target)) => {
                    if elapsed <= 0 {
                        StyleQueryResult::some(false, *p)
                    } else {
                        StyleQueryResult::some(false, target)
                    }
                }
                (Some(anim), Some(target)) => {
                    let transit = anim.transit(*p, target, elapsed);
                    StyleQueryResult::some(transit.0, transit.1)
                }
            }
        } else {
            StyleQueryResult::none(false)
        }
    }

    pub fn get_background_color(&self) -> Option<Color> {
        self.background_color.0
    }

    pub fn get_background_color_with_anim(&mut self, elapsed:i64, target:Option<Color>) -> StyleQueryResult<Color> {
        if let (Some(p), anim  ) = &mut self.background_color {
            match (anim,target) {
                (_, None) => {
                    StyleQueryResult::some(false, *p)
                }
                (None, Some(target)) => {
                    if elapsed <= 0 {
                        StyleQueryResult::some(false, *p)
                    } else {
                        StyleQueryResult::some(false, target)
                    }
                }
                (Some(anim), Some(target)) => {
                    let transit = anim.transit(*p, target, elapsed);
                    StyleQueryResult::some(transit.0, transit.1)
                }
            }
        } else {
            StyleQueryResult::none(false)
        }
    }

    pub fn get_border(&self) -> Option<BorderStyle> {
        self.border.0.clone()
    }

    pub fn get_border_with_anim(&mut self, elapsed:i64, target:Option<BorderStyle>) -> StyleQueryResult<BorderStyle> {
        if let (Some(p), anim  ) = &mut self.border {
            match (anim,target) {
                (_, None) => {
                    StyleQueryResult::some(false, p.clone())
                }
                (None, Some(target)) => {
                    if elapsed <= 0 {
                        StyleQueryResult::some(false, p.clone())
                    } else {
                        StyleQueryResult::some(false, target)
                    }
                }
                (Some(anim), Some(target)) => {
                    let transit = anim.transit(p.clone(), target.clone(), elapsed);
                    StyleQueryResult::some(transit.0, transit.1)
                }
            }
        } else {
            StyleQueryResult::none(false)
        }
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

    #[test]
    fn calc_test() {
        let anim = Animation { delay: 0., direction: Direction::Alternate, duration: 2000_000_000, iteration: 1., name: 1., timing_function: TimingFunction::Linear, fill_mode: 1. };
        let anim_state = AnimationState::from( anim );
        let mut styler = Styler {
            padding: ( Some( Insets { x0: 10., y0: 10., x1: 20., y1: 20. } ), Some(anim_state.clone()) ),
            margin: (None,None),
            font_size: ( Some(12.), Some(anim_state.clone())),
            width: (None,None),
            height: (None,None),
            text_color: (None,None),
            background_color: (None,None),
            border: (None,None),
        };

        println!("Get Initial : {:?}", styler.get_padding());

        //animation 50%
        let target = Some( Insets { x0: 20., y0: 20., x1: 40., y1: 40. } );
        let transit = styler.get_padding_with_anim( 1000_000_000, target);
        println!("+50%(=50%) progress forward : {:?}",  transit);
        assert_eq!( transit.into(), (true,Some(Insets::new(15., 15., 30., 30.))) );

        //animation 50% (with keep state)
        let transit = styler.get_padding_with_anim( 1000_000_000, target);
        println!("+50%(=100%) progress forward : {:?}",  transit);
        assert_eq!( transit.into(), (false,Some(Insets::new(20., 20., 40., 40.))) );

        //animation overflowing
        let transit = styler.get_padding_with_anim( 1000_000_000, target);
        println!("+50%(=150% but keeped 100%) progress forward : {:?}",  transit);
        assert_eq!( transit.into(), (false,Some(Insets::new(20., 20., 40., 40.))) );

        //backward 50% (current status is 100%)
        let transit = styler.get_padding_with_anim( -1000_000_000, target);
        println!("-50%(will be 50%) progress forward : {:?}",  transit);
        assert_eq!( transit.into(), (true,Some(Insets::new(15., 15., 30., 30.))) );

        let target = Some( 24. );
        let transit = styler.get_font_size_with_anim( 1000_000_000, target);
        println!("+50%(=50%) progress forward : {:?}",  transit);
        //assert_eq!( transit.into(), (true,Some(Insets::new(15., 15., 30., 30.))) );
    }
}