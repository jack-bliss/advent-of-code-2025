use core::f64;
use std::{collections::HashMap, fmt::Display};

use enum_common_fields::EnumCommonFields;

use crate::day_6::solve_equations;

enum Sign {
    Positive,
    Negative,
    None,
}

enum Value {
    Int(i64),
    Str(String),
    Float(f64),
    NaN,
    Inf(Sign),
    Exponent(f64, i32),
    Hex(u64),
    Oct(u64),
    Bin(u64),
    Bool(bool),
    Null,
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Str(s) => write!(f, "\"{}\"", s),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::NaN => write!(f, "NaN"),
            Value::Inf(sign) => match sign {
                Sign::Positive => write!(f, "Infinity"),
                Sign::Negative => write!(f, "-Infinity"),
                Sign::None => write!(f, "Infinity"),
            },
            Value::Exponent(base, exp) => write!(f, "{}e{}", base, exp),
            Value::Hex(h) => write!(f, "0x{:X}", h),
            Value::Oct(o) => write!(f, "0o{:o}", o),
            Value::Bin(b) => write!(f, "0b{:b}", b),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::List(lst) => {
                let elements: Vec<String> = lst.iter().map(|v| format!("{}", v)).collect();
                write!(f, "[{}]", elements.join(", "))
            }
            Value::Map(map) => {
                let elements: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v))
                    .collect();
                write!(f, "{{{}}}", elements.join(", "))
            }
        }
    }
}

#[allow(dead_code)]
mod day_1;
#[allow(dead_code)]
mod day_2;
#[allow(dead_code)]
mod day_3;
#[allow(dead_code)]
mod day_4;
#[allow(dead_code)]
mod day_5;
// #[allow(dead_code)]
mod day_6;

enum Events {
    BillOfLading,
    DerivedFromCargo,
}

#[derive(Debug)]
struct DisplayData {
    label: &'static str,
    color: &'static str,
}

#[derive(EnumCommonFields)]
#[common_field(own label: String)]
enum Thing {
    A { label: String },
    B { label: String },
    C { label: String },
}

fn get_label(thing: Thing) -> String {
    thing.into_label()
}

fn get_display_data(event: Events) -> DisplayData {
    match event {
        Events::BillOfLading => DisplayData {
            label: "Bill of Lading",
            color: "green",
        },
        Events::DerivedFromCargo => DisplayData {
            label: "Derived from Cargo",
            color: "red",
        },
    }
}

fn main() {
    let number = Value::Int(42);
    let text = Value::Str("Hello, World!".to_string());
    let null = Value::Null;
    let list = Value::List(vec![number, text, null]);
    let map = Value::Map(HashMap::from([
        ("key1".to_string(), Value::Float(f64::consts::PI)),
        ("key2".to_string(), list),
    ]));

    println!("{}", map);

    // println!("{:?}", solve_equations());

    // let thing = Thing::A {
    //     label: "Test A".to_string(),
    // };

    // println!("{}", get_label(thing));

    // println!("{:?}", get_display_data(Events::BillOfLading));
    // println!("{:?}", get_display_data(Events::DerivedFromCargo));
}
