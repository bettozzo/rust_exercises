pub fn is_it_luhn(str: &str) -> bool{
    if str.len() <= 1{
        return false;
    }
    let str = str.replace(" ", "");
    if contains_letters(&str){
        return false;
    }
    

    let mut somma:i32 = 0;
    for (i, num) in str.chars().enumerate(){
        let mut num = num.to_digit(10).unwrap() as i32;
        if i % 2 == 0{
            num *= 2;
            if num > 9{
                num -= 9;
            }
        }
        somma += num;
    }
    return somma % 10 == 0;
}

fn contains_letters(str: &str) -> bool{
    for c in str.chars(){
        if c.is_ascii_alphabetic(){
            return true;
        }
    }
    return false;
}
