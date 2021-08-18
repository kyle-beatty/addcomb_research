pub mod spanning {
    use addcomb_comp::setlike::SetLike;
    use addcomb_comp::setlike::Group;
    use addcomb_comp::exactset;
    use addcomb_comp::comb::gcd;

    use std::rc::Rc;
    use addcomb_comp::exactset::GElem;

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

    pub fn print_elements_span(k: u32, s: u32) -> Option<Vec<GElem>> {
        let g = Rc::new(vec![2, 2 * k]);
        let g_size = g.gsize();

        // WE'VE PROVEN HAESOO'S CONJECTURE!!!
        if 4*k > 2*s*s {
            return None;
        }

        for a in <Vec<GElem> as SetLike>::each_set_exact(g.clone(), 2) {
            let p = &a[0];
            let q = &a[1];
            let c = p.0[0];
            let x = p.0[1];
            let d = q.0[0];
            let y = q.0[1];

            // Optimization 1
            if c == 0 && d == 0 {
                continue;
            }

            // Optimization 4
            // Ignoring the "upper half" of potential spanning sets
            if x > k || y > k {
                continue;
            }

            // Optimization 2
            if gcd(2*k, gcd(x, y)) != 1 {
                continue;
            }

            // Optimization 3
            // Compute if x and y are not in [k / s, -k / s]
            // We've already knocked out any in the upper half
            let lb = k / s;
            if x < lb && y < lb {
                continue;
            }

            let size =
                exactset::hfold_interval_signed_sumset(&a, (0, s), g.clone()).len()
                as u32;

            if size == g_size {
                //println!("  {}: {:?}", s, a);
                //TODO remove this
                println!("{}: {:?}", k, a);
                return None;
            }
        }

        println!("{}: no {}-spanning pair", k, s);

        None
    }

}
