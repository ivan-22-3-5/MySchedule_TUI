use std::str::FromStr;

pub fn scan<T>(invalid_input_message: &str) -> T 
where
    T: FromStr
{
    let mut input = String::new();
    loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(parsed_input) => return parsed_input,
            Err(_) => { 
                println!("{invalid_input_message}"); 
                continue;
            }
        };
    }
}
