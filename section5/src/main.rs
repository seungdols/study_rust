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
}
