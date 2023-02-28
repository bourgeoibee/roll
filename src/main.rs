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

    if dice.len() == 1 {
        let mut total: u32 = 0;

        for _ in 0..dice[0].amount {
            let roll = rng.gen_range(1..=dice[0].sides);

            buffer
                .write_fmt(format_args!("{roll}\n"))
                .expect("Write failed");
            total += roll;
        }

        if flags.contains(&roll::Flag::Total) {
            buffer
                .write_fmt(format_args!("total: {total}\n"))
                .expect("Write failed");
        }
    } else {
        let mut total: u32 = 0;

        for die_group in dice {
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

            if flags.contains(&roll::Flag::Subtotals) {
                buffer
                    .write_fmt(format_args!("d{} subtotal: {subtotal}\n", die_group.sides))
                    .expect("Write failed");
            }
            total += subtotal;
        }

        if flags.contains(&roll::Flag::Total) {
            buffer
                .write_fmt(format_args!("total: {total}\n"))
                .expect("Write failed");
        }
    }

    buffer.flush().expect("Flush failed");

    Ok(())
}
