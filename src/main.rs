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

    let v_for_access = vec![1, 2, 3, 4, 5, 6];

    let third = &v_for_access[2];

    println!("This is the third element of vector: {third}");

    let third = v_for_access.get(2);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    let third = v_for_access.get(20);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }
}
