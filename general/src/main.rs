fn failing_example() {
    let x = String::from("marcell");


    // we need to burrow here so we can use the string afterwards
    moved_here(x);

    println!("will fail {} ", x); // the value was moved here
}

fn passing_example() {
    let x = String::from("marcell");


    // we need to burrow here so we can use the string afterwards
    moved_here(&x);

    println!("will fail {} ", x); // the value was moved here
}


fn main() {
    passing_example(); 
}

fn moved_here(x: &String) {
    println!("will fail {} ", x);
}
