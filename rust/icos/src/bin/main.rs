use icos::{alpha, Angle, Norm, Val};
use num_traits::ToPrimitive;

fn main() {
    let mut step = 1;
    let max_steps = 60;

    let fifth_turn = Angle::turn().div(&5.into());

    let mut t: Val = 1.into();
    let mut adjust = t.clone().div(&2.into());

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

        t = if delta.to_f64().unwrap() > 0.0 {
            t.add(&adjust)
        } else {
            t.sub(&adjust)
        };
        adjust = adjust.div(&2.into());

        step += 1;
        if step > max_steps {
            break;
        }
    }
}
