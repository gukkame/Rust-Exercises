fn main() {
    let mut i = 1;
    let something = loop {
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading");
   
        println!("Input: {} ", input.trim());
        if input.trim() == "The letter e" {
           
           break i.to_string();
        }
        i+=1
    };
    println!("Number of trials: {} ", something);
}
