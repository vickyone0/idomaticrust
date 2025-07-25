use std::thread;
use std::time::Duration;
use reqwest;

pub async   fn std_thread(){

    thread::spawn(async ||{
        for i in 1..10{
            println!("hi from {} in spawn thread",i);
            let client = reqwest::Client::new();

            let respose = client.get("http://127.0.0.1:8080/hello")
                            .send()
                            .await;
            thread::sleep(Duration::from_millis(10));
        }

    });

    for i in  1..10{
        println!("hi from {} in main thread",i);
        thread::sleep(Duration::from_millis(20));
    }



}


pub async fn multi_std (){

    let handle = thread::spawn(|| {
        for i in 1..5{
            println!("this is from spawn thread {}",i);
            thread::sleep(Duration::from_millis(1000));
        }
    });

    for i in 1..4 {
        println!("Main: Count {}", i);
        thread::sleep(Duration::from_millis(1000));
    }

    handle.join().unwrap();
}  