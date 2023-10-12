#![allow(dead_code)]
#![allow(unused_imports)]
pub mod es1;
pub mod es2;
pub mod es3;
pub mod es4;
pub mod es5;
pub mod es6;
pub mod es7;

use crate::es1::is_it_luhn;
use crate::es2::*;
use crate::es4::*;
use crate::es5::*;
use crate::es6::*;
use crate::es7::test::*;
use std::collections::HashMap;

fn main() {
    // let lihn_number = "4539 3195 0343 6467";
    // println!("*** Es 1:\nNumber: {}\nIs it luhn-? {}\n", lihn_number, is_it_luhn(lihn_number));
    
    // let ip = IPAdd::IPv4((127,0,0,1));
    // let ip2 = IPAdd::IPv6((
    //     AddressGroup::new(13, 3, 12, 4),
    //     AddressGroup::new(11, 3, 12, 4),
    //     AddressGroup::new(0, 3, 12, 4),
    //     AddressGroup::new(6, 6, 11, 4),
    //     AddressGroup::new(12, 3, 9, 4),
    //     AddressGroup::new(6, 3, 0, 4),
    //     AddressGroup::new(8, 2, 12, 4),
    //     AddressGroup::new(1, 3, 14, 4),
    // ));
    // println!("*** Es 2:\nIPv4:\t{}\nIPv6:\t{}\n", ip, ip2);
    
    // let mut plate_to_owner:HashMap<String, String> = HashMap::new();
    // plate_to_owner.insert(String::from("iasbf"), String::from("Marco ORssi"));
    // plate_to_owner.insert(String::from("MASL"), String::from("Marco Verdi"));
    // plate_to_owner.insert(String::from("ONASFASF"), String::from("Marco Gialli"));
    // plate_to_owner.insert(String::from("DAJE"), String::from("Marco Blui"));
    // plate_to_owner.insert(String::from("Masd4"), String::from("Marco Neri"));
    // plate_to_owner.insert(String::from("ON£fz"), String::from("Marco Binachi"));
    // plate_to_owner.insert(String::from("iaegpo"), String::from("Marco Verdetto"));
    // println!("*** Es 3:\nOwner: {:?}", es3::recognise_owner(&plate_to_owner, String::from("MASL")));

    // let mut items_macchinetta:HashMap<Item, usize> = HashMap::new();
    // items_macchinetta.insert(Item::Coffee, 50);
    // items_macchinetta.insert(Item::Coke, 10);
    // items_macchinetta.insert(Item::Soda, 30);
    // items_macchinetta.insert(Item::Wine, 100);
    // let mut macchinetta = VendingMachine::new(items_macchinetta);

    // macchinetta.insert_coin(&Coin::FiftyPennies).unwrap();
    // macchinetta.insert_coin(&Coin::FiftyPennies).unwrap();
    // macchinetta.insert_coin(&Coin::TenPennies).unwrap();
    // macchinetta.insert_coin(&Coin::TwoPennies).unwrap();

    // let bought_item = Item::Wine;
    // println!("*** Es 4:\nPrice of {}: {}\nTotal Coins: {}", bought_item, macchinetta.get_item_price(&bought_item), macchinetta.coins);
    // match macchinetta.buy(&bought_item){
    //     Ok(change) => println!("Coins after buying a Coffee: {}", change),
    //     Err(e) => println!("{}", e)
    // };

    // let shipment:BoxShipping = BoxShipping { name: String::from("Box full of surpises"),
    //                                          barcode: String::from("AHDJ LJNT IJNS"),
    //                                          shipment_date: Date(04, 10, 2023),
    //                                          shipment_hour: Hour( 13, 59) };
    // println!("*** Es 5:\nShipment with Debug:\n{:?}\nShipment with Display:\n{}", shipment, shipment);

    // let mut bup_library = Library::new();
    // bup_library.add_book(String::from("Come la malattia"), String::from("ADK JPN KLM"), 2021, String::from("Le Coliche"), String::from("Rizzoli Lizard"));
    // bup_library.add_book(String::from("Belle cifre da guadagnare al mese"), String::from("ADK FDK KLM"), 2022, String::from("Nino Frassica"), String::from("Rizzoli Lizard"));
    // bup_library.add_article(String::from("La matassa è davvero inscrogliabile?"), String::from("MTS DVR INS"), 2023, String::from("boh"));
    // bup_library.add_article(String::from("La matassa è davvero inscrogliabile? Ma ne siamo sicuri sicuri?"), String::from("JDK DVR INS"), 2032, String::from("boh"));
    // bup_library.add_magazine(String::from("Giorgetto è ancora vivo?"), String::from("GRG NCR VV"), 2020, 120 , 10);
    // println!("***Es 6:\nLibrary:\n{}", bup_library);

    println!("***Es 7:\ntest: {}\n", test());
}

