pub fn test_closures(){
    let add = |x,y| {
        println!("practicing {}",x+y);
        x+y
    };
    let res = add(5,5);
    println!("{}",res);


}