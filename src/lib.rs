use std::collections::HashSet;

#[derive(Clone)]
pub struct DieGroup {
    pub amount: u32,
    pub sides: u32,
}

#[derive(PartialEq, Eq, std::hash::Hash)]
pub enum Flag {
    Total,
    Subtotals,
    Help,
}

#[derive(Debug)]
pub enum ParseError {
    Flag,
    Dice,
}

pub enum Roll {
    Single(DieGroup),
    Multiple(Vec<DieGroup>),
}

// Parse cli args into the enabled flags and dice groups (# dice, # sides)
pub fn parsed_args(args: Vec<String>) -> Result<(HashSet<Flag>, Roll), ParseError> {
    let (flags, rolls): (Vec<String>, Vec<String>) =
        args.into_iter().partition(|a| (*a).starts_with('-'));

    let flags = parsed_flags(flags)?;
    let rolls = parsed_dice(rolls)?;

    Ok((flags, rolls))
}

fn parsed_flags(string_flags: Vec<String>) -> Result<HashSet<Flag>, ParseError> {
    let (long_options, short_options): (Vec<String>, Vec<String>) = string_flags
        .into_iter()
        .partition(|f| (*f).starts_with("--"));

    let mut flags = HashSet::new();

    for opt in long_options {
        let flag = match opt.as_str() {
            "--subtotals" => Flag::Subtotals,
            "--total" => Flag::Total,
            "--help" => Flag::Help,
            _ => return Err(ParseError::Flag),
        };

        flags.insert(flag);
    }

    for opt_group in short_options {
        let opts: Vec<char> = opt_group.chars().skip(1).collect();

        for c in opts {
            let flag = match c {
                's' => Flag::Subtotals,
                't' => Flag::Total,
                'h' => Flag::Help,
                _ => return Err(ParseError::Flag),
            };

            flags.insert(flag);
        }
    }

    Ok(flags)
}

// Parses command line args into groups of dice in form (amount of dice, number of sides)
// Defaults to one d20 if no args were passed in
fn parsed_dice(string_dice: Vec<String>) -> Result<Roll, ParseError> {
    if string_dice.is_empty() {
        return Ok(Roll::Single(DieGroup {
            amount: 1,
            sides: 20,
        }));
    }

    let mut dice = vec![];
    for die_group in string_dice {
        let (amount, sides) = die_group.split_once('d').ok_or(ParseError::Dice)?;

        let amount = amount.parse::<u32>().or_else(|error| match error.kind() {
            std::num::IntErrorKind::Empty => Ok(1),
            _ => Err(ParseError::Dice),
        })?;

        let sides = match sides.parse::<u32>() {
            Ok(0) | Err(_) => {
                return Err(ParseError::Dice);
            }
            Ok(n) => n,
        };

        dice.push(DieGroup { amount, sides })
    }

    if dice.len() == 1 {
        return Ok(Roll::Single(dice[0].clone()));
    }

    Ok(Roll::Multiple(dice))
}
