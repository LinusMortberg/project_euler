fn main() {
    let mut num1 = 1;
    let mut num2 = 1;
    let mut sum = 0;

    while (num1 + num2) < 4000000 {
        let intermit = num1 + num2;
        num1 = num2;
        num2 = intermit;
        if num2 % 2 == 0 {
            sum += num2;
        }
    }
    println!("{}", sum);
}
