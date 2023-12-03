use std::{fs, io::{self, BufRead}, cmp};

fn main() -> io::Result<()> {
    let f = fs::File::open("input")?;

    let mut reader = io::BufReader::new(f);

    let mut line = String::new();
    let mut total = 0;
    let mut total2 = 0;

    while reader.read_line(&mut line)? > 0 {
        println!("{}", line);
        let vs: Vec<&str> = line.trim().split(":").collect();
        let r = vs[0].replace("Game ", "");
        let id: u32 = r.trim().parse().unwrap();
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        let rounds: Vec<&str> = vs[1].split(";").collect();
        for round in rounds {
            let types : Vec<&str> = round.split(", ").collect();
            for t in types {
                let k: Vec<&str> = t.trim().split(" ").collect();
                println!("{:?}", k);
                let value: u32 = k[0].trim().parse().unwrap();
                match k[1] {
                    "red" => max_red = cmp::max(max_red, value),
                    "blue" => max_blue = cmp::max(max_blue, value),
                    "green" => max_green = cmp::max(max_green, value),
                    _ => {println!("WTF!!! {}", k[1])}
                }
            }
        }
        println!("Red: {}, green: {}, blue: {}", max_red, max_green, max_blue);
        let power = max_red * max_green * max_blue;
        println!("Power: {}", power);
        total2 += power;
        if max_red <= 12 && max_blue <= 14 && max_green <= 13 {
            total += id;
        }
        line.clear();
    }

    println!("Task 1: {}", total);
    println!("Task 2: {}", total2);
    Ok(())
}
