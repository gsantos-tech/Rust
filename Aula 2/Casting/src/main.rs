fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    let inteiro = 10;

    let inteiro_pra_float = inteiro as f32;

    let float = 2.5;

    let float_para_int = float as i32;

    let inteiro_pra_string = inteiro.to_string();

    let string = 42;

    let string_para_int = inteiro_pra_string.parse::<i64>().unwrap();

    println!("Valor da variável inteiro, {}: {}", inteiro, type_of(inteiro));
    println!("Valor da variável float: {}", inteiro_pra_float);

    println!("Valor da variável float: {}", float);
    println!("Valor da variável float para inteiro: {}", float_para_int);

    println!("Valor de Inteiro: {} para string: {}", inteiro, type_of(inteiro_pra_string));

    println!("Valor de String: {} para string: {}", type_of(string), type_of(string_para_int));
}
