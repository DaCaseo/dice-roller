/* CURRENT ISSUES:
- Can roll 0
- panics if you don't specify anything before "d"

I'm aware of its shortcomings for now, but those are not bugs.
*/
use rand::Rng;
use std::io;
use std::str::FromStr;

fn main() {
    // get user input
    let mut dice_input = String::new();
    println!("What dice would you like to roll? (<amount>d<limit>)");
    io::stdin().read_line(&mut dice_input).expect("Dice is wrong");
    let dice = dice_input.trim();
    // split the user input into rolls and dice size
    let split_dice: Vec<&str> = dice.split("d").collect();
    let rolls: u16 = FromStr::from_str(split_dice[0]).unwrap();
    let size: u16 = FromStr::from_str(split_dice[1]).unwrap();
    // generate roll
    let mut rng = rand::thread_rng();
    let random_roll: u16 = rng.gen_range(0, size);
    println!("{:?}", random_roll)
}
