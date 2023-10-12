use std::collections::HashMap;

pub fn recognise_owner(map:&HashMap<String,String>, plate:String) -> Option<&String>{
    map.get(&plate)
}
