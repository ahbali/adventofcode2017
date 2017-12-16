#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
// use regex::Split;


use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let file = File::open("in1.txt").expect("file not found");
    let reader = BufReader::new(&file);
    // let g = reader.get_mut();
    let lines = reader.lines();
    // let f = g.lines();
    // let g = lines.by_ref();
    // println!("lines count = {}", lines.count());

    // let file2 = File::open("in2.txt").expect("file not found");
    // let reader2 = BufReader::new(&file);
    // let lines2 = reader2.lines();

    lazy_static! {
        // (?P<sign>\W)??P<sign>\W)?"sign",
        static ref RE: Regex = Regex::new(r"^(?P<reg>\w+)?\s?(?P<inc_dec>\w+)?\s?(?P<num>\W?\d+)?\s?if?\s?(?P<left_side>\w+)?\s?(?P<comp>\S+)?\s?(?P<right_side>\W?\d+)?").unwrap();
    // static ref RE: Regex = Regex::new(r"^(\w+)\s\((\d+)\)\s?-?>?\s?(\w+)?,?\s?(\w+)?,?\s?(\w+)?,?\s?(\w+)?,?\s?(\w+)?,?\s?(\w+)?,?\s?(\w+)?").unwrap();
    }

    let mut registers: HashMap<String, i32> = HashMap::new();
    // let mut ful_sys: Vec<Vec<String>> = Vec::new();
    // let mut line_counter = 0;
    let names = ["reg", "inc_dec", "num", "left_side", "comp", "right_side"];

    let mut max_value = 0;

    // map all registers
    for line in lines {
        match line {
            Ok(my_line) => {
                let capture = RE.captures(&my_line).unwrap();
                // println!("{}", my_line);
                // // let mut vec: Vec<&str> = Vec::new();
                for i in 0..names.len() {
                    let j = names[i];
                    let k = &capture[j];
                    print!("{:?} ", k);
                    // vec.push(k);
                }
                println!("");
                registers.entry(String::from(&capture["reg"])).or_insert(0);
                registers
                    .entry(String::from(&capture["left_side"]))
                    .or_insert(0);


                let reg = &capture["reg"];
                let inc_dec = &capture["inc_dec"];
                let num: i32 = (&capture["num"])
                    .trim()
                    .parse::<i32>()
                    .expect("parse error");
                // println!("num = {}", num);
                let left_side = &capture["left_side"];
                let left_side_value = registers.get(left_side).unwrap().clone();
                // println!("left side = {}", left_side_value);
                let right_side: i32 = (&capture["right_side"])
                    .trim()
                    .parse::<i32>()
                    .expect("parse error");

                // let c = registers.get(left_side).unwrap();

                match &capture["comp"] {
                    ">" => if left_side_value > right_side {
                        if inc_dec == "inc" {
                            let t = registers.entry(String::from(reg)).or_insert(0);
                            *t += num;
                            if max_value < *t {
                                max_value = *t;
                            }
                        } else {
                            let t = registers.entry(String::from(reg)).or_insert(0);
                            *t -= num;
                            if max_value < *t {
                                max_value = *t;
                            }
                        }
                    },
                    ">=" => if left_side_value >= right_side {
                        if inc_dec == "inc" {
                            let t = registers.entry(String::from(reg)).or_insert(0);
                            *t += num;
                            if max_value < *t {
                                max_value = *t;
                            }
                        } else {
                            let t = registers.entry(String::from(reg)).or_insert(0);
                            *t -= num;
                            if max_value < *t {
                                max_value = *t;
                            }
                        }
                    },
                    "<" => if left_side_value < right_side {
                        if inc_dec == "inc" {
                            let t = registers.entry(String::from(reg)).or_insert(0);
                            *t += num;
                            if max_value < *t {
                                max_value = *t;
                            }
                        } else {
                            let t = registers.entry(String::from(reg)).or_insert(0);
                            *t -= num;
                            if max_value < *t {
                                max_value = *t;
                            }
                        }
                    },
                    "<=" => if left_side_value <= right_side {
                        if inc_dec == "inc" {
                            let t = registers.entry(String::from(reg)).or_insert(0);
                            *t += num;
                            if max_value < *t {
                                max_value = *t;
                            }
                        } else {
                            let t = registers.entry(String::from(reg)).or_insert(0);
                            *t -= num;
                            if max_value < *t {
                                max_value = *t;
                            }
                        }
                    },
                    "==" => if left_side_value == right_side {
                        if inc_dec == "inc" {
                            let t = registers.entry(String::from(reg)).or_insert(0);
                            *t += num;
                            if max_value < *t {
                                max_value = *t;
                            }
                        } else {
                            let t = registers.entry(String::from(reg)).or_insert(0);
                            *t -= num;
                            if max_value < *t {
                                max_value = *t;
                            }
                        }
                    },
                    "!=" => if left_side_value != right_side {
                        if inc_dec == "inc" {
                            let t = registers.entry(String::from(reg)).or_insert(0);
                            *t += num;
                            if max_value < *t {
                                max_value = *t;
                            }
                        } else {
                            let t = registers.entry(String::from(reg)).or_insert(0);
                            *t -= num;
                            if max_value < *t {
                                max_value = *t;
                            }
                        }
                    },
                    _ => (),
                }
                // println!("");
            }
            Err(_) => println!("error"),
        }
    }
    // let c = reader.lines().count();
    // println!("lines count = {}", lines.count());
    // let reader2 = BufReader::new(&file);
    // let lines2 = reader2.lines();
    // lines2.cycle()

    // println!("lines2 count = {}", lines2.count());
    // resolve te problem

    // for line in lines2 {
    //     match line {
    //         Ok(my_line) => {
    //             let capture = RE.captures(&my_line).unwrap();
    //             // let names = ["reg", "inc_dec", "num", "left_side", "comp", "right_side"];
    //             let reg = &capture["comp"];
    //             let inc_dec = &capture["inc_dec"];
    //             let num: i32 = (&capture["num"])
    //                 .trim()
    //                 .parse::<i32>()
    //                 .expect("parse error");
    //             println!("num = {}", num);
    //             let left_side = &capture["left_side"];
    //             let left_side_value = &registers.get(left_side).unwrap().clone();
    //             println!("left side = {}", left_side_value);
    //             let right_side: i32 = (&capture["right_side"])
    //                 .trim()
    //                 .parse::<i32>()
    //                 .expect("parse error");

    //             // let c = registers.get(left_side).unwrap();

    //             match &capture["comp"] {
    //                 ">" => if *left_side_value > right_side {
    //                     if inc_dec == "inc" {
    //                         *registers.entry(String::from(reg)).or_insert(0) += num;
    //                     } else {
    //                         *registers.entry(String::from(reg)).or_insert(0) -= num;
    //                     }
    //                 },
    //                 ">=" => if *left_side_value >= right_side {
    //                     if inc_dec == "inc" {
    //                         *registers.entry(String::from(reg)).or_insert(0) += num;
    //                     } else {
    //                         *registers.entry(String::from(reg)).or_insert(0) -= num;
    //                     }
    //                 },
    //                 "<" => if *left_side_value < right_side {
    //                     if inc_dec == "inc" {
    //                         *registers.entry(String::from(reg)).or_insert(0) += num;
    //                     } else {
    //                         *registers.entry(String::from(reg)).or_insert(0) -= num;
    //                     }
    //                 },
    //                 "<=" => if *left_side_value <= right_side {
    //                     if inc_dec == "inc" {
    //                         *registers.entry(String::from(reg)).or_insert(0) += num;
    //                     } else {
    //                         *registers.entry(String::from(reg)).or_insert(0) -= num;
    //                     }
    //                 },
    //                 "==" => if *left_side_value == right_side {
    //                     if inc_dec == "inc" {
    //                         *registers.entry(String::from(reg)).or_insert(0) += num;
    //                     } else {
    //                         *registers.entry(String::from(reg)).or_insert(0) -= num;
    //                     }
    //                 },
    //                 "!=" => if *left_side_value != right_side {
    //                     if inc_dec == "inc" {
    //                         *registers.entry(String::from(reg)).or_insert(0) += num;
    //                     } else {
    //                         *registers.entry(String::from(reg)).or_insert(0) -= num;
    //                     }
    //                 },
    //                 _ => (),
    //             }



    //             // println!("");
    //             // println!("{:?}", capture);
    //         }
    //         Err(_) => println!("error"),
    //     }
    // }

    println!("{:?}", registers);
    println!("max = {:?}", registers.values().max().unwrap());
    println!("max_value = {}", max_value);
}
