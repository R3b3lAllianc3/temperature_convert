/*  November 29, 2022
    Program to convert celsius to fahrenheit and vice versa.  Input is specified in 44c or 44f, where 44 can be
    any number.  The program handles all erroneous cases of input.  This was mainly an exercise in learning the match 
    expression in Rust.
    May not be the optimal solution but a first-attempt in the world of Rust.
*/

use std::io;

fn main() {
    let mut temperature_input = String::new();
    let mut temperature_number: f32;
    let mut temperature_metric: char;

    println!("** Fahrenheit to Celsius conversion program **");
    
    loop {
        temperature_input.clear(); //clear string from possible previous iterations

        println!("\nEnter temperature followed by C or F to indicate temperature_metric used:");

        io::stdin().read_line(&mut temperature_input)
            .expect("Failed to read line");

        let temperature_input: Vec<&str> = temperature_input.trim().split_whitespace().collect();

        //We only process first two elements in the vector.  Either user entered temperature with space in between or not.
        //E.g. 44F or 44 F.

        match temperature_input.len() {
            /* We have two user input strings; now validate that the first is a number and second is a unit in c or f. */
            2 => {  let (number, metric) = (temperature_input[0], temperature_input[1]);
                    temperature_number = match number.trim().parse() {
                        Ok(num) => num,
                        Err(_) => { println!("Invalid numeric designation"); 
                                    continue;
                        }
                    };

                    /* Check if unit designation is valid; should be one character. */
                    let metric_vec: Vec<char> = metric.to_string().chars().collect();
                    match metric_vec.len() {
                        1 => {
                            //let temperature_metric = metric_vec[0];
                            match metric_vec[0] {
                                'C' | 'c' | 'F' | 'f' => temperature_metric = metric_vec[0],
                                _ => {  println!("invalid metric designation");
                                        continue; 
                                }
                            }
                        }
                        _ => { println!("Invalid metric designation");
                                continue;
                        }
                    }
            }
            /* We have one user input string; now validate that the first part is a number and second part is a unit in c or f. */
            1 => {  let number_and_metric = temperature_input[0]; 
                    //assert!(number_and_metric.ends_with("C"));
                    match number_and_metric.chars().last().unwrap() {
                        'C' | 'c' | 'F' | 'f' => {  let mut temperature_number_str = number_and_metric.to_string();
                                                    //The last character should be the unit designation.
                                                    temperature_metric = number_and_metric.chars().last().unwrap();
                                                    //Remove the last unit designation; the remainder string should be a number.
                                                    temperature_number_str.pop();
                                                    //If remainder string is not a number then it's an error else we are good.
                                                    temperature_number = match temperature_number_str.trim().parse() {
                                                        Ok(num) => num,
                                                        Err(_) => { println!("Invalid numeric designation"); 
                                                                    continue;
                                                        }
                                                    };
                        },
                        _ => { println!("Invalid metric designation");
                               continue;
                        }
                    }
            }
            //If we have more than 2 entries then it's not a valid entry.
            _ => { println!("Invalid entry");
                   continue;
            } 
        }
        let (final_answer, final_metric) = convert_between(temperature_number, temperature_metric);
        println!("{:?} {:?} is {:?} {:?}", temperature_number, temperature_metric, final_answer.unwrap(), final_metric.unwrap());
    }
}

fn convert_between(value: f32, metric_label: char) -> (Option<f32>, Option<char>) {
    match metric_label {
        'f' | 'F' => return (Some(((value - 32.0)*5.0)/9.0), Some('C')),
        'c' | 'C' => return (Some((value * (9.0/5.0))+32.0), Some('F')),
        _ => return (None, None)
    }
}