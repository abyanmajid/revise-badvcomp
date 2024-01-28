use crate::types::{QuestionType, Topic, Unit};
use once_cell::sync::Lazy;

pub const ASCII_ART: &str = r"
  ___         _           ___   _      _      ___                
 | _ \_____ _(_)___ ___  | _ ) /_\  __| |_ __/ __|___ _ __  _ __ 
 |   / -_) V / (_-</ -_) | _ \/ _ \/ _` \ V / (__/ _ \ '  \| '_ \
 |_|_\___|\_/|_/__/\___| |___/_/ \_\__,_|\_/ \___\___/_|_|_| .__/
                                                           |_|   
";

pub const TOPICS_COUNT: usize = 2;

pub static UNITS: Lazy<[Unit; TOPICS_COUNT]> = Lazy::new(|| {
    [
        Unit {
            unit: String::from("ELEC1601"),
            syllabus: String::from("S2, 2023"),
            topics: vec![
                Topic {
                    id: 1,
                    topic: String::from("Base encoding"),
                    question_types: vec![],
                },
                Topic {
                    id: 2,
                    topic: String::from("Fixed-point numbers"),
                    question_types: vec![
                        QuestionType {
                            id: 1,
                            qtype: String::from("Convert fixed-point to decimal"),
                        },
                        QuestionType {
                            id: 2,
                            qtype: String::from("Convert decimal to fixed-point"),
                        },
                    ],
                },
                Topic {
                    id: 3,
                    topic: String::from("Memory size"),
                    question_types: vec![
                        QuestionType {
                            id: 1,
                            qtype: String::from("Calculate memory size"),
                        },
                        QuestionType {
                            id: 2,
                            qtype: String::from("Calculate cell size"),
                        },
                        QuestionType {
                            id: 3,
                            qtype: String::from("Calculate number of cells"),
                        },
                        QuestionType {
                            id: 4,
                            qtype: String::from("Calculate minimal address bits"),
                        },
                    ],
                },
                Topic {
                    id: 4,
                    topic: String::from("Registers"),
                    question_types: vec![QuestionType {
                        id: 1,
                        qtype: String::from("Trace registers value based on clock"),
                    }],
                },
                Topic {
                    id: 5,
                    topic: String::from("Stack"),
                    question_types: vec![QuestionType {
                        id: 1,
                        qtype: String::from("Keeping track of push and pop operations"),
                    }],
                },
                Topic {
                    id: 6,
                    topic: String::from("AVR Assembly"),
                    question_types: vec![QuestionType {
                        id: 1,
                        qtype: String::from("Bitwise AND + Reading Opcode"),
                    }],
                },
            ],
        },
        Unit {
            unit: String::from("MATH1061"),
            syllabus: String::from("S1, 2024"),
            topics: vec![Topic {
                id: 1,
                topic: String::from("To be added!"),
                question_types: vec![],
            }],
        },
    ]
});
