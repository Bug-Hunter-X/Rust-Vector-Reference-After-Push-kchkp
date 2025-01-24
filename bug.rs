fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let x = &vec[0];
    vec.push(3); // This line causes the error
    println!("x = {}", x);
}