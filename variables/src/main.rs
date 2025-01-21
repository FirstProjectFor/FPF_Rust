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

    println!(
        "Sum: {}, Difference: {}, Product: {}, Quotient: {}, Remainder: {}",
        sum, difference, product, quotient, remainder
    );

    let t = true;

    println!("The value of t is: {}", t);

    let c = 'z';
    println!("The value of c is: {}", c);

    let heart_eyed_cat = '😻';

    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    let tup: (i32, f64, u8, i32) = (500, 6.4, 1, 2);

    println!("The value of tup is: {:?}", tup);

    let (x, y, z, a) = tup;

    println!("The value of x, y, z, a is: {}, {}, {}, {}", x, y, z, a);
    println!(
        "The value of x, y, z is: {}, {}, {}, {}",
        tup.0, tup.1, tup.2, tup.3
    );

    let a = [1, 2, 3, 4, 5];

    println!("The value of a is: {:?}", a);

    let a = [3; 100];

    println!("The value of a is: {:?}", a);

    
    let mut index = 0;
    loop {
        index = index + 1;
        if index == 999 {
            println!("The value of a[0] is:{}", a[index]);
            break;
        }
    }

}
