pub fn prev_prime(nbr: u64) -> u64 {
    let mut i = 2;
    let mut res = 0;
    let mut is_prime = true;

    if nbr == 0 || nbr == 2 || nbr == 1 {
        return 0;
    }
    while i < nbr {
        is_prime = true;
        // for a in 2..i {
        // println!("A {:?}, {:?}", nbr,a);
        let mut a = 2;
        while a < i {
            if i % a == 0 {
                is_prime = false;
            }
            a += 1;
        }
        // }

        if is_prime {
            res = i;
        }
        i += 1;
    }

    res
}

fn main() {
    println!("The previous prime number before 3 is: {}", prev_prime(478140));
    println!("The previous prime number before 2 is: {}", prev_prime(2));
    println!("The previous prime number before 5 is: {}", prev_prime(5));
    println!("The previous prime number before 9 is: {}", prev_prime(9));
}
