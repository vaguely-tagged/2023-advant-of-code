use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(filename: &str){

	let contents = File::open(filename).expect("cannot open file");
	let reader = BufReader::new(contents);

	let mut total = 0;

	for line in reader.lines() {

		let line = match line{
            Ok(line) => line,
            Err(error) => panic!("line could not be read {:?}",error)
        };

        let mut first_number: Option<String> = None;

        let mut buffer = String::new();
        for character in line.chars() {

        	buffer.push(character);

        	let does_contain = check_contain(&buffer);

        	if does_contain != None {
        		first_number = Some(does_contain.unwrap());
        		break;
        	}
        }

        let mut second_number: Option<String> = None;

        let mut buffer = String::new();
        for character in line.chars().rev() {

        	buffer.push(character);

        	let reversed_buffer = buffer.chars().rev().collect();
        	let does_contain = check_contain(&reversed_buffer);

        	if does_contain != None {
        		second_number = Some(does_contain.unwrap());
        		break;
        	}
        }


        let complete_number = first_number.unwrap() + &second_number.unwrap();

        let number: i32 = complete_number.parse().expect("should be number");
        total+=number;
	}

	println!("total is {}",total);
}

fn check_contain(buffer: &String) -> Option<String>{

	let word_vector: Vec<&str> = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

	for (i,word) in word_vector.iter().enumerate(){

		if buffer.contains(word){

			if i <= 9 {
				return Some(word_vector[i+10].to_string());
			} 

			return Some(word_vector[i].to_string());
		}
	}

	return None
}