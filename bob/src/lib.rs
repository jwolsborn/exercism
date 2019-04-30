//using inflections crate to check if characters are in alphabet
extern crate inflections;

use inflections::case::*;

pub fn reply(message: &str) -> &str {

	//first checks if there is no message. then checks for all caps and if the characters are in he alphabet
	//then checks last char for question mark
	//finally checks for just a question mark
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
