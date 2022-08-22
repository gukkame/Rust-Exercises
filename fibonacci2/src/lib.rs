pub fn fibonacci(n: u32) -> u32 {
    if n == 1 {
		return 1;
	}

	let mut sum = 0;
	let mut last = 0;
	let mut curr = 1;
	for _i in 1..n {  //u32 mean only positive numbers
		sum = last + curr;
		last = curr;
		curr = sum;
	}
	sum
}