fn main() {
    let sum = sum_of_multiples(3, 999) + sum_of_multiples(5, 999) - sum_of_multiples(15, 999);
    println!("The sum of all the multiples of 3 or 5 below 1000 is: {}", sum);
}

fn sum_of_multiples(factor: u32, limit: u32) -> u32 {
    let n = limit / factor;
    factor * n * (n + 1) / 2
}
