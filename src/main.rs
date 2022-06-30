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

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            for s in (2..13).step_by(2) {
                print_spannable(s);
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
