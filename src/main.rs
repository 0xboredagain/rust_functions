fn main() {
    let m = 19;
    let value = 31;
    let unit_label = 'h';

    println!("Hello, world!");

    another_function();
    another_func_with_params(m);
    print_labeled_measurements(value, unit_label);

    let x = five();
    println!("the number is: {x}");

    let y = plus_one(x);
    println!("the number plus one is: {y}");
}

fn another_function() {
    println!("Another function.");
}

fn another_func_with_params(x: i32) {
    println!("The value of x is : {}!", x);
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}!");
}

//functions with return values

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}