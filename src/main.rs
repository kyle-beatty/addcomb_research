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

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            let s = 8;
            let k = 6;
            println!("s = {}, k = {}", s, k);

            /*
            match pair_span_four(k, s) {
                Some(pair) => {
                    println!("Spanning pair: {:?}", pair);
                },
                None => {
                    println!("No spanning pair for (k, s) = ({}, {})", k, s);
                }
            }*/

            println!("Z_4 x Z_{}", 4 * k);
            print_pair_span_four(k, s);
            println!();
            println!("Z_2 x Z_{}", 8 * k);
            print_spanning_pairs(4 * k, s);
            println!();
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
        8 => {
            let s = args[1].parse().expect("arguments must be integers");
            let m : u32 = args[2].parse().expect("arguments must be integers");
            let k : u32 = args[3].parse().expect("arguments must be integers");

            let a = args[4].parse().expect("arguments must be integers");
            let x = args[5].parse().expect("arguments must be integers");
            let b = args[6].parse().expect("arguments must be integers");
            let y = args[7].parse().expect("arguments must be integers");

            let g = Rc::new(vec![m, m * k]);

            //TODO when defining pair use mod to fit too high values
            //into group
            let p = GElem(vec![a, x]);
            let q = GElem(vec![b, y]);
            let pair = vec![p, q];

            let span =
                exactset::hfold_interval_signed_sumset(&pair, (0, s), g.clone());

            let does_it_span = (span.len() as u32) == m * m * k;

            if does_it_span {
                println!("k ={:>3} The pair {:?} {}-spans Z_{} x Z_{}\n",
                         k, &pair, s, m, m*k);
            } else {
                //println!("  {}-span of {:?}: {:?}\n", s, pair, span);
                println!("k ={:>3} The pair {:?} DOES NOT {}-span Z_{} x Z_{}\n",
                         k, &pair, s, m, m*k);
            }
        },
        _ => { println!("Wrong number of arguments given"); },
    }

}
