mod two;
mod boradv;
mod three;
mod interbiew;

use crate::two::*;
use crate::three::*;
use crate::interbiew::*;




static  BYTES:[u8; 3] = [1, 2, 3];
static mut BYTES_MUT: [u8; 3] = [1, 2, 3];

fn main() {

    clos();

    let black_cat = Cat::new(String::from("harry"), CatColor::Black);

    match_on_black_cats(&black_cat);

    let big_pumkin = Pumkin{
        mass: 10,
        diameter: 20,
    };

    println!("cloned pumkin is {:?}", big_pumkin.clone());
    println!("pumkin is {:?}", big_pumkin);
    println!("pumkin is {:?}", Pumkin::default());

    //println!("Iam a {}",describe_type::<Dog>());
    two::out();

    //MUT_BYTES_MUT[0] = 4;

    unsafe {
        BYTES_MUT[0] = 4;
        assert_eq!(BYTES_MUT[0], 4);
    }


    let rect = Rectangle::new(10, 20);

    println!("Rectangle of lenth {} and width {} has area {}",
                rect.get_length(),
                rect.get_width(),
                rect.get_area(),
            );

    let squ = Square::new(10);        
    
    println!("Square of lenth {}  has area {}",
                rect.get_length(),
                rect.get_area(),
            );

    
   read_file("input.txt");
}
