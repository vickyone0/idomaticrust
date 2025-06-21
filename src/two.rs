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