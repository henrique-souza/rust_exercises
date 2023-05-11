fn sum(first_number: i32, second_number: i32) -> i32 {
    first_number + second_number
}

fn subtract(first_number: i32, second_number: i32) -> i32 {
    first_number - second_number
}

fn main() {
    let other_number: i32;

    other_number = 12;

    println!(
        "Quanto é a soma de: {} + {}? \n\nA soma é {}",
        3,
        2,
        sum(2, 3)
    );
    println!("");

    println!(
        "E quanto é {} + {}? \n\nResultado: {}",
        other_number,
        sum(2, 3),
        sum(other_number, sum(2, 3))
    );
    println!("");

    println!(
        "E se diminuirmos {} de {}? \n\nResultado: {}",
        sum(2, 3),
        other_number,
        subtract(other_number, sum(2, 3))
    );
}
