
use std::fs::File;
use std::io::{ BufReader, BufWriter, Read, Write};

pub fn read_file(name: &str)-> Result<(), Box<dyn std::error::Error>>{

    //featch the file
    let file = File::open(name)?;
    //add to inmemory
    let mut reader: BufReader<File> = BufReader::new(file);

    

    let mut  content = String::new();
    //copy to string
    reader.read_to_string(&mut content);
 
    print!("{}",content);
    //create file
    let file = File::create("output.txt")?;
    //make the file writable
    let mut write = BufWriter::new(file);

    let modefied_content = content.replace("ji", "7");

    write.write_all(modefied_content.as_bytes())?;

   

    Ok(())


}



