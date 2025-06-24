fn some_or_none<T: std::fmt::Display>(option: &Option<T>) {
    match option {
        Some(v) => println!("is some! where v is {}",v),
        None => println!("is none"),
    }
}

fn what_type_ingetger(value: i32) {
    match value {
        1 => println!("its one"),
        2 => println!("its two"),
        3 => println!("its three"),
        _ => println!("its something else"),
    }
}

fn destructure_tuple(tuple: &(i32,i32,i32)){
    match tuple {
        (first,..) => {
            println!("First value is {}",first)
        }
    }

    match tuple {
        (..,last) => {
            println!("First value is {}",last)
        }
    }

    match tuple {
        (_, middle, _) => {
            println!("First value is {}",middle)
        }
    }
    match tuple {
        (first,middle,last) => {
            println!("First value is {} {} {}",first,middle,last)
        }
    }
}

enum DistinctTypes {
    Name(String),
    Count(i32),
}

fn enum_match_type(enum_type:&DistinctTypes) {

    match enum_type {
        DistinctTypes::Name(value) =>  println!(" value is {}", value),
        DistinctTypes::Count(count) => println!("count is {}",count)

    }

}

pub enum CatColor{
    Black,
    Red,
    Chocolate,
    Cinnamon,
    Blue,
    Cream,
    Cheshire,
}

pub struct Cat {
    name: String,
    color: CatColor,
}

impl Cat {
    pub fn new(name:String,color:CatColor) ->Self{
        Self{
            name,
            color,
        }
        
    }
}

pub fn match_on_black_cats(cat: &Cat) {
    match cat {
        Cat {
            name,
            color: CatColor::Black,
        } => println!("This is a black cat named {name}"),
        Cat { name, color: _ } => println!("{name} is not a black cat"),
    }
}
