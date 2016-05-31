extern crate rand;
use rand::distributions::{IndependentSample, Range};

fn dice_roll(sides: i32) -> i32 {
//	return rand::random::<u8>() % 6;
	let between = Range::new(0, sides);
	let mut rng = rand::thread_rng();
	return between.ind_sample(&mut rng);
}

fn main() {

	// first character
	let mut sm_hlth = 100;
	let sm_str = dice_roll(11) + 10;
	println!("Space marine has health {} and strength {}", sm_hlth, sm_str);

	// second chraracter
	let mut cd_hlth = 100;
	let cd_str = dice_roll(11) + 10;
	println!("Chaos Deomon has health {} and strength {}", cd_hlth, cd_str);

// dice roll of 2 instead of complex odd or evens. Just roll a 2 sided die, 1 is hit 0 is miss
// commenting out just to test something else
//	let mut hit_or_miss = dice_roll(2);

// while loop attemp
	while cd_hlth > 0 && sm_hlth > 0 {

		let hit_or_miss = dice_roll(2);

		println!("Roll to hit or miss {}", hit_or_miss);
		if hit_or_miss == 1 {
			cd_hlth -= dice_roll(6) * sm_str;
		} else {
			println!("You missed, Chaos Demon will now roll");
		}

		println!("Chaos Demon Health {}", cd_hlth);
		println!("Chaos Demon now rolls {}", hit_or_miss);

		if hit_or_miss == 1 {
			sm_hlth -= dice_roll(6) * cd_str;
		} else {
			println!("Chaos Demon missed, Space Marine will now roll");
		}
	}

	if cd_hlth <= 0 {
		println!("Chaos demon died! You win!");
	}

	if sm_hlth <= 0 {
		println!("You died! You lose!");
	}


// while loop contains "take one away from health" do one damage every turn. make a stub function jsut to see if while loop is working. Then work on dice logic
//	let mut cd_hlth = cd_hlth - 101;
//	println!("Chaos Demon's strength is now {}", cd_hlth);
//sm_hlth - = 1

}
