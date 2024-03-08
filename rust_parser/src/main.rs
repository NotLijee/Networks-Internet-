use std::io;

fn main() {
    println!("Enter the string in the format XXXX:XXXX:XXXX:XXXX:XXXX:XXXX:XXXX:XXXX");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    input = input.trim().to_string();

    let parts: Vec<&str> = input.split(':').collect();

    if parts.len() != 8 {
        panic!("Invalid input format");
    }

    let part1 = parts[0];
    let part2 = parts[1];
    let part3 = parts[2];
    let part4 = parts[3];
    let part5 = parts[4];
    let part6 = parts[5];
    let part7 = parts[6];
    let part8 = parts[7];

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 3: {}", part3);
    println!("Part 4: {}", part4);
    println!("Part 5: {}", part5);
    println!("Part 6: {}", part6);
    println!("Part 7: {}", part7);
    println!("Part 8: {}", part8);
}
