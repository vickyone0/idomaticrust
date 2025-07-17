#[derive(Debug)]
struct Bicycle {
    make: String,
    model: String,
    size: i32,
    color:String,
}

impl Bicycle {
    fn make(&self) -> &String {
        &self.make

    }

    fn model(&self) -> &String {
        &self.model
    }

    fn size(&self) -> i32 {
        self.size
    }

    fn color(&self) -> &String {
        &self.color
    }
}


struct BicycleBuilder {
    bicycle: Bicycle,
}
trait Builder<T> {
    fn new() -> Self;
    fn build(self) -> T;
}
impl Builder<Bicycle> for BicycleBuilder {
    fn new() -> Self {
        Self {
            bicycle: Bicycle {
                make: String::new(),
                model: String::new(),
                size: 0,
                color: String::new(),
            },
        }
    }
    fn build(self) -> Bicycle {
        self.bicycle
    }
}
trait Buildable<Target, B: Builder<Target>> {
    fn builder() -> B;
}
impl Buildable<Bicycle, BicycleBuilder> for Bicycle {
    fn builder() -> BicycleBuilder {
        BicycleBuilder::new()
    }
}

impl BicycleBuilder {
    

    fn with_make(&mut self, make:&str){
        self.bicycle.make = make.into()
    }
    fn with_model(&mut self, model:&str){
        self.bicycle.model = model.into()

    }

    fn with_size(&mut self, size: i32) {
        self.bicycle.size = size
    }
    fn with_color(&mut self, color: &str) {
        self.bicycle.color = color.into()
    }
    fn build(self) -> Bicycle {       
        self.bicycle
    }


}
pub fn outs(){



// let mut bicycle_builder = BicycleBuilder::new();
// bicycle_builder.with_make("Huffy");
// bicycle_builder.with_model("Radio");
// bicycle_builder.with_size(46);
// bicycle_builder.with_color("red");
// let bicycle = bicycle_builder.build();
// println!("My new bike: {:#?}", bicycle);

let mut bicycle_builder = Bicycle::builder();
bicycle_builder.with_make("Huffy");
bicycle_builder.with_model("Radio");
bicycle_builder.with_size(46);
bicycle_builder.with_color("red");
let bicycle = bicycle_builder.build();
println!("My new bike: {:?}", bicycle);

}