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
        _ => {
            println!("Junxue main default");
        }
    }
}
