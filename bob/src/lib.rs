extern crate inflections;

use inflections::case::*;

pub fn reply(message: &str) -> &str {
    let message = message.trim();		
	if message.is_empty(){
		return "Fine. Be that way!";
	}else if is_upper_case(message) && message.chars().any(|x| x.is_alphabetic()){
		if message.ends_with("?") {
			return "Calm down, I know what I'm doing!";
		}
		return "Whoa, chill out!";
	}else if message.ends_with("?") {
		return "Sure.";
	}
	"Whatever."	
}
