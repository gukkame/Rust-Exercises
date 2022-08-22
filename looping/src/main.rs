use std::io;
fn main() {
    let mut i = 1;
    let something = loop {
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading");

        if input.trim() == "The letter e" {
           
           break i.to_string();
        }
        i+=1
    };
    println!("It took you {} trial to get the right answer", something);
}
