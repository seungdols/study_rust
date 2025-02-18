struct User {
    name: String,
    email: String,
    active: bool,
}

fn build_user(name: String, email: String) -> User {
    User {
        // name: name,
        // email: email,
        name,
        email,
        active: true,
    }
}


struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // method 첫번째 파라미터는 self, &self, &mut self 모두 가능
    fn area(&self) -> u32 {
        &self.width * &self.height
    }

}

fn area (width: u32, height: u32) -> u32 {
    width * height
}

fn area2(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
fn main() {
    {
        let mut user = User {
            name: String::from("seungdols"),
            email: String::from("seungdols@company.com"),
            active: true,
        };


        user.email = String::from("sh@gmail.com");
        println!("name = {}", user.name);
    }

    {
        let user1 = User {
            name: String::from("seungdols"),
            email: String::from("seungdols@company.com"),
            active: true,
        };
        let user2 = User {
            name: user1.name,
            email: user1.email,
            active: false,
        };

        println!("user2.email = {}", user2.email);
    }

    {
        let user1 = User {
            name: String::from("seungdols"),
            email: String::from("seungdols@company.com"),
            active: true,
        };
        let user2 = User {
            active: false,
            ..user1
        };

        println!("user2.email = {}", user2.email);
    }

    {
        let color = Color(1,2,3);
        let point = Point(1,2,3);
        color.0;
    }

    {
        let width = 20;
        let height = 30;

        println!("Area: {}", area(width, height));
    }

    {
        let rect = Rectangle {
            width: 20,
            height: 30
        };

        // println!("Area: {}", area2(&rect));
        // println!("Rectangle: {:?}", rect);
        // dbg!(rect);


        println!("Area: {}", rect.area());

        println!("정사각형: {:?}", Rectangle::square(20));
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}
