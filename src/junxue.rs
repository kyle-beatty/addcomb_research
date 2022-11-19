use addcomb_comp::setlike::SetLike;
use addcomb_comp::exactset;

use std::rc::Rc;
use addcomb_comp::exactset::GElem;

pub fn junxue_main(args : &Vec<String>) {
    match args.len() {
        4 => {
            let s : u32 = args[2].parse().expect("arguments must be integers");
            let k : u32 = args[3].parse().expect("arguments must be integers");
            let g = Rc::new(vec![3, 3 * k]);
            let g_size = 9 * k;

            println!("Signed {}-spanning pairs for Z_3 x Z_{};", s, 3 * k);

            for pair in <Vec<GElem> as SetLike>::each_set_exact(g.clone(), 2) {
                // Filter out first components of 2
                if (pair[0].0[0] == 2) || (pair[1].0[0] == 2) {
                    continue;
                }

                // Check if pair spans group
                let span = exactset::hfold_interval_signed_sumset(
                    &pair, (0, s), g.clone());
                let size = span.len() as u32;

                if size == g_size {
                    println!("{:?};", pair);
                }
            }
        },
        8 => {
            let s = args[2].parse().expect("arguments must be integers");
            let k : u32 = args[3].parse().expect("arguments must be integers");

            let a = args[4].parse().expect("arguments must be integers");
            let x = args[5].parse().expect("arguments must be integers");
            let b = args[6].parse().expect("arguments must be integers");
            let y = args[7].parse().expect("arguments must be integers");

            let g = Rc::new(vec![3, 3 * k]);

            let p = GElem(vec![a, x]);
            let q = GElem(vec![b, y]);
            let pair = vec![p, q];

            let span =
                exactset::hfold_interval_signed_sumset(&pair, (0, s), g.clone());

            let does_it_span = (span.len() as u32) == 9 * k;

            if does_it_span {
                println!("k ={:>3} The pair {:?} {}-spans Z_2 x Z_{}\n",
                         k, &pair, s, 2*k);
            } else {
                //println!("  {}-span of {:?}: {:?}\n", s, pair, span);
                println!("k ={:>3} The pair {:?} DOES NOT {}-span Z_2 x Z_{}\n",
                         k, &pair, s, 2*k);
            }
        },
        _ => {
            println!("Junxue main default");
        }
    }
}

pub fn do_two_elements_span(k: u32, s: u32) -> Option<Vec<GElem>> {
    let g = Rc::new(vec![3, 3 * k]);
    let _g_size = 9 * k;

    for pair in <Vec<GElem> as SetLike>::each_set_exact(g.clone(), 2) {
        if does_pair_span(k, s, &g, &pair) {
            return Some(pair);
        }
    }

    None
}

pub fn does_pair_span(k: u32, s: u32, g: &Rc<Vec<u32>>, pair: &Vec<GElem>) -> bool {
    let g_size = 9 * k;

    let span = exactset::hfold_interval_signed_sumset(&pair, (0, s), g.clone());
    let size = span.len() as u32;

    size >= g_size
}
