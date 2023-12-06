use std::fs;
use std::str;
use std::collections::HashMap;


const number_string: [&str; 19] = ["one", "two", "three", 
                "four", "five", "six", 
                "seven", "eight", "nine",
                "0", "1", "2", "3", 
                "4", "5", "6", 
                "7", "8", "9"];


fn main() {
    // --snip--
    let var_name = "givendata.txt";
    let file_path: &str = var_name;

    println!("In file {}", file_path);

    let number_map: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("zero", "0")
    ]);

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
        println!("\n\n{:?}", &line);

        // error use of moved value
        let chars: Vec<char> = line.chars().collect();

        let mut right_number: usize = 10000;       // start with large number
        let mut left_number: usize = 0;

        let mut right_string: &str = "";
        let mut left_string: &str = "";

        // string.find()
        // for -> get last digit instead of reversed
        for number in number_string{
            match line.find(&number){
                Some(x) => {
                    println!("right_number | Encontrado {:?} em {:?}", number, x);
                    if x < right_number{
                        right_number = x;
                        if number_map.contains_key(&number){
                            match number_map.get(&number){
                                Some(y) => {
                                    right_string = y;

                                }
                                None =>{
                                    right_string = "";
                                }
                            }
                        }
                        else{
                            right_string = number;
                        }
                        println!("right_number | Escolhido {:?} em {:?}", right_string, x);
                    }
                }
                None => {}
            }
        }
        // right_number += 1;
        println!("{right_number}");
        for number in number_string{
            // let segment = &line[right_number..];
            // println!("segmento {segment} | indice offset {right_number}");
            
            match line.rfind(&number){
                Some(x) => {
                    println!("left_number | Encontrado {:?} em {:?}", number, x);
                    if x >= left_number{
                        left_number = x;
                        if number_map.contains_key(&number){
                            match number_map.get(&number){
                                Some(y) => {
                                    left_string = y;
                                }
                                None =>{
                                    right_string = "";
                                }
                            }
                        }
                        else{
                            left_string = number;
                        }
                        println!("left_number | Escolhido {:?} em {:?}", left_string, x);
                    }
                    
                    else{
                        println!("{:?} {:?}", x, number);
                    }
                }
                None => {}
            // println!("{:?}", line.find(number));
            }
        }

        calibration = String::from(right_string);
        calibration += left_string;

        println!("Calibracao {:?}", calibration);
        
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
    }

    println!("{:?}", sum_calibration);
}

