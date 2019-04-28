pub fn check(candidate: &str) -> bool {
    
	if candidate.is_empty() == true{
		return true;
	}
	
	for x in candidate.to_lowercase().chars(){
		let mut dupes:u64 = 0;
		if x != ' ' && x != '-'{
			for y in candidate.to_lowercase().chars(){
				if x == y {
					dupes = dupes +1;
				}
				if dupes == 2{
					return false;
				}
			}
		}
	}
	true
}
