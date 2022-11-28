fn main() {
    let v: Vec<i32> = Vec::new();

    println!("This is empty vector! {:?}", v);

    let v_two = vec![1, 2, 3];

    println!("This is non-empty vector! {:?}", v_two);

    let mut v_mut = vec![1];

    println!("This is mutable vector! {:?}", v_mut);

    v_mut.push(6);
    v_mut.push(8);

    println!("This is mutable vector! {:?}", v_mut);
}
