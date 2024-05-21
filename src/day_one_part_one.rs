use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_lines(fileame: &str){

    let file = match File::open(fileame) {
        Ok(file) => file,
        Err(error) => panic!("file open error {:?}",error)
    };

    let reader = BufReader::new(file);
    let mut total = 0;

    for line in reader.lines() {

        let line = match line{
            Ok(line) => line,
            Err(error) => panic!("line could not be read {:?}",error)
        };

    
        let mut first_number: Option<char> = None;
        for character in line.chars(){
            if character.is_ascii_digit(){
               first_number = Some(character);
               break;
            }
        }

        let mut second_num: Option<char> = None;
        for character in line.chars().rev(){
            if character.is_ascii_digit(){
                second_num = Some(character);
                break;
            }
        }

        if first_number == None || second_num == None {
            panic!("cannot be none");
        }

        let first_number_string: String = first_number.unwrap().to_string();
        let second_number_string: String = second_num.unwrap().to_string();

        let number_string: String = first_number_string + &second_number_string;

        let number: i32 = number_string.parse()
            .expect("cannot convert to i32");
        total+=number;
    }

    println!("total is {}",total);
}