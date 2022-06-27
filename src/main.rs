use std::env;

use addcomb_research::spanning::*;

fn run(s : u32, k : u32) {
    println!("{:?}", do_two_elements_span(k, s));
}

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
        _ => { println!("Wrong number of arguments given"); },
    }
}
