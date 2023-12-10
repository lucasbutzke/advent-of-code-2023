use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::result;
use core::num::ParseIntError;


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> 
    io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
            let file = File::open(filename)?;
            Ok(io::BufReader::new(file).lines())
        }


struct GameLine{
    ID: u32,
    green: usize,
    red: usize,
    blue: usize,
    power: usize,
    green_few: usize,
    blue_few: usize,
    red_few: usize
}

impl GameLine{
    pub fn process_game_line(&mut self,line: String)
        // dados: &mut HashMap<u32, ((&str, usize), (&str, usize), (&str, usize))>
            // -> io::Result<Vec<String>>
    {
        
        let data: Vec<&str> = line.split(":").collect();

        println!("{:?}", data);

        if self.get_game_max_color_qnt(data[1]){
            self.get_game_index(data[0]);
        }
        else 
        {
            println!("Deu ruim!!!! \n");
        }

        self.get_game_power(data[1]);

        println!("Game IDs sum: {:?}", self.ID);
    }

    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    pub fn get_game_index(&mut self, data: &str){
        println!("String index: {:?}", data);

        let game_index_str: Vec<&str> = data.split_whitespace().collect();

        match game_index_str[game_index_str.len() - 1].trim().parse::<u32>(){
            Ok(result) => {
                println!("Index: {:?}", result);
                // self.ID = result;
                self.ID += result;
            }
            Err(e) =>{
                println!{"{e}"};
            }
        }

    }

    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    pub fn get_game_max_color_qnt(&mut self, data: &str) -> bool{
        println!("current game colors: {:?}", data);

        let data_iter: Vec<&str> = data.trim().split(";").collect();

        println!("{:?}", data_iter);

        let mut flag_return: bool = true;

        for values in &data_iter{
            let value: Vec<&str> = values.split_whitespace().collect();

            println!("actual line red: {:?}", value);

            match value.iter().position(|&x| x == "red" || x == "red,") {
                Some(result) => {
                    println!("{:?}", value[result - 1]);
                    match value[result - 1].trim().parse::<usize>(){
                        Ok(x) => {
                            println!("valor: {:?} | red: {:?}", x, self.red);
                            if x > self.red{
                                return false;
                                // self.red = result;
                                // flag_return = false;
                            }
                        }
                        Err(e) => {
                            println!("{e}");
                        }
                    }
                    
                }
                None => {
                    // println!("Nada");
                }
            }
        }

        let data_iter: Vec<&str> = data.trim().split(";").collect();
        
        for values in &data_iter{
            let value: Vec<&str> = values.split_whitespace().collect();

            println!("actual line blue: {:?}", value);

            match value.iter().position(|&x| x == "blue," || x == "blue") {
                Some(result) => {
                    println!("{:?}", value[result - 1]);
                    match value[result - 1].trim().parse::<usize>(){
                        Ok(x) => {
                            println!("valor: {:?} | blue: {:?}", x, self.blue);
                            if x > self.blue{
                                return false;
                                // flag_return = false;
                            }
                        }
                        Err(e) => {
                            println!("{e}");
                        }
                    }
                }
                None => {
                    // println!("Nada");
                }
            }
        }

        let data_iter: Vec<&str> = data.trim().split(";").collect();

        for values in &data_iter{
            let value: Vec<&str> = values.split_whitespace().collect();

            println!("actual line green: {:?}", value);

            match value.iter().position(|&x| x == "green" || x == "green,") {
                Some(result) => {
                    println!("{:?}", value[result - 1]);
                    match value[result - 1].trim().parse::<usize>(){
                        Ok(x) => {
                            println!("valor: {:?} | green: {:?}", x, self.green);
                            if x > self.green{
                                return false;
                                // flag_return = false;
                            }
                        }
                        Err(e) => {
                            println!("{e}");
                        }
                    }
                }
                None => {
                    // println!("Nada");
                }
            }
        }

        return true;
    }

    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    pub fn get_game_power(&mut self, data: &str) -> bool{
        println!("current game colors: {:?}", data);

        let data_iter: Vec<&str> = data.trim().split(";").collect();

        println!("{:?}", data_iter);

        let mut flag_return: bool = true;

        self.red_few = 0;
        self.green_few = 0;
        self.blue_few = 0;

        for values in &data_iter{
            let value: Vec<&str> = values.split_whitespace().collect();

            println!("actual line red: {:?}", value);

            match value.iter().position(|&x| x == "red" || x == "red,") {
                Some(result) => {
                    println!("{:?}", value[result - 1]);
                    match value[result - 1].trim().parse::<usize>(){
                        Ok(x) => {
                            println!("valor: {:?} | red: {:?}", x, self.red_few);
                            if x > self.red_few{
                                self.red_few = x;
                            }
                        }
                        Err(e) => {
                            println!("{e}");
                        }
                    }
                    
                }
                None => {
                    // println!("Nada");
                }
            }
        }

        let data_iter: Vec<&str> = data.trim().split(";").collect();
        
        for values in &data_iter{
            let value: Vec<&str> = values.split_whitespace().collect();

            println!("actual line blue: {:?}", value);

            match value.iter().position(|&x| x == "blue," || x == "blue") {
                Some(result) => {
                    println!("{:?}", value[result - 1]);
                    match value[result - 1].trim().parse::<usize>(){
                        Ok(x) => {
                            println!("valor: {:?} | blue: {:?}", x, self.blue_few);
                            if x > self.blue_few{
                                self.blue_few = x;
                            }
                        }
                        Err(e) => {
                            println!("{e}");
                        }
                    }
                }
                None => {
                    // println!("Nada");
                }
            }
        }

        let data_iter: Vec<&str> = data.trim().split(";").collect();

        for values in &data_iter{
            let value: Vec<&str> = values.split_whitespace().collect();

            println!("actual line green: {:?}", value);

            match value.iter().position(|&x| x == "green" || x == "green,") {
                Some(result) => {
                    println!("{:?}", value[result - 1]);
                    match value[result - 1].trim().parse::<usize>(){
                        Ok(x) => {
                            println!("valor: {:?} | green: {:?}", x, self.green_few);
                            if x > self.green_few{
                                self.green_few = x;
                            }
                        }
                        Err(e) => {
                            println!("{e}");
                        }
                    }
                }
                None => {
                    // println!("Nada");
                }
            }
        }

        flag_return = true;
        
        println!("\nred_few {:?} blue_few {:?} green_few {:?}\n", self.red_few, self.blue_few, self.green_few);
        
        self.power += self.blue_few * self.red_few * self.green_few;

        return flag_return;
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();

    // Game ID: (green: number, red: number, blue:number)
    // Game max number of each color shown each time
    // let mut number_map: HashMap<u32, ((&str, usize), (&str, usize), (&str, usize))> = HashMap::new();

    //@TODO melhorar parte 2
    let mut gamestruct = GameLine{ID:0, power:0, green:13, green_few:0,
                        red:12, red_few:0, blue_few:0, blue:14};

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(args[1].clone()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(linha) = line {
                println!("{}", linha);

                gamestruct.process_game_line(linha); // , &mut number_map

            }
        }
    }

    println!("Game IDs sum: {:?} | Game total power: {:?}", gamestruct.ID, gamestruct.power);

}
        