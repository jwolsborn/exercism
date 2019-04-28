pub fn encode(n: u64) -> String {
	let mut solution = String::new();
	let mut val = n;
	let mut val2 = n;
	let mut val3 = n;

	if n == 0 {
		solution = "zero".to_string();
		return solution;
	}
	
	//break apart n
	//quintillions
	if n >= 1000000000000000000 {
		val = n / 1000000000000000000;
		val2 = n % 1000000000000000000;
		solution.insert_str(solution.len(), &word(val));
		solution.insert_str(solution.len(), " quintillion ");	 
	}

	//quadrillion
	if val2 >= 1000000000000000 {
		val = val2 / 1000000000000000;
		val2 = val2 % 1000000000000000;
		if val >= 100{
			val3 = val % 100;
			val = val / 100;
			solution.insert_str(solution.len(), &word(val));
			solution.insert_str(solution.len()," hundred ");
		}else{
            val3 = val % 100;
		}

		if val3 >= 20 {
			solution.insert_str(solution.len(), &word(val3));
			val3 = val3 % 10;
			if val3 > 0 {
				solution.insert_str(solution.len(), "-");
				solution.insert_str(solution.len(), &word(val3));
			}
		}else if val3 > 0{
          	solution.insert_str(solution.len(), &word(val3));
        }

		solution.insert_str(solution.len(), " quadrillion ");
		
	}
	
	//trillions
	if val2 >= 1000000000000 {
		val = val2 / 1000000000000;
		val2 = val2 % 1000000000000;                      		
		if val >= 100{                                       			
			val3 = val % 100;
			val = val / 100;
			solution.insert_str(solution.len(), &word(val));
			solution.insert_str(solution.len()," hundred ");
		}else{
            val3 = val % 100;
		}

		if val3 >= 20 {
			solution.insert_str(solution.len(), &word(val3));
			val3 = val3 % 10;
			if val3 > 0 {
				solution.insert_str(solution.len(), "-");
				solution.insert_str(solution.len(), &word(val3));
			}
		}else if val3 > 0{
          	solution.insert_str(solution.len(), &word(val3));
        }

		solution.insert_str(solution.len(), " trillion ");
		
	}


	//billions
	if val2 >= 1000000000 {
		val = val2 / 1000000000;
		val2 = val2 % 1000000000;
		if val >= 100{
			val3 = val % 100;
			val = val / 100;
			solution.insert_str(solution.len(), &word(val));
			solution.insert_str(solution.len()," hundred ");
		}else{
			val3 = val % 100;
		}

		if val3 >= 20 {
			solution.insert_str(solution.len(), &word(val3));
			val3 = val3 % 10;
			if val3 > 0 {
				solution.insert_str(solution.len(), "-");
				solution.insert_str(solution.len(), &word(val3));
			}
		}else if val3 > 0{
         	solution.insert_str(solution.len(), &word(val3));
        }

		solution.insert_str(solution.len(), " billion ");
		
	}                                                 

	//millions
	if val2 >= 1000000 {
		val = val2 / 1000000;
		val2 = val2 % 1000000;
		if val >= 100{
			val3 = val % 100;
			val = val / 100;
			solution.insert_str(solution.len(), &word(val));
			solution.insert_str(solution.len()," hundred ");
		}else{
            val3 = val % 100;
		}

		if val3 >= 20 {
			solution.insert_str(solution.len(), &word(val3));
			val3 = val3 % 10;
			if val3 > 0 {
				solution.insert_str(solution.len(), "-");
				solution.insert_str(solution.len(), &word(val3));
			}
		}else if val3 > 0{
			solution.insert_str(solution.len(), &word(val3));
		}

		solution.insert_str(solution.len(), " million ");
		
	}

	//thousands
	if val2 >= 1000 {
		val = val2 / 1000;
		val2 = val2 % 1000;
		if val >= 100{
			val3 = val % 100;
			val = val / 100;
			solution.insert_str(solution.len(), &word(val));
			solution.insert_str(solution.len()," hundred ");
		}else{
            val3 = val % 100;
		}

		if val3 >= 20 {
			solution.insert_str(solution.len(), &word(val3));
			val3 = val3 % 10;
			if val3 > 0 {
				solution.insert_str(solution.len(), "-");
				solution.insert_str(solution.len(), &word(val3));
			}
		}else if val3 > 0{
         	solution.insert_str(solution.len(), &word(val3));
        }

		solution.insert_str(solution.len(), " thousand ");
		
	}

	//hundreds


	if val2 >= 100{
		val3 = val2 % 100;
		val2 = val2 / 100;
		solution.insert_str(solution.len(), &word(val2));
		solution.insert_str(solution.len(), " hundred ");
	}else{
		val3 = val2 % 100;
	}

	if val3 >= 20 {
    	solution.insert_str(solution.len(), &word(val3));
    	val3 = val3 % 10;
    	if val3 > 0 {
    		solution.insert_str(solution.len(), "-");
    		solution.insert_str(solution.len(), &word(val3));
    	}
    }else if val3 > 0{
     	solution.insert_str(solution.len(), &word(val3));
    }
                                                             
	
	if n <= 19 {
		solution = word(n);
	}

	


	solution.trim().to_string() 
}

fn word(n: u64) -> String {
	
	match n {
		1 => "one",
		2 => "two",
		3 => "three",
		4 => "four",
		5 => "five",
		6 => "six",
		7 => "seven",
		8 => "eight",
		9 => "nine",
		10 => "ten",
		11 => "eleven",
		12 => "twelve",
		13 => "thirteen",
		14 => "fourteen",
		15 => "fifteen",
		16 => "sixteen",
		17 => "seventeen",
		18 => "eighteen",
		19 => "nineteen",
		20...29 => "twenty",
		30...39 => "thirty",
		40...49 => "forty",
		50...59 => "fifty",
		60...69 => "sixty",
		70...79 => "seventy",
		80...89 => "eighty",
		90...99 => "ninety",
		_ => "how did you get here?",
	}.to_string()	

}
