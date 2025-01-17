const PI: f64 = 3.14;

fn main() {
    println!("The value of PI is: {}", PI);

    let x = 5;
    println!("The value of x is: {}", x);

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    let x = x + 1;
    println!("The value of x is: {}", x);

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    println!("Sum: {}, Difference: {}, Product: {}, Quotient: {}, Remainder: {}", sum, difference, product, quotient, remainder);

    let t = true;
    
    println!("The value of t is: {}", t);

    let c = 'z';
    println!("The value of c is: {}", c);

    let heart_eyed_cat = '😻';

    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);
}
