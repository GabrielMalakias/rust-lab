fn main() {
    let vector: Vec<i32> = (1..19).collect();

    println!("{:?}", vector.first());
    println!("{:?}", vector.last());

    println!("{:?}", vector)
}
