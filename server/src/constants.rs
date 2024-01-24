use crate::types::{Topic, Unit};

pub const ASCII_ART: &str = r"
  ___         _           ___   _      _      ___                
 | _ \_____ _(_)___ ___  | _ ) /_\  __| |_ __/ __|___ _ __  _ __ 
 |   / -_) V / (_-</ -_) | _ \/ _ \/ _` \ V / (__/ _ \ '  \| '_ \
 |_|_\___|\_/|_/__/\___| |___/_/ \_\__,_|\_/ \___\___/_|_|_| .__/
                                                           |_|   
";

pub const PORT: u32 = 8000;

pub const TOPICS_COUNT: usize = 1;

pub static UNITS: [Unit; TOPICS_COUNT] = [Unit {
    unit: "ELEC1601",
    topics: [Topic {
        id: 1,
        topic: "Base encoding",
    }],
}];
