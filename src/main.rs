pub mod myfs;
pub mod closures;
pub mod mymutex;
pub mod mympsc;
pub mod myargs;
fn main() {
    // println!("Practicing File Systems ");
    // myfs::test_create_dir();
    // myfs::create_files();
    // myfs::read_somefile();
    // println!("Practicing Closures");
    // closures::test_closures();
    // println!("Practicing mutex");
    // mymutex::test_mutex();
    // println!("Practicing mpsc");
    // mympsc::test_channels();
    println!("Practicing CLI args parser");
    myargs::args_parser();

}
