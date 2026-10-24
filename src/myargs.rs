use std::env::args;

pub fn args_parser() {
    let myargs: Vec<String> = args().collect();
    println!("{:?}", myargs);
    if myargs.len() != 3 {
        println!("Hey , you didn't specify two arguments.");
    }
    let name:String = myargs.get(1).unwrap().into();
    let parsed_year = myargs.get(2).unwrap().parse::<i32>();
    if !parsed_year.is_ok(){
        println!("The specified dog year was invalid.Please specify an integer 32 value.");
        return;
    }
    let year_born = parsed_year.ok().unwrap();
    println!("The Dog name is {name} and the year born is {year_born}");
    let dog1 = new_dog(name, year_born);
    dog1.get_details();
    
}
fn new_dog(name: String, year_born: i32) -> Dog {
    return Dog { name, year_born };
}
struct Dog {
    name: String,
    year_born: i32,
}
impl Dog {
    fn get_details(&self){
        println!("The Dog name is {} and the year born is {}",self.name,self.year_born);
    }
}