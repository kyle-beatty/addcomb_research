use addcomb_comp::setlike::SetLike;
use addcomb_comp::exactset;

use std::rc::Rc;
use addcomb_comp::exactset::GElem;

pub fn cass_main(args : &Vec<String>) {
    match args.len() {
        /*
        3 => {
            let s : u32 = args[3].parse().expect("arguments must be integers");
            println!("[0,{}] signed spanning sets;", s);

            let lower_n = 2 * (s+1);
            let upper_n = 

            // Anything below lower bound is spanned by 1, uninteresting
            for n in (2*s + 2)..(max_n+1) {
                print!("Z_{}; ", n);
                match do_three_elements_span(n, s) {
                    Some(A) => {
                        println!("{:?};", A)
                    },
                    None => {
                        println!("No spanning set");
                    }
                }
            }
        },
        */
        4 => {
            let max_n : u32 = args[2].parse().expect("arguments must be integers");
            let s : u32 = args[3].parse().expect("arguments must be integers");
            println!("[0,{}] signed spanning sets;", s);

            // Anything below lower bound is spanned by 1, uninteresting
            for n in (2*s + 2)..(max_n+1) {
                print!("Z_{}; ", n);
                match do_three_elements_span(n, s) {
                    Some(A) => {
                        println!("{:?};", A)
                    },
                    None => {
                        println!("No spanning set");
                    }
                }
            }
        },
        _ => { println!("Wrong number of arguments given"); },
    }
}

pub fn do_three_elements_span(n: u32, s: u32) -> Option<Vec<GElem>> {
    let g = Rc::new(vec![n]);

    for A in <Vec<GElem> as SetLike>::each_set_exact_no_zero(g.clone(), 3) {
        if does_subset_span(n, s, &g, &A) {
            return Some(A);
        }
    }

    None
}

pub fn does_subset_span(n : u32, s: u32, g: &Rc<Vec<u32>>, A: &Vec<GElem>)
    -> bool {
        //println!("{:?}", A);
        
        let span = exactset::hfold_interval_signed_sumset(&A, (0, s), g.clone());

        //println!("span: {:?}", span);
        //
        //TODO find out weird behavior with extra 3D "zero" element
        // that makes this necessary
        span.len() as u32 == (n+1)
}
