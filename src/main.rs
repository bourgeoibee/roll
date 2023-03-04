use roll::{Flag, Roll};

use std::io::prelude::*;
use std::io::stdout;
use std::io::BufWriter;

use rand::prelude::*;

fn main() -> Result<(), roll::ParseError> {
    let raw_args: Vec<String> = std::env::args().collect();
    let args = raw_args
        .get(1..)
        .expect("1.. is never out of bounds")
        .to_vec();

    let (flags, dice) = roll::parsed_args(args)?;

    let mut rng = thread_rng();
    let stdout = stdout();
    let mut buffer = BufWriter::new(stdout);
    let mut total: u32 = 0;

    match dice {
        Roll::Single(die_group) => {
            for _ in 0..die_group.amount {
                let roll = rng.gen_range(1..=die_group.sides);

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
        Roll::Multiple(die_groups) => {
            for die_group in die_groups {
                let mut subtotal: u32 = 0;

                buffer
                    .write_fmt(format_args!("d{}:\n", die_group.sides))
                    .expect("Write failed");

                for _ in 0..die_group.amount {
                    let roll = rng.gen_range(1..=die_group.sides);

                    buffer
                        .write_fmt(format_args!(" {roll}\n"))
                        .expect("Write failed");
                    subtotal += roll
                }

                if flags.contains(&Flag::Subtotals) {
                    buffer
                        .write_fmt(format_args!("subtotal: {subtotal}\n"))
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
