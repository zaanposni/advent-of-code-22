
fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut array = [0; 5000];
    let mut current_index = 0;
    for line in input.lines() {
        if line == "" {
            current_index += 1;
            continue;
        }

        let line_int = line.parse::<i32>().unwrap();
        array[current_index] += line_int;
    }

    array.sort();

    println!("{}", array[4999]);

    let mut sum = 0;
    // iterate over three last numbers
    for i in array.len() - 3..array.len() {
        println!("{} ", array[i]);
        sum += array[i];
    }

    println!("Sum: {}", sum);
}
