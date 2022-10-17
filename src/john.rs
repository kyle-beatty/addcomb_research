use std::rc::Rc;
use addcomb_comp::exactset::GElem;
use addcomb_comp::comb::chapter_a::nu;

pub fn john_main(_args : &Vec<String>) {

    let g = Rc::new(vec![43]);
    let result = nu::<Vec<GElem>>(g, 7, 2, true);
    println!("Result: {}", result);
}
