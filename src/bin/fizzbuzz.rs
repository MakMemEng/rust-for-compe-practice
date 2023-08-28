fn main() {
    let n: usize = 20;

    println!("FizzBuzz");
    fizz_buzz(n);

    println!("FizzBuzz match");
    fizz_buzz_match(n);
}

fn fizz_buzz(n: usize) {
    for i in 1..=n {
        if i % 15 == 0 {
            println!("{} : FizzBuzz", i);
        } else if i % 3 == 0 {
            println!("{} : Fizz", i);
        } else if i % 5 == 0 {
            println!("{} : Buzz", i);
        } else {
            println!("{}", i)
        }
    }
}

fn fizz_buzz_match(n: usize) {
    for i in 1..=n {
        match (i % 3, i % 5) {
            (0, 0) => println!("{} : FizzBuzz", i),
            (0, _) => println!("{} : Fizz", i),
            (_, 0) => println!("{} : Buzz", i),
            _ => println!("{}", i),
        }
    }
}
