fn sum(first_number: i32, second_number: i32) -> i32 {
    first_number + second_number
}

fn subtract(first_number: i32, second_number: i32) -> i32 {
    first_number - second_number
}

fn main() {
    // variável mutável
    let mut other_number: i32 = 12;

    // variável imutável
    // let other_number: i32 = 12;

    println!(
        "Quanto é a soma de: {} + {}? \n\nA soma é {}",
        3,
        2,
        sum(2, 3)
    );

    println!(
        "\nE quanto é {} + {}? \n\nResultado: {}",
        other_number,
        sum(2, 3),
        sum(other_number, sum(2, 3))
    );

    /*
     * impossibilitado de mudar a variável imutável
     * ERROR: cannot mutate immutable variable `other_number`
     * other_number = 40;
     */

    // modificando a variável mutável
    other_number = 40;

    println!(
        "\nE se diminuirmos {} de {}? \n\nResultado: {}",
        sum(2, 3),
        other_number,
        subtract(other_number, sum(2, 3))
    );

    /*
     * Nesse caso não seria uma variável mutável pois usamos a chamada dela mesma para 'sombrear' novos valores
     * variável não assinada, ou seja, 'u(bits)' não podem ser número negativo
     * já variáveis assinadas 'i(bits)' podem ser número negativo
     */
    let shadow_num: i32 = 5;

    println!(
        "\nInicializando Sombreamento de variável: o valor atual de 'shadow_num' é: {}",
        shadow_num
    );

    let shadow_num: i32 = shadow_num + 5;

    println!(
        "\nAdicionando 5 ao Sombreamento de variável: o valor atual de 'shadow_num' é: {}",
        shadow_num
    );

    let shadow_num: i32 = shadow_num * 2;

    println!(
        "\nMultiplicando por 2 o Sombreamento de variável: o valor atual de 'shadow_num' é: {}",
        shadow_num
    );

    // let number_64: f64 = 4.0;

    // let number_32: f32 = 5.0;
}
