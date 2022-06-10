use std::collections::HashMap;

fn main() {
    println!("----------------------Hello Worlds----------------------");
    // unsigned integer
    // u8, u16, u32, u54, u128
    let unsigned: u8 = 10;

    // signed integer
    // i8, i16, i32, i64, i128
    let signed: i8 = -10;

    // float is used for decimals
    let float: f32 = 1.2;

    println!("unsign: {} sign: {} float: {}", unsigned, signed, float);

    // char - can only be
    let letter = "c1232";
    let emoji = "\u{1F600}"; // :D

    println!("letter: {}, emoji: {}", letter, emoji);

    let is_true: bool = true;

    println!("isTrue: {}", is_true);


    println!("----------------------array----------------------");


    let arr: [u8; 3] = [1, 2, 3];
    let other_arr: [u8; 5] = [100; 5];

    println!("index: {}, length: {}", arr[0], other_arr.len());

    println!("{:?}", other_arr);


    println!("----------------------tuple----------------------");

    let tuple: (u8, bool, f32) = (5, true, 2.1);
    let tuple2 = (3,5);

    // print structure of array and other objects
    println!("first {}, second {}, third {}", tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple2);

    let (a, b, c) = tuple;

    // descructing
    println!("first {}, second {}, third {}", a, b, c);


    println!("----------------------function----------------------");

    println!("is even ? {}", is_even(1));

    println!("----------------------mutability----------------------");

    let mut num1 = 5;
    num1 = 3;

    println!("{}", num1);

    println!("----------------------array + slices----------------------");

    let arr2 = [0, 1, 2, 3]; //length
    let slice = &arr2[1 .. 3]; //[1, 2] don't know the length
    borrowing_slice(arr2, slice);

    println!("----------------------Strings----------------------");

    let str: &str = "hello worlds";
    let mut string: String = String::from("Hello Worlds");

    let slice2 = &string[.. 6];
    slice2.len();

    string.push('1');
    string.push_str("! Bob");
    string.replace("Hello", "Bye");
    println!("{}", string);

    println!("----------------------if statement----------------------");

    let n = 3;
    if n > 0{
        println!("greater than 0");
    } else if n < 0{
        println!("less than 0");
    } else {
        println!("is 0");
    }

    println!("----------------------for loop----------------------");

    for i in 0..6{
        println!("{}", i);
    }

    println!("----------------------while loop----------------------");

    let mut i = 0;
    while i < 4{
        println!("{}", i);
        i += 1;

        if i == 3{
            println!("exit");
            break;
        }
    }

    println!("----------------------match statement----------------------");

    i = 5;
    match i {
        0 => println!("0"),
        1 | 2 => println!("1,2"),
        3..=4 => println!("3,4"),
        _ => println!("default")
    }

    println!("----------------------struct----------------------");

    let name = String::from("BirdMissile");
    let bird = Bird{ name: name, attack: 5};
    bird.print_name();


    println!("----------------------Traits----------------------");
    println!("{} {}", bird.can_fly(), bird.is_animal());


    println!("----------------------enum----------------------");
    let enuma: MyEnum = MyEnum::A;
    let enumb: MyEnum = MyEnum::B(5);
    let enumc: MyEnum = MyEnum::C{x:10, y:20};
    println!("{:?}", enuma);
    println!("{:?}", enumb);
    println!("{:?}", enumc);

    if let MyEnum::B(val) = enumb {
        println!("{}", val);
    }
    if let MyEnum::C{x, y} = enumc {
        println!("{} {}", x, y);
    }


    println!("----------------------vector----------------------");

    let mut vec: Vec<i64> = vec![1,2,3,4,5];
    vec.len();
    vec[0];
    vec.push(6);
    vec.remove(0);
    println!("{:?}", vec);

    println!("----------------------Hash Map----------------------");

    let mut map = HashMap::new();

    map.insert(0,"Hi");
    map.insert(1,"Hi2");

    println!("{:?}", map);

    match map.get(&0){
        Some(str1) => println!("{}", str1),
        None => println!("Doesn't exist in map"),
    }
    match map.get(&2){
        Some(str) => println!("{}", str),
        _ => println!("Doesn't exist in map"),
    }

    map.remove(&0);
    println!("{:?}", map);



    println!("----------------------options----------------------");

    let divide1: Option<i32> = divide(4, 2);
    // let divide2: Option<i32> = divide(2, 3);

    println!("{:?} unwraps to{}", divide1, divide1.unwrap());

    //will be panic
    // println!("{:?} unwraps to{}", divide2, divide2.unwrap());


    println!("----------------------results----------------------");

    let divide = divides(4, 2);
    // let res = divide2.expect("we crashed");

    match divide {
        Ok(v) => println!("{}", v),
        Err(v) => println!("{:?}", v)
    }

    // if divide.is_ok(){
    //     println!("{}", divide.unwrap());
    // }
    // println!("{}", divide.unwrap());
    // println!("{}", divide.unwrap_or(100));
    // println!("{}", res);

    println!("----------------------conculusion----------------------");

}

#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
    C{x: i32, y: i32}
}

struct Bird {
    name: String,
    attack: u64
}
impl Bird {
    fn print_name(&self){
        println!("{}", self.name);
    }
}

trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}

impl Animal for Bird {
    fn can_fly(&self) -> bool{
        true
    }
    fn is_animal(&self) -> bool {
        false
    }
}

pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 // return bool
}

fn borrowing_slice(arr: [u8; 4], slice: &[u8]){
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}

fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend/divisor)
    }
}

#[derive(Debug)]
enum MyError
{
    Error1
}

fn divides(dividend: i32, divisor: i32) -> Result<i32, MyError> {
    if dividend % divisor != 0 {
        Err(MyError::Error1)
    } else {
        Ok(dividend/divisor)
    }
}