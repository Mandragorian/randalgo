use rayon::prelude::*;

use rand::random;
use num_bigint::BigUint;
use num_traits::{Zero, One, ToPrimitive};

use fraction::{Fraction, Decimal};

use clap::{App, Arg};

fn create_parser() -> App<'static, 'static> {
    App::new("mocapi")
        .version("0.1")
        .author("mandragore")
        .about("Calculates pi using target shooting")
        .arg(
            Arg::with_name("points")
            .long("points")
            .default_value("1000000")
            )
        .arg(
            Arg::with_name("threads")
            .long("threads")
            .default_value("1")
            )
        .arg(
            Arg::with_name("precision")
            .long("precision")
            .default_value("5")
            )
}
        

fn main() -> Result<(), ()>{
    let matches = create_parser().get_matches();

    let one: BigUint = One::one();    

    let points: usize = matches.value_of("points").expect("No points").parse().or(Err(()))?;
    let tasks: usize = matches.value_of("threads").expect("No threads").parse().or(Err(()))?;
    let precision: usize = matches.value_of("precision").expect("No precision").parse().or(Err(()))?;

    let res: BigUint = (0..tasks).into_par_iter().map(|_| {
        let mut total: BigUint = Zero::zero();
        let mut circle: BigUint = Zero::zero();
        for _ in 0..(points / tasks) {
            let x = random::<f64>();
            let y = random::<f64>();

            let d = x * x + y * y;

            if d <= 1. {
                circle += &one;
            }
            total += &one;
        }
        circle
    }).sum();
    let f = Fraction::from(res);
    let f = f * Fraction::from(4);
    let pi = Decimal::from_fraction(f / Fraction::from(points));
    pi.set_precision(precision as u8);

    //let fmt = format!("{{:.{}}}", precision);
    println!("{:.15}", pi.to_f64().unwrap());
    Ok(())
}
