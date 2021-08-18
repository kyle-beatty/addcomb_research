use std::env;
use std::rc::Rc;

use addcomb_comp::exactset;
use addcomb_comp::exactset::GElem;

use addcomb_research::spanning::print_elements_span;

// Test my conjecture for s odd
fn test_conj_odd(s : u32) {
    if s % 2 == 0 {
        return;
    }

    // generate the range of k
    //let start = 3 * s - 3;
    let start = (s+1) / 2;
    let end = (s * s + 1) / 2;

    // generate our spanning set for max k, to test it for smaller
    let x = match s % 4 {
        1 => (s+1)/2,
        3 => (s-1)/2,
        _ => {panic!("Something went terribly wrong with the spanning pair");}
    };

    let y = match s % 4 {
        1 => (s-1)/2,
        3 => (s+1)/2,
        _ => {panic!("Something went terribly wrong with the spanning pair");}
    };

    let a_1 = GElem(vec![0, x]);
    let a_2 = GElem(vec![1, y]);
    let a = vec![a_1, a_2];

    println!("s = {}, A = {:?}", s, a);

    for k in start..end {
        let g = Rc::new(vec![2, 2 * k]);
        let g_size = 4 * k;

        let sumset = exactset::hfold_interval_signed_sumset(&a, (0, s), g.clone());
        let size = sumset.len() as u32;
        
        if size == g_size {
            println!("  k = {:3}: success", k);
        } else {
            println!("X k = {:3}: FAILURE", k);
        }
    }

}


fn run(s : u32, k : u32) {
    print_elements_span(k, s);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => { run(5, 1) },
        2 => {
            let s = args[1].parse().expect("arguments must be integers");

            run(s, 1);
        },
        3 => {
            let s = args[1].parse().expect("arguments must be integers");
            let k = args[2].parse().expect("arguments must be integers");

            run(s, k);
        },
        _ => { println!("Wrong number of arguments given"); },
    }
}
