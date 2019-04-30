pub fn is_leap_year(year: u64) -> bool {
	//simply uses modulo to check the year based upon the conditions provided    
	if year % 4 != 0 {
		return false
    } else if year % 4 == 0 && year % 100 == 0 {
		if year % 400 == 0 {
			return true
		} else {
			return false
		}
	} else if year % 4 == 0 {
		return true
	} 
	
	return false
}
