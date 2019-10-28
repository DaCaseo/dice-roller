/* CURRENT ISSUES:
- Can roll 0
- panics if you don't specify anything before "d"

I'm aware of its shortcomings for now, but those are not bugs.
*/
use rand::Rng;
use std::io;
use std::str::FromStr;

fn main() {
    println!("What dice would you like to roll? (<amount>d<size>)");
    loop {
        // get user input
        let mut dice_input = String::new();
        io::stdin().read_line(&mut dice_input).expect("Dice is wrong");
        let dice = dice_input.trim();
        // split the user input into rolls and dice size
        let split_dice: Vec<&str> = dice.split("d").collect();
        let rolls: u16 = FromStr::from_str(split_dice[0]).unwrap();
        let size: u16 = FromStr::from_str(split_dice[1]).unwrap();
        // generate roll
        let rolled_dice = roll_dice(rolls, size);
        println!("{:?}", rolled_dice);
    }
}

fn roll_dice(rolls: u16, size: u16) -> u16 {
    let mut rng = rand::thread_rng();
    let mut random_roll: u16 = 0;
    let mut x: u16 = 0;
    let gen_size: u16 = size + 1;
    while x < rolls {
        let tmp_roll: u16 = rng.gen_range(1, gen_size);
        random_roll += tmp_roll;
        x += 1;
    }
    return random_roll;
}
