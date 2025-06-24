mod two;
mod boradv;
use crate::two::*;


static  BYTES:[u8; 3] = [1, 2, 3];
static mut BYTES_MUT: [u8; 3] = [1, 2, 3];

fn main() {

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


}
