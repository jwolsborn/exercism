pub fn collatz(n: u64) -> Option<u64> {
	
	match n {
	0 => None,
	_ => {
		let mut count = 0;
		let mut m = n;
		while m != 1 {
			if m % 2 == 0 {
				m = m / 2;
				count = count + 1;
			}else{
				m = m * 3 + 1;
				count = count + 1;
			}	
		}

		Some(count)},
	}
}
