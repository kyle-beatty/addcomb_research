use std::rc::Rc;
use addcomb_comp::exactset::GElem;
use addcomb_comp::setlike::SetLike;
use addcomb_comp::exactset;
//use addcomb_comp::comb::chapter_d::rho;
use addcomb_comp::comb::chapter_d::rho_signed;

pub fn noah_main(args : &Vec<String>) {
    match args.len() {
        5 => {
            let n = args[2].parse().expect("arguments must be integers");
            let m = args[3].parse().expect("arguments must be integers");
            let h = args[4].parse().expect("arguments must be integers");

            let g = Rc::new(vec![n]);
            // verbosely compute rho
            let value = rho_signed::<Vec<GElem>>(g, m, h, true);
            println!("\nrho(Z_{}, {}, {}) = {}", n, m, h, value);
        },
        6 => {
            let n = args[2].parse().expect("arguments must be integers");
            let m = args[3].parse().expect("arguments must be integers");
            let h = args[4].parse().expect("arguments must be integers");
            let rho = args[5].parse().expect("arguments must be integers");
            print_subsets(n, m, h, rho);
        },
        _ => {
            println!("Noah main default");
        }
    }
}

pub fn print_subsets(n : u32, m : u32, h : u32, rho : u32) {
    println!(
        "Subsets A of Z_{} of size {} with {}-fold signed sumsets of size {};",
             n, m, h, rho);
    let g = Rc::new(vec![n]);
    //Note suBset and suMset are different
    for subset in <Vec<GElem> as SetLike>::each_set_exact(g.clone(), m) {
        let sumset =
            exactset::hfold_interval_signed_sumset(&subset, (h,h), g.clone());
        let size = sumset.len() as u32;
        if size == rho {
            println!("{:?};", subset);
        } else if size < rho {
            println!("LESS THAN RHO;");
            println!("{:?};", subset);
        }
    }

    println!("")
}
