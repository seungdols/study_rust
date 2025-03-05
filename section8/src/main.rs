fn smallest<T: std::cmp::PartialOrd>(list: &[T]) -> &T{
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}
// fn smallest_i32(list: &[i32]) -> &i32 {
//     let mut smallest = &list[0];

//     for item in list {
//         if item < smallest {
//             smallest = item;
//         }
//     }

//     smallest
// }

// fn smallest_char(list: &[char]) -> &char {
//     let mut smallest = &list[0];

//     for item in list {
//         if item < smallest {
//             smallest = item;
//         }
//     }

//     smallest
// }

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let numbers = vec![3,4,1,6,8,10];
    let result = smallest(&numbers);
    println!("가장 작은 수는 {}", result);

    let chars = vec!['홍', '길', '동'];
    let result = smallest(&chars);
    println!("가장 작은 문자는 {}", result);

    let result = smallest(&["홍길도", "둘리", "도우너"]);
    println!("가장 작은 문자열은 {}", result);

    let p1 = Point { x: 2, y:3 };
    let p2 = Point { x: 5.0, y: 1.0};
    println!("p1 = {:?}, p2 = {:?}", p1, p2);
}
