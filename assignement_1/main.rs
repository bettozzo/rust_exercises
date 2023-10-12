use std::collections::HashMap;
const SPEED_LIGHT:i32 = 299792458;

fn main() {
    let to_be_reversed:&str = "ciao";
    println!("1.\nString: {}\nreversed: {}\n", to_be_reversed, string_reverse(to_be_reversed));

    let s1:i32 = 5;
    let s2:i32 = 10;
    println!("2.\nnum1: {}, num2: {}\nmax(num1,num2)={}\n", s1, s2, bigger(s1,s2));

    let m1:i32=2;
    let m2:f32=6.134;
    let m3:f64=23.4;
    println!("3.\nnum1: {}, num2: {}, num3: {}\nnum1*num2*num3={}\n", m1, m2, m3, multiply(m1,m2,m3));

    let mass:f32 = 40.5;
    println!("4.\nMass: {}\nE=mc2->Energy: {}\n", mass, e_equals_mc_squared(mass));

    let mut vector_min_max:Vec<i32> = Vec::new();    vector_min_max.push(4);    vector_min_max.push(7);    vector_min_max.push(9);    vector_min_max.push(2);    vector_min_max.push(6);
    let max_and_min:(i32, i32) = max_min(vector_min_max.clone());
    println!("5.\nVector: {:?}\nmin: {}, max:{}\n", vector_min_max, max_and_min.0, max_and_min.1);

    let string_with_es:String = String::from("Oh quante belle figlie, Madama Doré");
    println!("6.\nOriginal: {}\nFarquaaded: {}\n", string_with_es, lord_farquaad(string_with_es.clone()));

    let mut furniture: HashMap<String,f32> = HashMap::new();    furniture.insert("wardrobe".to_string(), 10.4);    furniture.insert("bed".to_string(), 20.2);    furniture.insert("desk".to_string(), 30.1);    furniture.insert("table".to_string(), 40.85);
    let element:String = String::from("desk");
    println!("7.\nHashMap: {:?}, element:{}\nkey:{}\n", furniture, element, hash_map_contains(&furniture, &element));

    let to_be_appended:String = String::from("Hello ");
    println!("8.\nOriginal: {}\nAppended: {}\n", to_be_appended, append(to_be_appended.clone()));

    let number_armstrong:i32 = 153;
    println!("9.\nNumber: {}\nis an Armstrong number? {}\n", number_armstrong, is_armstrong(number_armstrong));

    let matrish: ((i32,i32), (i32,i32)) = ((1,2), (3,4));
    let matrish_trans = transpose(matrish.clone());
    println!("10.\nMatrix:\t{:?},\tTransposed:\t{:?}\n\t\t{:?}\t\t\t\t{:?}\n",matrish.0, matrish_trans.0, matrish.1, matrish_trans.1);
}

fn string_reverse(str:&str) -> String{
    //in verità questo non funziona in tutti i casi, se vuoi quello chiedi a salvatore
    let mut output:String = String::from("");
    for i in 0..str.len(){
        output.push(str.chars().nth(str.len()-1-i).unwrap());
    }
    return output;
}

fn bigger(n1:i32, n2:i32) -> i32{
    return n1*(n1>n2) as i32 +n2*(n1<=n2) as i32;
}

fn multiply(n1:i32, n2:f32, n3:f64) -> f64{
    return n1 as f64 * n2 as f64 * n3;
}

fn e_equals_mc_squared(mass:f32) -> f32{
    return mass * SPEED_LIGHT as f32;
}

fn max_min(vect: Vec<i32>) -> (i32, i32) {
    return (*vect.iter().min().unwrap(), *vect.iter().max().unwrap());
}

fn lord_farquaad(str:String) -> String{
    return str.replace("e", "💥");
}

fn hash_map_contains(hash_map:&HashMap<String,f32>, el:&String) -> f32{
    match hash_map.get(el) {
        None => return -1.0,
        Some(t) => return *t
    } ;
}

fn append(mut str:String) -> String{
    str.push('f');    str.push('o');    str.push('o');    str.push('b');    str.push('a');    str.push('r');
    return str;
}

fn is_armstrong(num:i32) -> bool{
    let digits: Vec<_> = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let length = digits.len() as u32;
    let mut sum:i32 = 0;
    for digit in digits{
        sum += digit.pow(length) as i32;
    }
    return sum == num;
}

fn transpose(mat: ((i32,i32), (i32,i32))) -> ((i32,i32), (i32,i32)){
    let mut first_row:(i32,i32) = mat.0;
    let mut second_row:(i32,i32) = mat.1;

    let tmp = first_row.1;
    first_row.1 = second_row.0;
    second_row.0 = tmp;

    return (first_row, second_row);
}
