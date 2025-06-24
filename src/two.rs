use std::marker::PhantomData;
pub enum Option<T> {
    Some(T),
    None,
}

struct Dog<Breed> {
    name: String,
    breed: PhantomData<Breed>,
}

struct Labrador {}
struct Retriever {}
struct Poodle {}
struct Dachshund {}

pub fn out(){
 let my_poodle: Dog<Poodle> = Dog {
    name: "jeffrey".into(),
    breed: PhantomData,
};

println!("My dog is a {}, named {}",
            my_poodle.breed_name(),
            my_poodle.name,
        );
}


impl Dog<Labrador> {
    fn breed_name(&self) -> &str {
        "labrador"
    }
}

impl Dog<Retriever> {
    fn breed_name(&self) -> &str {
        "retriever"
    }
}

impl Dog<Poodle> {
    fn breed_name(&self) -> &str {
        "poodle"
    }
}

impl Dog<Dachshund> {
    fn breed_name(&self) -> &str{
        "dachshund"
    }
}

trait DoesItBark {
    fn it_barks(&self) -> bool;
}

struct Cat;
impl DoesItBark for Cat {
    fn it_barks(&self) -> bool {
        true
    }
}

pub struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Self{
        Self { width, height }
    }
} 
    
pub struct Square {
    side: i32,
}

impl Square {
    pub fn new(side: i32) -> Self {
        Self { side }
    }

    pub fn get_side(&self) -> i32 {
        self.side
    }
}


pub trait Rectanglur {

     fn get_width(&self) -> i32;
     fn get_length(&self) -> i32;
     fn get_area(&self) -> i32;
}

impl Rectanglur for Rectangle {
     fn get_area(&self) -> i32 {
        self.height * self.width
    }

    fn get_length(&self) -> i32 {
        self.height
    }

    fn get_width(&self) -> i32 {
        self.width
    }
}


impl Rectanglur for Square {
    fn get_area(&self) -> i32 {
        self.side *self.side
    }
    fn get_length(&self) -> i32 {
        self.side
    }
    fn get_width(&self) -> i32 {
        self.side
    }
}

trait SelfDescribing {
    fn describe() -> String;
}

pub fn describe_type<T: SelfDescribing>() ->string {
    T::describe()
}

impl SelfDescribing for Dog {
    fn describe() -> String {
        "happy little dog".into()
    }
}

impl SelfDescribing for Cat {
    fn describe() -> String {
        "happy little cat".into()
    }
}

