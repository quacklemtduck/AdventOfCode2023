use std::{fs, io::{self, BufRead}};

fn main() -> io::Result<()> {
    let f = fs::File::open("input")?;

    let mut reader = io::BufReader::new(f);

    let mut line = String::new();
    let mut total = 0;

    while reader.read_line(&mut line)? > 0 {
        let mut first: Option<u32> = None;
        let mut last = 0;
        line = line.replace("one", "one1one");
        line = line.replace("two", "two2two");
        line = line.replace("three", "three3three");
        line = line.replace("four", "four4four");
        line = line.replace("five", "five5five");
        line = line.replace("six", "six6six");
        line = line.replace("seven", "seven7seven");
        line = line.replace("eight", "eight8eight");
        line = line.replace("nine", "nine9nine");
        for c in line.trim().chars() {
            let digit = c.to_digit(10);
            match digit {
                Some(x) => {
                    if first.is_none() {
                        first = Some(x);
                    }
                    last = x;
                }
                None => {}
            }
        }
        let value = (first.unwrap() * 10) + last;
        println!("{} {}{} add: {}",line.trim(), first.unwrap(), last, value);
        total += value;
        line.clear();
    }

    println!("{}", total);
    Ok(())
}
