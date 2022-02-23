fn main() {
    // Ordinary types
    let a_string = "Bang!";
    let a_float = 1.2;
    let a_boolean = true;

    println!();
    println!("a string:\t {}", a_string);
    println!("a float:\t {}", a_float);
    println!("a bool:\t\t {}", a_boolean);

    // Different integer typs
    let default = 10;      // i32 by default
    let defu32:u32 = 20;   // unsigned
    let defi32:i32 = 5-15; // signed
    let iarch:isize = 10;  // signed architecture type, 32/64 depends on machine
    let uarch:usize = 30;  // unsigned equivalent

    println!();
    println!("default:\t {}", default);
    println!("def u32:\t {}", defu32);
    println!("def i32:\t {}", defi32);
    println!("def iarch:\t {}", iarch);
    println!("def uarch:\t {}", uarch);

    let special_character = '@'; //default
    let alphabet:char = 'A';
    let emoji:char = '😁';

    println!();
    println!("special:\t {}", special_character);
    println!("alpha:\t\t {}", alphabet);
    println!("emoji:\t\t {}", emoji);
}
