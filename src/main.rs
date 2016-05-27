extern crate rand;
use rand::distributions::{IndependentSample, Range};

fn dice_roll(sides: u8) -> u8 {
//	return rand::random::<u8>() % 6;

let between = Range::new(0,sides);
let mut rng = rand::thread_rng();
return between.ind_sample(&mut rng);
}



fn main() {
let mut sm_hlth : i32
let mut sm_str : i32
}
