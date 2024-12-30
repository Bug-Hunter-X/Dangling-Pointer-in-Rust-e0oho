fn main() {
    let mut v = vec![1, 2, 3];
    let value = v[0];
    v[0] = 10;
    println!("Original Value: {}", value);
    println!("Modified Vector: {:?}", v);
}