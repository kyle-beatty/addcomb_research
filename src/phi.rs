use addcomb_comp::setlike::SetLike;
use addcomb_comp::exactset;
use addcomb_comp::comb::gcd;

use std::rc::Rc;
use addcomb_comp::exactset::GElem;

/*
 * Finds spannable groups of given order
 */
pub fn find_spannable(n : u32, s : u32) {
    // We already have a theorem for all n_1 = 2, so will check later
    for n_1 in 3..(2*s + 1) {
        if n % (n_1 * n_1) != 0 {
            continue;
        }

        //TODO should this be a let?
        let k = n / (n_1 * n_1);
        match do_two_elements_span_general(n_1, k, s) {
            Some(pair) => {
                println!("  Z_{} x Z_{}: A = {:?}", n_1, n_1*k, pair);
            },
            None => {
                println!("  Z_{} x Z_{}: NOT SPANNED", n_1, n_1*k);
            }
        }
}
}

pub fn largest_group_spanned(s : u32) {
    println!("s = {}", s);
    let mut current_max : u32 = 4 * (s * s / 2);
    println!("Starting max: {}", current_max);
    let upper_bound = (s*s) + (s+1)*(s+1);
    println!("Upper bound: {}", upper_bound);

    for n_1 in 3..(2*s+1) {
        // We're not filtering these out yet
        //if (2*s + 1) % n_1 != 0 && n_1 != s {
        //    println!("Continue");
        //    continue;
        //}
        
        //println!("Testing n_1 = {}", n_1);

        for k in 1..(s*s) {
            if n_1 * n_1 * k <= current_max {
                continue;
            }

            if n_1 * n_1 * k > upper_bound {
                continue;
            }

            println!("Testing (n_1, k) = ({}, {})", n_1, k);
            match do_two_elements_span_general(n_1, k, s) {
                Some(pair) => {
                    current_max = n_1 * n_1 * k;
                    println!("New maximum: {}", current_max);
                    println!("  Group: Z_{} x Z_{}", n_1, n_1*k);
                    println!("  Pair : {:?}", pair);
                    println!();
                },
                None => {
                }
            }
        }
    }

    println!();
    println!("MAXIMUM for s = {} is |G| = {}\n", s, current_max);
}

pub fn do_two_elements_span_general(n_1 : u32, k : u32, s : u32)
    -> Option<Vec<GElem>> {
    let g = Rc::new(vec![n_1, n_1 * k]);
    let g_size = n_1 * n_1 * k;
    // upper bound, equal to 2s^2 + 2s + 1
    let upper_bound = (s * s) + (s+1)*(s+1);

    if g_size > upper_bound {
        return None;
    }

    for pair in <Vec<GElem> as SetLike>::each_set_exact(g.clone(), 2) {
        // Filter out "upper half"
        // added a +1 just to be safe
        if (pair[0].0[1] > n_1 * k / 2 + 1) || (pair[1].0[1] > n_1 * k / 2 + 1) {
            //println!("{}, {}: {}", pair[0].0[1], pair[1].0[1], k);
            continue;
        }

        let span = exactset::hfold_interval_signed_sumset(&pair, (0, s), g.clone());
        let size = span.len() as u32;
        
        if size == g_size {
            return Some(pair);
        }
    }

    None
}

pub fn do_two_elements_span(k: u32, s: u32) -> Option<Vec<GElem>> {
    let g = Rc::new(vec![2, 2 * k]);
    let g_size = 4 * k;

    // Haesoo's conjecture proven!
    if g_size > 2*s*s {
        return None;
    }

    for pair in <Vec<GElem> as SetLike>::each_set_exact(g.clone(), 2) {
        // Filter out "upper half"
        if (pair[0].0[1] > k) || (pair[1].0[1] > k) {
            //println!("{}, {}: {}", pair[0].0[1], pair[1].0[1], k);
            continue;
        }

        if does_pair_span(k, s, &g, &pair) {
            return Some(pair);
        }
    }

    None
}


// Make sure we pass it a pair of spanning elements
pub fn does_pair_span(k: u32, s: u32, g: &Rc<Vec<u32>>, pair: &Vec<GElem>) -> bool {
    // Puts the pair into form {(c, x), (d, y)}
    let p = &pair[0];
    let q = &pair[1];
    let c = p.0[0];
    let x = p.0[1];
    let d = q.0[0];
    let y = q.0[1];

    // Optimization 1 + bonus:
    // Proposition 10 of Park's Conjecture proof paper
    if c == 0 {
        if (d == 0) || /* Prop 10 : */ (x % 2 == 0) {
            return false;
        }
    } else if (d == 0) && (y % 2 == 0) { /* more prop 10*/
        return false;
    }

    // Optimization 2
    if gcd(2*k, gcd(x, y)) != 1 {
        return false;
    }

    // Optimization 3
    // Compute if x and y are not in [k / s, -k / s]
    let lb = k / s;
    let ub = 2*k - (k / s);
    if (x < lb || x > ub) && (y < lb || y > ub) {
        return false;
    }

    // Finally, check span
    // WARNING this should be the same as the size of g, but if somebody
    // passes this the wrong stuff then we're screwed
    let g_size = 4 * k;

    let span = exactset::hfold_interval_signed_sumset(&pair, (0, s), g.clone());
    let size = span.len() as u32;

    size == g_size
}

pub fn print_spanning_pairs(k: u32, s: u32) {
    let g = Rc::new(vec![2, 2 * k]);

    println!("Printing spanning pairs for (s, k) = ({}, {}) :", s, k);

    // WE'VE PROVEN HAESOO'S CONJECTURE!!!
    if 4*k > 2*s*s {
        return;
    }

    for pair in <Vec<GElem> as SetLike>::each_set_exact(g.clone(), 2) {
        let p = &pair[0];
        let q = &pair[1];
        let x = p.0[1];
        let y = q.0[1];

        // Optimization 4
        // Ignoring the "upper half" of potential spanning sets
        // NOTE only one not included in does_pair_span below
        if x > k || y > k {
            continue;
        }


        if does_pair_span(k, s, &g, &pair) {
            println!("{}: {:?}", k, pair);
            continue;
        }
    }
}

pub fn print_spannable(s : u32) {
    println!("Spannable groups for s = {}:", s);

    println!("  k < s given by proposition B.55");

    // This range excludes the maximum k, since we already know it by Park's
    for k in s..(s*s / 2) {
        match do_two_elements_span(k, s) {
            Some(pair) => {
                println!("  k = {:>3}  {:?}", k, pair);
            }
            None => {
                println!("  k = {:>3}  no spanning pair", k);
            }
        }
    }

    println!("  k = s^2 / 2 given by Park's\n");
}

pub fn print_pair_span_four(k: u32, s: u32) {
    let g = Rc::new(vec![4, 4 * k]);

    for pair in <Vec<GElem> as SetLike>::each_set_exact(g.clone(), 2) {
        // Filter out "upper half"
        if (pair[0].0[1] > 2*k) || (pair[1].0[1] > 2*k) {
            //println!("{}, {}: {}", pair[0].0[1], pair[1].0[1], k);
            continue;
        }

        // Filter out 3's
        if pair[0].0[0] == 3 || pair[1].0[0] == 3 {
            continue;
        }

        if does_pair_span_four(k, s, &g, &pair) {
            println!("  spanning pair: {:?}", pair);
        }
    }
}

pub fn pair_span_four(k: u32, s: u32) -> Option<Vec<GElem>> {
    let g = Rc::new(vec![4, 4 * k]);

    for pair in <Vec<GElem> as SetLike>::each_set_exact(g.clone(), 2) {
        // Filter out "upper half"
        if (pair[0].0[1] > 2*k) || (pair[1].0[1] > 2*k) {
            //println!("{}, {}: {}", pair[0].0[1], pair[1].0[1], k);
            continue;
        }

        if does_pair_span_four(k, s, &g, &pair) {
            return Some(pair);
        }
    }

    None
}

pub fn does_pair_span_four(k: u32, s: u32, g: &Rc<Vec<u32>>, pair: &Vec<GElem>) -> bool {
    // Puts the pair into form {(c, x), (d, y)}
    let p = &pair[0];
    let q = &pair[1];
    let c = p.0[0];
    let x = p.0[1];
    let d = q.0[0];
    let y = q.0[1];

    // Optimization 1 + bonus:
    // Proposition 10 of Park's Conjecture proof paper
    // Should still apply for 4 x 4k
    if c % 2 == 0 {
        if (d % 2 == 0) || /* Prop 10 : */ (x % 2 == 0) {
            return false;
        }
    } else if (d % 2 == 0) && (y % 2 == 0) { /* more prop 10*/
        return false;
    }

    // Optimization 2
    if gcd(4*k, gcd(x, y)) != 1 {
        return false;
    }

    // Finally, check span
    // WARNING this should be the same as the size of g, but if somebody
    // passes this the wrong stuff then we're screwed
    let g_size = 16 * k;

    let span = exactset::hfold_interval_signed_sumset(&pair, (0, s), g.clone());
    let size = span.len() as u32;

    size == g_size
}
