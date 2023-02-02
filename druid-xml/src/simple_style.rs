use std::{rc::Rc, ops::{Deref, DerefMut}, time::Duration};

use druid::{Size, Insets, Color, Rect, piet::StrokeStyle};

use crate::curve::AnimationCurve;

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
    delay : f64, //delay for start
    direction : Direction, //when animation is end how to start
    duration : i64, //animation time in one cycle. actually this is the speed (nanosecond)
    iteration : f64, //how many repeat animation
    name : f64, //animation progression state
    timing_function : TimingFunction, //timinig function
    fill_mode : f64, //how to fill when animation start/end
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
        println!("alpha {} {} {}", self.elapsed, self.anim.duration, alpha);

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
        // println!("inter  {} {}",Duration::from_nanos(interval).as_secs_f64(), duration);
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
        let self_rgba = self.as_rgba_u32();
        let diff_rgba = target.as_rgba_u32() - self_rgba;
        Color::from_rgba32_u32( self_rgba + (diff_rgba as f64 * alpha) as u32 )
    }
}

impl Transit for BorderStyle {
    fn transit(self, target:Self, alpha:f64) -> Self {
        let diff_width = target.width - self.width;
        let self_rgba = self.color.as_rgba_u32();
        let diff_rgba = target.color.as_rgba_u32() - self_rgba;
        BorderStyle { style: self.style, 
            width: self.width + diff_width * alpha,
            color: Color::from_rgba32_u32( self_rgba + (diff_rgba as f64 * alpha) as u32 )
        }
    }
}

#[derive(Clone)]
pub struct BorderStyle {
    pub style : StrokeStyle,
    pub width: f64,
    pub color: Color,
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

pub struct Styler {
    pub(crate) padding : (Option<Insets>,Option<AnimationState>),
    pub(crate) margin : (Option<Insets>,Option<AnimationState>),
    pub(crate) font_size : (Option<f64>,Option<AnimationState>),
    pub(crate) width : (Option<f64>,Option<AnimationState>),
    pub(crate) height : (Option<f64>,Option<AnimationState>),
    pub(crate) text_color : (Option<Color>,Option<AnimationState>),
    pub(crate) background_color : (Option<Color>,Option<AnimationState>),
    pub(crate) border : (Option<BorderStyle>,Option<AnimationState>),
}


impl Styler {
    pub fn get_padding(&self) -> Option<Insets> {
        self.padding.0
    }

    pub fn get_padding_with_anim(&mut self, elapsed:i64, target:Option<Insets>) -> StyleQueryResult<Insets> {
        if let (Some(p), anim  ) = &mut self.padding {
            if let (Some(anim), Some(target)) = (anim,target) {
                let transit = anim.transit(*p, target, elapsed);
                return StyleQueryResult::some(transit.0, transit.1);
            } else {
                return StyleQueryResult::some(false, *p);
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
            if let (Some(anim), Some(target)) = (anim,target) {
                let transit = anim.transit(*p, target, elapsed);
                return StyleQueryResult::some(transit.0, transit.1);
            } else {
                return StyleQueryResult::some(false, *p);
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
            if let (Some(anim), Some(target)) = (anim,target) {
                let transit = anim.transit(*p, target, elapsed);
                return StyleQueryResult::some(transit.0, transit.1);
            } else {
                return StyleQueryResult::some(false, *p);
            }
        } else {
            StyleQueryResult::none(false)
        }
    }

    pub fn get_width(&self) -> Option<f64> {
        self.width.0
    }

    pub fn get_width_with_anim(&mut self,time:f64, base_size:f64, content_size:f64) -> StyleQueryResult<Color> {
        todo!()
    }

    pub fn get_height(&self) -> Option<f64> {
        self.height.0
    }

    pub fn get_height_with_anim(&mut self,time:f64, base_size:f64, content_size:f64) -> StyleQueryResult<Color> {
        todo!()
    }

    pub fn get_text_color(&self) -> Option<Color> {
        self.text_color.0
    }

    pub fn get_text_color_with_anim(&mut self, base_color:Color) -> StyleQueryResult<Color> {
        todo!()
    }

    pub fn get_background_color(&self) -> Option<Color> {
        self.background_color.0
    }

    pub fn get_background_color_with_anim(&mut self, base_color:Color) -> StyleQueryResult<Color> {
        todo!()
    }

    pub fn get_border(&self) -> Option<BorderStyle> {
        self.border.0.clone()
    }

    pub fn get_border_with_anim(&mut self) -> StyleQueryResult<f64> {
        todo!()
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
        assert_eq!( transit.into(), (true,Some(Insets::new(20., 20., 40., 40.))) );

        //animation overflowing
        let transit = styler.get_padding_with_anim( 1000_000_000, target);
        println!("+50%(=150% but keeped 100%) progress forward : {:?}",  transit);
        assert_eq!( transit.into(), (false,Some(Insets::new(20., 20., 40., 40.))) );

        //backward 50% (current status is 100%)
        let transit = styler.get_padding_with_anim( -1000_000_000, target);
        println!("-50%(will be 50%) progress forward : {:?}",  transit);
        assert_eq!( transit.into(), (true,Some(Insets::new(15., 15., 30., 30.))) );
    }
}