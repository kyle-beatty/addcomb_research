//use addcomb_comp::setlike::SetLike;
use addcomb_comp::exactset;

use std::rc::Rc;
use addcomb_comp::exactset::GElem;

pub fn quentin_main(args : &Vec<String>) {
    match args.len() {
        4 => {
            let max_n : u32 = args[2].parse().expect("arguments must be integers");
            //let m : u32 = args[3].parse().expect("arguments must be integers");
            let h : u32 = args[3].parse().expect("arguments must be integers");
            let mut g;

            for n in (7..max_n+1).step_by(7) {
                println!("{}-fold exact sumset in Z_{};", h, n);

                g = Rc::new(vec![n]);

                // create subset A of the given cosets/congruence classes
                let mut subset = vec![];

                for i in (1..n).step_by(7) {
                        subset.push(GElem(vec![i]));
                }

                for i in (6..n).step_by(7) {
                        subset.push(GElem(vec![i]));
                }

                let sumset = exactset::hfold_interval_signed_sumset(
                    &subset, (h, h), g.clone());

                println!("{:?}; {:?};", subset, sumset);
            }
        },
        _ => {
            println!("Quentin main default");
        }
    }
}
