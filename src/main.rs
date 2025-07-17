mod two;
mod boradv;
mod three;
mod interbiew;
mod multithread;
mod backend;
mod four;
mod pattern;
use actix_web::Error;

use crate::two::*;
use crate::three::*;
use crate::interbiew::*;
use crate::multithread::*;
use crate::backend::*;
use crate::four::*;

use crate::pattern::*;


use actix_web::{get, web, App, HttpServer, Responder};

static  BYTES:[u8; 3] = [1, 2, 3];
static mut BYTES_MUT: [u8; 3] = [1, 2, 3];
#[actix_web::main]
async fn main()  {

//     clos();

//     let black_cat = Cat::new(String::from("harry"), CatColor::Black);

//     match_on_black_cats(&black_cat);

//     let big_pumkin = Pumkin{
//         mass: 10,
//         diameter: 20,
//     };

//     println!("cloned pumkin is {:?}", big_pumkin.clone());
//     println!("pumkin is {:?}", big_pumkin);
//     println!("pumkin is {:?}", Pumkin::default());

//     //println!("Iam a {}",describe_type::<Dog>());
//     two::out();

//     //MUT_BYTES_MUT[0] = 4;

//     unsafe {
//         BYTES_MUT[0] = 4;
//         assert_eq!(BYTES_MUT[0], 4);
//     }


//     let rect = Rectangle::new(10, 20);

//     println!("Rectangle of lenth {} and width {} has area {}",
//                 rect.get_length(),
//                 rect.get_width(),
//                 rect.get_area(),
//             );

//     let squ = Square::new(10);        
    
//     println!("Square of lenth {}  has area {}",
//                 rect.get_length(),
//                 rect.get_area(),
//             );

    
//    read_file("input.txt");

// std_thread();
//   HttpServer::new(|| {
//         App::new()
//             .service(hello)  // Register the hello endpoint  // Register the info endpoint
//     })
//     .bind("127.0.0.1:8080")?
//     .workers(1)
//     .run()
//     .await

// let mut pizza_hut = Pizza::new(
//      vec![String::from("honey"),]
// );
   
//    println!("{:?}",pizza_hut);

//  use std::path::Path;
//    let path = Path::new("Cargo.toml");

//    println!("the 3rd line in cargo.toml is {:?}", 
//                 read_nth_line(path, 0));


// multi_std().await;
    
// macro_rules! print_what_it_is {
//     () => {
//         println! (" A macro with no arguments")
//     };
//     ($e:expr) => {
//         println!("A macro with an expression")
//     };
//     ($s:stmt) => {
//         println! (" a macro wirh a statement")
//     };
// }

// print_what_it_is!();
// print_what_it_is!({});
// print_what_it_is!(;);
                

//                 macro_rules! special_println {
//     ($($arg:tt)*) => {           
//         println!($($arg)*)       
//     };
// }

// special_println!("jkjfn");

outs();
}

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello from actix_web!"
}
