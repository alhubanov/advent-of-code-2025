use std::fs;

fn main() {

    let file_path = "input.txt";
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut current_position = 50;
    let mut result = 0;
    for line in contents.lines() {
        let direction = &line[..1];
        let distance : i32 = (&line[1..]).to_owned().parse::<i32>().unwrap() % 100;

        if direction == "L" {
            current_position -= distance;
            if current_position < 0 {
                current_position += 100;
            }
        }
        else if direction == "R" {
            current_position += distance;
            if current_position > 99 {
                current_position -= 100;
            }
        }

        if current_position == 0 {
            result += 1;
        }
    }

    println!("{}", result)
}
