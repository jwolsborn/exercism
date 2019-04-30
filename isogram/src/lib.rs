pub fn check(candidate: &str) -> bool {
    //checks for empty string
	if candidate.is_empty() == true{
		return true;
	}
	//sets all to lowercase then keeps a duplicate count.  dupe has to be two as the character gets compared to itself
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
