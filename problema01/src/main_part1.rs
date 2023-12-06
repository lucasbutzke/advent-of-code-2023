use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str;

fn main() {
    // --snip--
    let file_path: &str = "givendata.txt";

    println!("In file {}", file_path);

    // let contents = fs::read_to_string(file_path)
    //     .expect("Should have been able to read the file");

    let contents: String = match fs::read_to_string(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("my error message: {}", err);
            std::process::exit(1);
        }
    };
    
    let lines: str::SplitWhitespace<'_> = contents.split_whitespace();

    // scan strings
    let mut calibration: String = "".to_string();
    let mut vec_calibration: Vec<String> = vec![];
    
    for line in lines{
        // error use of moved value
        let chars: Vec<char> = line.chars().collect();
        let chars_rev: String = line.chars().rev().collect::<String>();
        let chars_rev: Vec<char> = chars_rev.chars().collect();

        let right_number: char;
        let left_number: char;

        // println!("{:?}", chars);
        // get number from the right
        for letra in chars{
            match letra.is_numeric(){
                true => {
                    right_number = letra;
                    calibration = String::from(right_number);
                    break;
                },
                false => {
                    // println!("{:?}", letra)
                },
                _ => {
                    // println!("{:?}", letra)
                }
            }
        }

        // println!("{:?}", chars_rev);
        // get number from the left
        for letra in chars_rev{
            match letra.is_numeric(){
                true => {
                    left_number = letra;
                    calibration.push(left_number); 
                    break;
                },
                false => {
                    // println!("{:?}", letra)
                },
                _ => {
                    // println!("{:?}", letra)
            }
            }
        }

        vec_calibration.push(calibration.clone());
    }
    println!("{:?}", vec_calibration);
    
    // sum all calibration values
    let mut sum_calibration: u32 = 0;

    for cal in vec_calibration{
        let temp = cal.parse::<String>().unwrap();
        match temp.trim().parse::<u32>(){
            Ok(result) => {
                sum_calibration += result;
            },
            Err(e) => {
                println!{"{e}"};
            }
        }
        // sum_calibration += temp.trim().parse();
    }

    println!("{:?}", sum_calibration);
}

