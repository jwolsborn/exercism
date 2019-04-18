pub fn build_proverb(list: &[&str]) -> String {
	
	let mut final_string = String::new();
	
	if list.len() == 1{

		final_string.push_str(&format!("And all for the want of a {}.", list[0]));
		return final_string
	
	}else if list.len() >= 2{
		
		for i in 0..list.len()-1{
			final_string.push_str(&format!("For want of a {} the {} was lost.\n", list[i], list[i+1]));
		}

		final_string.push_str(&format!("And all for the want of a {}.", list[0]));
		return final_string			
		
	}		
	final_string
}
