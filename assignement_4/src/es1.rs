#![allow(dead_code, unused_imports)]
use rand;
use rand::Rng;

fn find_equal<'a, 'b>(s1: &'a str, s2: &'b str) -> Option<(&'a str, &'b str)>{
    for (i,(c1,c2)) in s1.chars().zip(s1.chars().skip(1)).enumerate(){
        let due_chars = c1.to_string() + c2.to_string().as_str();
        match s2.find(due_chars.as_str()) {
            Some(j) => return Some((&s1[i..i+2], &s2[j..j+2])),
            None => {}
        } 
    }
    return None;
}

fn lucky_slice<'a>(input_str:&'a str) ->Option<&'a str>{
    let mut new_str =  String::new() ;
    for _ in 0..input_str.len(){
        let random_number: u8 = rand::thread_rng().gen();
        new_str.push((random_number%(122-97+1) + 97) as char);
    }

    println!("Randomly built string: {}", new_str);

    match find_equal(new_str.as_str(), input_str){
        Some((_, s)) => return Some(s),
        None => return None
    }
}

pub fn main_es1(){
    println!("***\nEsercizio 1:");
    let mut found_it = false;
    while !found_it{
        match lucky_slice("ciao aurora"){
            Some(s) =>{
                                println!("Found it!!!\n {:?}", s);
                                found_it = true
                            },
            None => {}
        }
    }
}