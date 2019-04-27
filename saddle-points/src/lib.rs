pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    
	let mut solution = Vec::new();
	
	for i in 0..input.len()-1{
		let mut iter_one:usize = 0;
		let iter_two:usize = i;

		for j in 0..input[i].len()-1{
			let mut min:u64 = input[i][0];
			if min >= input[i][j]{
				min = input[i][j];
				iter_one = j;
			}
		}
		
		let mut max:u64 = input[i][iter_one];

		for k in 0..input.len()-1{
			if max < input[k][iter_one]{
				max = input[k][iter_one];
				break;
			}
		}
		
		if max == input[i][iter_one]{
			solution.push((iter_two, iter_one));
		}	
		
	 }
	
	solution
}
