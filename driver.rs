mod fibonacci;

fn main() {
    match fibonacci::fibonacci(8) {
        Ok(number) => println!("8th index number is {:?}", number),
        Err(mut error) => println!("error is {:?}", error.description()),
    }

    let before_overflow = fibonacci::fibonacci(85);
    match before_overflow {
        Ok(number) => println!("85th index number is {:?}", number),
        Err(mut error) => println!("error is {:?}", error.description()),
    }

    let overflow = fibonacci::fibonacci(10000);
    match overflow {
        Ok(number) => println!("10000th index number is {:?}", number),
        Err(mut error) => println!("error is {:?}", error.description()),
    }
}
