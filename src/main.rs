use std::env;

use addcomb_research::spanning::*;

//use addcomb_comp::setlike::SetLike;
use addcomb_comp::exactset;
//use addcomb_comp::comb::gcd;

use std::rc::Rc;
use addcomb_comp::exactset::GElem;


/*
fn run(s : u32, k : u32) {
    println!("{:?}", do_two_elements_span(k, s));
}
*/

fn print_max_k(s : u32, pair : &Vec<GElem>) {
    for i in 0..(s * s / 2) {
        let k = (s * s / 2) - i;
        let g = Rc::new(vec![2, 2 * k]);
        let span =
            exactset::hfold_interval_signed_sumset(&pair, (0, s), g);

        if (span.len() as u32) == 4 * k {
            println!("max k = {}", k);
            break;
        }
    }
}

/*
 * For testing s^2 - s stuff
 */
fn print_lambdas(s : i32, a : i32) {
    let lower_l2 = (a / (2 * (s-2)) as i32) * 2;
    let upper_l2 = lower_l2 + 2;
    //println!("{} {}", lower_l2, upper_l2);
    let mu;

    if upper_l2*(s-2) - a < a - lower_l2*(s-2) {
         mu = vec![a - upper_l2*(s-2), upper_l2];
    } else {
         mu = vec![a - lower_l2*(s-2), lower_l2];
    }

    if mu[0].abs() + mu[1].abs() <= s {
        println!(" OG: ({:>3}, {:>3})", mu[0], mu[1]);
    } else {
        println!(" OG: ({:>3}, {:>3})  XX", mu[0], mu[1]);
        println!("NEW: ({:>3}, {:>3})", mu[0] + s - 4, mu[1] - s - 2);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            let s = 10;

            for a in 0..(s*s - s) {
                println!("a ={:>3}", a);
                print_lambdas(s, a);
            }
        },
        2 => {
            let s = args[1].parse().expect("arguments must be integers");

            //run(s, 1);
            print_spannable(s);
        },
        3 => {
            let s = args[1].parse().expect("arguments must be integers");
            let k = args[2].parse().expect("arguments must be integers");

            print_spanning_pairs(k, s);
        },
        6 => {
            let s = args[1].parse().expect("arguments must be integers");

            let a = args[2].parse().expect("arguments must be integers");
            let x = args[3].parse().expect("arguments must be integers");
            let b = args[4].parse().expect("arguments must be integers");
            let y = args[5].parse().expect("arguments must be integers");

            //TODO when defining pair use mod to fit too high values
            //into group
            let p = GElem(vec![a, x]);
            let q = GElem(vec![b, y]);
            let pair = vec![p, q];

            print_max_k(s, &pair);
        },
        7 => {
            let s = args[1].parse().expect("arguments must be integers");
            let k : u32 = args[2].parse().expect("arguments must be integers");

            let a = args[3].parse().expect("arguments must be integers");
            let x = args[4].parse().expect("arguments must be integers");
            let b = args[5].parse().expect("arguments must be integers");
            let y = args[6].parse().expect("arguments must be integers");

            let g = Rc::new(vec![2, 2 * k]);

            //TODO when defining pair use mod to fit too high values
            //into group
            let p = GElem(vec![a, x]);
            let q = GElem(vec![b, y]);
            let pair = vec![p, q];

            let span =
                exactset::hfold_interval_signed_sumset(&pair, (0, s), g.clone());

            let does_it_span = (span.len() as u32) == 4 * k;

            if does_it_span {
                println!("k ={:>3} The pair {:?} {}-spans Z_2 x Z_{}\n",
                         k, &pair, s, 2*k);
            } else {
                //println!("  {}-span of {:?}: {:?}\n", s, pair, span);
                println!("k ={:>3} The pair {:?} DOES NOT {}-span Z_2 x Z_{}\n",
                         k, &pair, s, 2*k);
            }
        },
        _ => { println!("Wrong number of arguments given"); },
    }

}
