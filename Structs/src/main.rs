
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

//  Method implementation for rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height   
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Assocatied function implementation for rectangle struct
impl Rectangle {
    fn square (size: u32) -> Rectangle {
        Rectangle {
            width : size,       
            height : size
        }
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true
    }
}

fn cal_area (rect: &Rectangle) -> u32 {
    return rect.width * rect.height;
}

fn main() {
    let user1 = build_user( String::from("ad_18@example.com"), String::from("ad_18"));
    println!("Username: {}, Email: {}, Sign-in Count: {}, Active: {}", 
             user1.username, user1.email, user1.sign_in_count, user1.active);

    let user2 = User {
        email : String::from("ad_19@example.com"),
        username : String::from("ad_19"),
        ..user1
    };

    println!("Username: {}, Email: {}, Sign-in Count: {}, Active: {}", 
             user2.username, user2.email, user2.sign_in_count, user2.active);




    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    // Way to use display trait for non primitive types
    println!("Rectangle info: {:#?}", rect1); // This will throw error unless we derive Debug trait for User struct

    println!("The area of the rectangle is {}.", cal_area(&rect1));

    let rect2 = Rectangle {
        width: 10,
        height: 40
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45
    };

    // Method calls
    println!("Area of rect2 is {} square pixels.", rect2.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


    // assiciated function call
    let square1 = Rectangle::square(20);
    println!("Square1 info: {:#?}", square1);
}