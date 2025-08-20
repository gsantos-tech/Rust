fn main() {
    function_name();
    with_return();
    let with_return_value = with_return();
    println!("Com return: {}", with_return_value);
    no_return();
    let no_return_value = no_return();
    println!("Com no_return: {}", no_return_value);
    higher_value(2,7);

    let d = 5;

    let resultado = increasing_function(d);
    println!("Com resultado: {}", resultado);
}

fn function_name() {
    println!("Hello, world!");
}

fn with_return() -> i8 {
    return 3;
}

fn no_return() -> i8 {
    3
}

fn higher_value(n1: i32, n2: i32) -> i32 {
    {
        if n1 > n2 {
            println!("higher_value: {}", n1);
            return n1;
        }
        else {
            println!("higher_value: {}", n2);
            return n2;
        }
    }

}

fn increasing_function(mut n1: i32) -> i32 {
    n1 += 1;
    n1
}