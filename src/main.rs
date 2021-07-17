use std::io;

fn main() {
    loop {
        println!("Enter temperature string to convert:");
        let mut temp_string: String = String::new();
        io::stdin()
            .read_line(&mut temp_string)
            .expect("Error when reading from stdin");

        temp_string = temp_string
            .trim()
            .split(' ')
            .collect::<String>()
            .to_string();

        let unit_string: String = match temp_string.chars().rev().next() {
            Some(chr) => {
                temp_string = temp_string
                    .strip_suffix(chr)
                    .expect("Error when stripping unit string")
                    .to_string();
                chr.to_string().to_uppercase()
            }
            None => String::from(""),
        };

        let metric: bool;
        match unit_string.as_str() {
            "C" => metric = true,
            "F" => metric = false,
            _ => {
                println!("Not a valid temperature unit!");
                continue;
            }
        }

        let temp_value = match temp_string.parse::<f64>() {
            Ok(f) => f,
            Err(_) => {
                println!("Not a valid temperature value");
                continue;
            }
        };

        if metric {
            println!("{:.1} °F", (temp_value * 9.0 / 5.0) + 32.0);
            return;
        } else {
            println!("{:.1} °C", (temp_value - 32.0) * 5.0 / 9.0);
            return;
        }
    }
}
