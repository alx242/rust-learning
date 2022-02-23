fn main(){
    // Cannot be changed
    const CANT_CHANGE:u32 = 55;
    println!("Constant is: {}", CANT_CHANGE);

    // Variables are default immutable also but can be set to be
    // mutable also
    let mut apa:u32 = 44;
    apa = apa + 1;
    println!("Mutable variables is: {}", apa);
}

