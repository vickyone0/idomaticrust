
#[derive(Debug,Clone)]
pub struct Pizza {
    toppings: Vec<String>
}

impl Pizza {
    pub fn new(value:Vec<String>) -> Self {
        Self { toppings: value }
    }

    pub fn toppings(&self) -> &[String] {
        self.toppings.as_ref()
    }

    pub fn toppings_mut(&mut self) -> &mut Vec<String> {
        &mut self.toppings
    }

    pub fn set_toppings(&mut self, toppings:Vec<String>) {
        self.toppings = toppings;
    }
}