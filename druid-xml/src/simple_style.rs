use std::{rc::Rc, ops::{Deref, DerefMut}, time::Duration};

use druid::{Size, Insets, Color, Rect, piet::StrokeStyle};

#[derive(Clone)]
pub enum JumpTerm {
    JumpStart, //Denotes a left-continuous function, so that the first jump happens when the animation begins
    JumpEnd, //Denotes a right-continuous function, so that the last jump happens when the animation ends
    JumpNone, //There is no jump on either end. Instead, holding at both the 0% mark and the 100% mark, each for 1/n of the duration
    JumpBoth, //Includes pauses at both the 0% and 100% marks, effectively adding a step during the animation iteration
    Start, //Same as jump-start
    End //Same as jump-end
}

#[derive(Clone)]
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
    duration : u64, //animation time in one cycle. actually this is the speed (nanosecond)
    iteration : f64, //how many repeat animation
    name : f64, //animation progression state
    timing_function : TimingFunction, //timinig function
    fill_mode : f64, //how to fill when animation start/end
}

pub struct AnimationState {
    counted : u64,
    anim : Animation
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

    pub fn into(self) -> Option<T> {
        self.data
    }

    pub fn has_next_animation(&self) -> bool {
        self.is_animated
    }
}

trait Transit {
    /// `forward_dir` flag is linear forward
    /// `target` is the goal of transit
    /// `duration` is animation time
    /// `interval` how to elapsed time
    /// (bool,Self) first bool is reach the end. Self is calculate value
    fn transit(self, forward_dir:bool, target:Self, duration:u64, interval:u64) -> (bool,Self);
}

impl Transit for f64 {
    fn transit(self, forward_dir:bool, target:Self, duration:u64, interval:u64) -> (bool,Self) {
        if forward_dir {
            let total = target - self;
            let alpha = Duration::from_nanos(interval).as_secs_f64().min(duration) / duration;
            curr + total * alpha
        } else {
            let total = self - target;
        }
    }
}

impl Transit for Insets {
    fn transit(self, curr:Self, target:Self, duration:f64, interval:u64) -> Self {
        let diff_x0 = target.x0 - self.x0;
        let diff_y0 = target.y0 - self.y0;
        let diff_x1 = target.x1 - self.x1;
        let diff_y1 = target.y1 - self.y1;
        let alpha = Duration::from_nanos(interval).as_secs_f64().min(duration) / duration;
        // println!("inter  {} {}",Duration::from_nanos(interval).as_secs_f64(), duration);
        // println!("alpha  {}",alpha);
        // println!("diff  {} {} {} {}",diff_x0, diff_y0, diff_x1, diff_y1);
        Self { 
            x0: curr.x0 + diff_x0 * alpha, 
            y0: curr.y0 + diff_y0 * alpha, 
            x1: curr.x1 + diff_x1 * alpha, 
            y1: curr.y1 + diff_y1 * alpha 
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
    pub(crate) padding : (Option<Insets>,Option<Animation>),
    pub(crate) margin : (Option<Insets>,Option<Animation>),
    pub(crate) font_size : (Option<f64>,Option<Animation>),
    pub(crate) width : (Option<f64>,Option<Animation>),
    pub(crate) height : (Option<f64>,Option<Animation>),
    pub(crate) text_color : (Option<Color>,Option<Animation>),
    pub(crate) background_color : (Option<Color>,Option<Animation>),
    pub(crate) border : (Option<BorderStyle>,Option<Animation>),
}


impl Styler {
    pub fn get_padding(&self) -> Option<Insets> {
        self.padding.0
    }

    pub fn get_padding_with_anim(&mut self, is_forward:bool, time:u64, curr:Option<Insets>, target:Option<Insets>) -> StyleQueryResult<Insets> {
        if let Some(curr) = curr {
            if let (Some(p), ref anim) = self.padding {
                if let (Some(anim), Some(target)) = (anim,target) {
                    return StyleQueryResult::some(true, p.transit(curr, target, anim.duration, time));
                } else {
                    return StyleQueryResult::some(false, p);
                }
            } else {
                StyleQueryResult::none(false)
            }
        } else {
            StyleQueryResult::new( false, self.padding.0 )
        }
    }

    pub fn get_margin(&self) -> Option<Insets> {
        self.margin.0
    }

    pub fn get_margin_with_anim(&mut self, time:f64, base_margin:Insets) -> StyleQueryResult<Insets> {
        todo!()
    }

    pub fn get_font_size(&self) -> Option<f64> {
        self.font_size.0
    }

    pub fn get_font_size_with_anim(&mut self,time:f64, base_size:f64) -> StyleQueryResult<Color> {
        // for n in self.iter() {
        //     if let Target::FontSize(s) = n.target {
        //         let diff = base_size - s;
        //         if let Some(anim) = n.animation {
        //             return StyleQueryResult::some(true, p);
        //         } else {
        //             return StyleQueryResult::some(false, p);
        //         }
        //     }
        // }
        // StyleQueryResult::none(false)
        todo!()
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

    use crate::simple_style::{Styler, Animation, Direction, TimingFunction};

    #[test]
    fn calc_test() {
        let anim = Animation { delay: 0., direction: Direction::Alternate, duration: 2., iteration: 1., name: 1., timing_function: TimingFunction::Linear, fill_mode: 1. };
        let mut styler = Styler {
            padding: ( Some( Insets { x0: 10., y0: 10., x1: 20., y1: 20. } ), Some(anim.clone()) ),
            margin: (None,None),
            font_size: ( Some(12.), Some(anim.clone())),
            width: (None,None),
            height: (None,None),
            text_color: (None,None),
            background_color: (None,None),
            border: (None,None),
        };

        println!("Get Initial : {:?}", styler.get_padding());

        let curr = Some( Insets { x0: 10., y0: 10., x1: 20., y1: 20. } );
        //forward
        let target = Some( Insets { x0: 20., y0: 20., x1: 40., y1: 40. } );
        println!("Get 50% progress forward : {:?}", styler.get_padding_with_anim( true, 1000_000_000, curr, target) );
        assert_eq!( styler.get_padding_with_anim( true, 1000_000_000, curr, target).into(), Some(Insets::new(15., 15., 30., 30.)) );

        //reverse
        let target = Some( Insets { x0: 5., y0: 5., x1: 10., y1: 10. } );
        println!("Get 50% progress reverse : {:?}", styler.get_padding_with_anim( true, 1000_000_000, curr, target) );
        assert_eq!( styler.get_padding_with_anim( true, 1000_000_000, curr, target).into(), Some( Insets { x0: 7.5, y0: 7.5, x1: 15., y1: 15. } ) );
    }
}