use rand::prelude::*;

fn main() {
    let raw_args: Vec<String> = std::env::args().collect();
    let args = raw_args.get(1..).expect("1.. is never out of bounds");

    let dice = match input_to_dice(args) {
        Some(d) => d,
        None => {
            eprintln!("Failed to parse dice");
            return;
        }
    };

    let mut rng = thread_rng();

    for (times, die) in dice {
        assert!(die != 0, "Parser did not prevent zero-sided die");

        for _ in 0..times {
            let roll = rng.gen_range(1..=die);
            println!("d{die}: {roll}");
        }
    }
}

// Parses command line args into groups of dice in form (ammount of dice, number of sides)
// Matches empty input to 1 d20
fn input_to_dice(args: &[String]) -> Option<Vec<(u32, u32)>> {
    let mut dice: Vec<(u32, u32)> = vec![];

    if args.is_empty() {
        return Some(Vec::from([(1, 20)]));
    }

    for arg in args {
        let (times, die) = arg.split_once('d')?;

        let times = if times.is_empty() {
            1u32
        } else {
            times.parse::<u32>().ok()?
        };

        let die = die.parse::<u32>().ok()?;

        if die == 0 {
            return None;
        }

        dice.push((times, die))
    }

    Some(dice)
}
