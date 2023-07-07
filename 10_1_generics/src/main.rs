fn main() {
    // Not with Generics
    let number_list = vec![1,2,3,4,5];
    let result_1 = find_largest_number(number_list);
    println!("result_1: {:?}", result_1);

    let char_list = vec!['e', 'z', 'n'];
    let result_2 = find_largest_char(char_list);
    println!("result_2: {:?}", result_2);

    // Or with Generics!
    let list = vec!['e', '1', 'n','2','f','z','Z'];
    let largest = find_largest_generic(list);
    println!("largest: {:?}", largest);
    let list_2 = vec![8123,2838,42924,123,942841,2720,19];
    let largest_2 = find_largest_generic(list_2);
    println!("largest: {:?}", largest_2);

    let point_diff = Point { x: 42, y: 12.42 };
    let point_same = Point2 { x: 42.12, y: 12.42 };

    println!("{:?}\n{:?}", point_diff, point_same);

    let get_x = point_diff.x();
    let get_y = point_diff.y();
    let get_x2 = point_same.x();
    let get_y2 = point_same.y();
    println!("get_x: {}\nget_y: {}", get_x, get_y);
    println!("get_x: {}\nget_y: {}", get_x2, get_y2);

    let point_crazy = Point { x: "Hello", y: 'c' };
    let mixup = point_diff.mixup(point_crazy);
    println!("mixup: {:?}", mixup); // { x: 42, y: 'c' }
}

fn find_largest_number(vec: Vec<i32>) -> i32 {
    let mut largest = vec[0];
    for number in vec {
        if number > largest {
            largest = number
        }
    }
    largest
}
fn find_largest_char(vec: Vec<char>) -> char {
    let mut largest = vec[0];
    for char in vec {
        if char > largest {
            largest = char
        }
    }
    largest
}

fn find_largest_generic<T: PartialOrd + Copy>(vec: Vec<T>) -> T {
    let mut largest = vec[0];
    for val in vec {
        if val > largest {
            largest = val
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

#[derive(Debug)]
struct Point2<T> {
    x: T,
    y: T
}

impl<T, U> Point<T, U> {
    pub fn x(&self) -> &T {
        &self.x
    }
    pub fn y(&self) -> &U {
        &self.y
    }
    pub fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point2<f64> {
    pub fn x(&self) -> &f64 {
        &self.x
    }
    pub fn y(&self) -> &f64 {
        &self.y
    }
}
