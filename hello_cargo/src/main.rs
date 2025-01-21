#[derive(Debug)]
struct User {
    username: String,
    age: u16,
    email: String,
}

fn main() {
    let mut user1 = User {
        age: 23,
        email: String::from("example@www.com"),
        username: String::from(""),
    };
    user1.username = String::from("王五");
    user1.email = String::from("anotheremail@example.com");
    user1.age = 24;


    let user2 = User {
        username: String::from("李四"),
        age: 23,
        email: String::from("anotheremail@example.com"),
    };

    println!("The value of user1 is: {:?}", user1);
    println!("The value of user2 is: {:?}", user2);
}
