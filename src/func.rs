fn main() {
    let calc: fn(i32) -> i32;
    calc = square;
    let calculated = calc(5); 
    println!("{}", calculated);
}

fn square(x: i32) -> i32 {
    x * x
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn square_works() {
        assert_eq!(square(5), 25);
    }
}
