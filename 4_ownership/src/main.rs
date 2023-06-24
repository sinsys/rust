use std::io;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("\n_VARIABLES________\n");
    variables();
    println!("\n_DATA_TYPES________\n");
    data_types();
    // println!("\n_BREAK_STUFF_______\n");
    // break_stuff();
    println!("\n_FUNCTIONS________\n");
    accept_param(5);
    print_label(5, 'D');
    expressions();
}

fn variables() {
    // Mutable variable. We can alter but not change data type
    let mut mutable = 1;
    println!("The value of mutable is {mutable}");
    mutable = 2;
    println!("The value of mutable is {mutable}\n");

    // Shadowing. We can reassign. We can even change types!
    let spaces = "    ";
    println!("The value of our spaces is [{spaces}]");
    let spaces = spaces.len();
    println!("The value of our spaces is [{spaces}]\n");

    let shadow = 3;
    println!("The value of shadow is {shadow}");
    let shadow = shadow + 1;
    println!("The value of shadow is {shadow}");
    {
        // And scoped into a new memory copy!
        let shadow = shadow * 2;
        println!("The value of shadow is {shadow}");
    }
    // And now refs the original
    println!("The value of shadow is {shadow}\n");

    // Reads constant outside of func
    println!("The value of our constant is {THREE_HOURS_IN_SECONDS}");
}

fn data_types() {
    // Integers
    let int8: u8 = 255;
    println!("u8 [{int8}]");

    // Floats
    let f32: f32 = 3.45;
    println!("f32 (single precision) [{f32}]");
    let f64: f64 = 6.78;
    println!("f64 (double precision) [{f64}]");

    // Calcs
    let sum = 5 + 10;
    println!("sum is [{sum}]");
    let diff = 95.5 - 4.3;
    println!("diff is [{diff}]");
    let product = 4 * 30;
    println!("product is [{product}]");
    let quotient = 56.7 / 32.2;
    println!("quotient is [{quotient}]");
    let truncated = -5 / 3;
    println!("truncated is [{truncated}]");
    let remainder = 43 % 5;
    println!("remainder is [{remainder}]");

    // Misc
    let bool = false;
    println!("bool is [{bool}]");
    let char = 'z';
    println!("char is [{char}]");
    let z: char = 'Z';
    println!("z is [{z}]");
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat is [{heart_eyed_cat}]");

    // Complex types
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (foo, bar, baz) = tuple;
    println!("tuple vals are [{foo}] [{bar}] [{baz}]");
    let tuple_extract = tuple.1; // 6.4
    println!("tuple_extract is [{tuple_extract}]");
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let array_fill = [1; 5]; // [1,1,1,1,1]
    let array_el = array[4];
    let array_fill_el = array_fill[4];
    println!("array index 4 extract is [{array_el}]");
    println!("array_fill index 4 extract is [{array_fill_el}]");
}

fn break_stuff() {
    let a = [1,2,3,4,5];
    println!("Guess");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index.trim()
        .parse()
        .expect("Index entered was not a number");
    
    let element = a[index];
    println!("The value at index {index} is: {element}");
}

fn accept_param(x: i32) {
    println!("The value of x is: {x}")
}

fn print_label(x: i32, label: char) {
    println!("The measurement is: {x}{label}")
}

fn expressions() {
    let y = {
        let x = 3;
        x + 1 // Returns 4
    };

    println!("The value of y is: {y}");
}