fn main() {
    let calc: fn(i32) -> i32;
    calc = square;
    let calculated = calc(5); 
    println!("{}", calculated);

    let result = if calculated == 5*5 { "true" } else { "false" };
    println!("if_result is {}", result);
}

fn square(x: i32) -> i32 {
    x * x
}
