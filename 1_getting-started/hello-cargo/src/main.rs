use rand::Rng;
fn main() {
    // ! macro execution, not function
    println!("Hello, world!");
    let rand = rand::thread_rng()
        .gen_range(1..=100);
    println!("random num: [{rand}]");
}
