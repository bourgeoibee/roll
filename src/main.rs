use std::collections::HashSet;
use std::io::prelude::*;
use std::io::stdout;
use std::io::BufWriter;
// use std::str::FromStr;

use rand::prelude::*;

fn main() -> Result<(), ParseError> {
    let raw_args: Vec<String> = std::env::args().collect();
    let args = raw_args
        .get(1..)
        .expect("1.. is never out of bounds")
        .to_vec();

    let (flags, dice) = parsed_args(args)?;

    let mut rng = thread_rng();
    let stdout = stdout();
    let mut buffer = BufWriter::new(stdout);

    match dice.len() {
        // No args -> roll d20
        0 => {
            let roll = rng.gen_range(1..=20);

            buffer
                .write_fmt(format_args!("{roll}\n"))
                .expect("Write failed");
        }
        1 => {
            let mut total: u32 = 0;

            for _ in 0..dice[0].amount {
                let roll = rng.gen_range(1..=dice[0].sides);

                buffer
                    .write_fmt(format_args!("{roll}\n"))
                    .expect("Write failed");
                total += roll;
            }

            if flags.contains(&Flag::Total) {
                buffer
                    .write_fmt(format_args!("total: {total}\n"))
                    .expect("Write failed");
            }
        }
        _ => {
            let mut total: u32 = 0;

            for die in dice {
                assert!(die.sides != 0, "Parser did not prevent zero-sided die");

                let mut subtotal: u32 = 0;

                buffer
                    .write_fmt(format_args!("d{}:\n", die.sides))
                    .expect("Write failed");

                for _ in 0..die.amount {
                    let roll = rng.gen_range(1..=die.sides);

                    buffer
                        .write_fmt(format_args!(" {roll}\n"))
                        .expect("Write failed");
                    subtotal += roll
                }

                if flags.contains(&Flag::Subtotals) {
                    buffer
                        .write_fmt(format_args!("d{} subtotal: {subtotal}\n", die.sides))
                        .expect("Write failed");
                }
                total += subtotal;
            }

            if flags.contains(&Flag::Total) {
                buffer
                    .write_fmt(format_args!("total: {total}\n"))
                    .expect("Write failed");
            }
        }
    }

    buffer.flush().expect("Flush failed");

    Ok(())
}

struct DiceGroup {
    amount: u32,
    sides: u32,
}

#[derive(PartialEq, Eq, std::hash::Hash)]
enum Flag {
    Total,
    Subtotals,
}

#[derive(Debug)]
enum ParseError {
    Flag,
    Dice,
}

// Parse cli args into the enabled flags and dice groups (# dice, # sides)
fn parsed_args(args: Vec<String>) -> Result<(HashSet<Flag>, Vec<DiceGroup>), ParseError> {
    let (str_flags, str_dice): (Vec<String>, Vec<String>) =
        args.into_iter().partition(|a| (*a).starts_with('-'));

    let flags = parsed_flags(str_flags)?;
    let dice = parsed_dice(str_dice)?;

    Ok((flags, dice))
}

fn parsed_flags(args: Vec<String>) -> Result<HashSet<Flag>, ParseError> {
    let mut flags = HashSet::new();

    for arg in args {
        let flag = match arg.as_str() {
            "--total" | "-t" => Flag::Total,
            "--subtotals" | "-s" => Flag::Subtotals,
            _ => return Err(ParseError::Flag),
        };

        flags.insert(flag);
    }

    Ok(flags)
}

// Parses command line args into groups of dice in form (amount of dice, number of sides)
fn parsed_dice(args: Vec<String>) -> Result<Vec<DiceGroup>, ParseError> {
    let mut dice = vec![];

    for arg in args {
        let (amount, sides) = arg.split_once('d').ok_or(ParseError::Dice)?;

        let amount = amount.parse::<u32>().or_else(|error| match error.kind() {
            std::num::IntErrorKind::Empty => Ok(1),
            _ => Err(ParseError::Dice),
        })?;

        let sides = match sides.parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                return Err(ParseError::Dice);
            }
        };

        if sides == 0 {
            return Err(ParseError::Dice);
        }

        dice.push(DiceGroup { amount, sides })
    }

    Ok(dice)
}
