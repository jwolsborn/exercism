use std::collections::HashSet;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    
	let mut solution = Vec::new();
	
	//definetly not the most elegant solution as it finds the mins in each column and max in each row and adds
	//those indexes as tuples to two hashsets.  The intersection of those hashsets is then pushed onto the return vec
	if input.len() != 0 && input[0].len() != 0{

		let mut mins = HashSet::new();
		let mut maxs = HashSet::new();

		for i in 0..input.len(){
			for j in 0..input[i].len(){
				if is_max(&input[i], &input[i][j]) == true{	
					maxs.insert((i, j));
				}
				
				if is_min(&input, &input[i][j], j) == true{
					mins.insert((i, j));
				}			
			}
		}
		let intersection: HashSet<_> = mins.intersection(&maxs).collect();
		for x in intersection.iter(){
			solution.push(**x);
		}
	}
	solution
}

//finds the max in each row
fn is_max(input: &Vec<u64>,val: &u64) -> bool {

	for i in 0..input.len(){
			if input[i] > *val {
			return false;
		}
	}

	true	
}

//finds the min in each column
fn is_min(input: &[Vec<u64>], val: &u64, col_idx: usize) -> bool{

	for i in 0..input.len(){
			if input[i][col_idx] < *val {
			return false;
		}
	}
	
	true
}	
