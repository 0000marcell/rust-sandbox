fn main() {
    let x = String::from("marcell");
    let y = x;

    println!("will fail {} ", x); // the value was moved here
}
