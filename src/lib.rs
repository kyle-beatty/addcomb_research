pub mod spanning {
    use addcomb_comp::setlike::SetLike;
    use addcomb_comp::exactset;
    use addcomb_comp::comb::gcd;

    use std::rc::Rc;
    use addcomb_comp::exactset::GElem;

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

    // NOTE OLD FUNCTION
    // REMINDER this is for use with k above Haesoo's conjectured upper limit
    // We've modified this so much that it excludes valid spanning sets for lower k
    pub fn print_spans(s: u32, k: u32) {
        let g = Rc::new(vec![2, 2 * k]);
        //let g_size = g.gsize();

        //NEW lowered the upper bound in light of new work
        // if 4*k >= 2*s*s + 2*s {
        //     println!("Group cannot be spanned: ({}, {})", k, s);
        //     return;
        // }

        for a in <Vec<GElem> as SetLike>::each_set_exact(g.clone(), 2) {
            let p;
            let q;
            let x;
            let y;

            // Assign to p and q based on which one is even and which is odd
            if a[0].0[0] == 0 {
                if a[1].0[0] == 0 {
                    continue;
                } else {
                    p = &a[0];
                    q = &a[1];
                }
            } else {
                if a[1].0[0] == 0 {
                    q = &a[0];
                    p = &a[1];
                } else {
                    continue;
                }
            }

            // Now that we've figured that out, assign other variables
            x = p.0[1];
            y = q.0[1];

            // Don't worry about upper halves
            if x > k || y > k {
                continue;
            }

            // Skip even x values and (NEW) odd y values
            if x % 2 == 0 || y % 2 == 1 {
                continue;
            }

            // Skip if x, y, and 2k all share a factor
            if gcd(2*k, gcd(x, y)) != 1 {
                continue;
            }

            // Optimization 3
            // Compute if x and y are less than k / s (above its inverse already
            // ruled out by skipping top halves)
            // TODO make this more precise using float division?
            let lb = k / s;
            if x < lb && y < lb {
                continue;
            }

            // Optimization 4 NEW
            // Ignoring the "upper half" of potential spanning sets
            if x > k || y > k {
                continue;
            }

            println!("Inspecting sumset of (x, y) = ({}, {})", x, y);

            let sumset = exactset::hfold_interval_signed_sumset(&a, (0, s), g.clone());
            let odd_middle = GElem(vec![1, k]);
            let even_middle = GElem(vec![0, k]);
            let one    = GElem(vec![1, 0]);
            let mut missing = false;

            if !sumset.contains(&odd_middle) {
                println!("(x, y) = ({}, {}) does not contain (1, k) = (1, {})",
                x, y, k);
                missing = true;
            }

            if !sumset.contains(&even_middle) {
                println!("(x, y) = ({}, {}) does not contain (0, k) = (0, {})",
                x, y, k);
                missing = true;
            }

            if !sumset.contains(&one) {
                println!("(x, y) = ({}, {}) does not contain (1, 0)",
                x, y);
                missing = true;
            }

            if !missing {
                println!("Holy shit");
            }

            println!();
        }
    }
}
