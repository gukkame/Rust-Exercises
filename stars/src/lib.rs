pub fn stars(n: u32) -> String {
    let mut i = 0;
    let mut s = String::new();
    while i < 2_i32.pow(n) {
        s.push('*');
        i+=1;
    }
    println!("{}", s);
    s
}


