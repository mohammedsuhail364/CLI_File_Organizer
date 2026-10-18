use std::fs;
pub fn read_somefile() {
    let file_to_read = "./data/file1.txt";
    let file_to_read1 = "./data/file2.txt";
    let file_to_read2 = "./data/file3.txt";
    let result = fs::read(file_to_read);
    let result1 = fs::read(file_to_read1);
    let result2 = fs::read(file_to_read2);
    println!();
    match result {
        Ok(vec) => {
            for x in vec {
                
                print!("{}", x as char);
            }
        }
        Err(err) => println!("Error Occured {} =>", err),
    }
    println!();
    match result1 {
        Ok(vec) => {
            for x in vec {
                
                print!("{}", x as char);
            }
        }
        Err(err) => println!("Error Occured {} =>", err),
    }
    println!();
    match result2 {
        Ok(vec) => {
            for x in vec {
                
                print!("{}", x as char);
            }
        }
        Err(err) => println!("Error Occured {} =>", err),
    }
}
pub fn create_files() {
    let path = "./data/file1.txt";
    let path1 = "./data/file2.txt";
    let path2 = "./data/file3.txt";
    let text = "Mohammed Suhail";
    let text1 = "Mohammed";
    let text2 = "Mohammed Suhail S";
    let _ = fs::write(path, text);
    let _ = fs::write(path1, text1);
    let _ = fs::write(path2, text2);
}
pub fn test_create_dir() {
    let path = "./data";
    let my_path = std::path::Path::new(path);
    if my_path.exists() {
        print!("Directory already exists! skipping creation...");
        return;
    }
    let create_dir = fs::create_dir(path);
    match create_dir {
        Ok(val) => print!("Created new data directory! {:?}", val),
        Err(err) => print!("Some Problem occured {:?} ", err),
    }
}
