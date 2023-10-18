fn skip_prefix<'a>(telephone_number:&'a str, prefix:&str) ->&'a str{
    if &telephone_number[..prefix.len()] == prefix{
        return &telephone_number[prefix.len()-1..];
    }
    return telephone_number;
}

pub fn main_es7(){
    println!("{}", skip_prefix("0123456789", "0123"));
}