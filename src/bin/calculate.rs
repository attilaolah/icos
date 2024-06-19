use icos::{alpha, Angle, Norm, Val};
use num_traits::ToPrimitive;

fn main() {
    calculate()
}

fn calculate() {
    let mut step = 1;
    let max_steps = 60;

    let fifth_turn = Angle::turn().div(&5.into());

    let mut t: Val = 1.into();
    let mut adjust = t.clone();

    loop {
        let by = alpha().mul(&t).div(&2.into());

        let a = Norm::zero().south(&by);
        let b = Norm::zero().south(&alpha()).north(&by);
        let c = a.clone().east(&fifth_turn);

        let ab = a.clone().distance_to(b);
        let ac = a.clone().distance_to(c);
        let delta = ab.clone().sub(&ac);

        println!(
            "{:0.16}: {:0.16} - {:0.16} = {:+0.16}",
            t.to_f64().unwrap(),
            ab.clone().to_f64().unwrap(),
            ac.clone().to_f64().unwrap(),
            delta.to_f64().unwrap(),
        );

        adjust = adjust.div(&2.into());
        t = if delta.to_f64().unwrap() > 0.0 {
            t.add(&adjust)
        } else {
            t.sub(&adjust)
        };

        step += 1;
        if step > max_steps {
            break;
        }
    }
}

fn _print_formula() {
    let fifth_turn = Angle::turn().div(&5.into());

    let t = Val::param(1);
    let by = alpha().mul(&t).div(&2.into());

    let a = Norm::zero().south(&by);
    let b = Norm::zero().south(&alpha()).north(&by);
    let c = a.clone().east(&fifth_turn);

    let ab = a.clone().distance_to(b);
    let ac = a.clone().distance_to(c);
    let delta = ab.clone().sub(&ac);

    println!("function f({}) {} endfunction", t, delta);
}
