use std::collections::HashSet;

pub struct DieGroup {
    pub amount: u32,
    pub sides: u32,
}

#[derive(PartialEq, Eq, std::hash::Hash)]
pub enum Flag {
    Total,
    Subtotals,
}

#[derive(Debug)]
pub enum ParseError {
    Flag,
    Dice,
}

// Parse cli args into the enabled flags and dice groups (# dice, # sides)
pub fn parsed_args(args: Vec<String>) -> Result<(HashSet<Flag>, Vec<DieGroup>), ParseError> {
    let (flags, dice): (Vec<String>, Vec<String>) =
        args.into_iter().partition(|a| (*a).starts_with('-'));

    let flags = parsed_flags(flags)?;
    let dice = parsed_dice(dice)?;

    Ok((flags, dice))
}

fn parsed_flags(string_flags: Vec<String>) -> Result<HashSet<Flag>, ParseError> {
    let mut flags = HashSet::new();

    for flag in string_flags {
        let flag = match flag.as_str() {
            "--total" | "-t" => Flag::Total,
            "--subtotals" | "-s" => Flag::Subtotals,
            _ => return Err(ParseError::Flag),
        };

        flags.insert(flag);
    }

    Ok(flags)
}

// Parses command line args into groups of dice in form (amount of dice, number of sides)
// Defaults to one d20 if no args were passed in
fn parsed_dice(string_dice: Vec<String>) -> Result<Vec<DieGroup>, ParseError> {
    if string_dice.is_empty() {
        return Ok(vec![DieGroup {
            amount: 1,
            sides: 20,
        }]);
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

    Ok(dice)
}
