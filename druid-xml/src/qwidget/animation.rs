

struct Animation {
    delay : f64, //delay for start
    direction : u8, //when animation is end how to start
    duration : f64, //animation time in one cycle
    iteration : f64, //how many repeat animation
    name : f64, //animation progression state
    timing_function : u8, //timinig function
    fill_mode : f64 //how to fill when animation start/end
}