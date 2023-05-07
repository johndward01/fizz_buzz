fn main() {
    let fizzes_and_buzzes = fizz_buzz(100);
    for i in fizzes_and_buzzes {
        println!("{i}");
    }
}

fn fizz_buzz(n: usize) -> Vec<String> {
    let mut vec = vec![String::new(); 100];
    for i in 1..n {
        if i % 3 == 0 && i % 5 == 0 {
            vec[i] = "FizzBuzz".into();
        } else if i % 3 == 0 {
            vec[i] = "Fizz".into();
        } else if i % 5 == 0 {
            vec[i] = "Buzz".into();
        } else {
            vec[i] = format!("{i}")
        }
    }
    return vec;
}
