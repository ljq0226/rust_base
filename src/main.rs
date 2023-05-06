#![allow(unused)]




fn greet_world(){
    let chinese = "你好，世界";

    let english = "hello world!";
    let regions = [chinese,english];
    for region in regions.iter() {
        println!("{}",region)
    }
}
fn test(){
    let x: i32 = 1;
    let _y:i32;
    println!("x's value is {}",x)
}


fn main() {
    println!("Hello, world!");
    greet_world();
    test()
}
