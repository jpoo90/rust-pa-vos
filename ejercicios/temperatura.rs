use std::io;

fn main() {
    loop {
        println!("Select the conversion");
        println!("1. F to C");
        println!("2. C to F");

        let mut selection = String::new();

        io::stdin()
            .read_line(&mut selection)
            .expect("Error reading line");

        let selection: u32 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut base_temp = String::new();

        println!("Temp");

        io::stdin()
            .read_line(&mut base_temp)
            .expect("Error reading line");

        let base_temp: f64 = match base_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match selection {
            1 => println!("{}F are {}C", base_temp, f_to_c(base_temp)),
            2 => println!("{}C are {}F", base_temp, c_to_f(base_temp)),
            _ => println!("Select 1 or 2"),
        };
    }
}

fn c_to_f(temp_c: f64) -> f64 {
    (temp_c * 9.0 / 5.0) + 32.0
}

fn f_to_c(temp_f: f64) -> f64 {
    (temp_f - 32.0) * (5.0 / 9.0)
}
ol) -> String {
  if index == 0 {
    if is_listing {
      format!("and a {}", items[index])
    } else {
      format!("A {}", items[index])
    }
  } else {
    format!("{} {}", index + 1, items[index])
  }
}
