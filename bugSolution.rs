fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let x = vec[0]; // Now we copy the value, avoiding the reference issue
    vec.push(3);
    println!("x = {}", x);
}
