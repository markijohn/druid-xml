
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
    pub fn parse<'a>(mut stack:impl Iterator<Item=&'a str>) -> Result<Self,InvalidNumberError> {
        let rel = stack.next().ok_or_else(|| InvalidNumberError)?;
        let op = stack.next().ok_or_else(|| InvalidNumberError)?;
        let abs = stack.next().ok_or_else(|| InvalidNumberError)?;

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