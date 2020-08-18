use std::io;
use std::io::Write;

/*
 * Use alt + -> / <- go to previous/next cursor position
*/

/*
Decimal	98_222
Hex	0xff
Octal	0o77
Binary	0b1111_0000
Byte (u8 only)	b'A'
*/

fn numeric_literals()
{
    println!("\n\n\nNumeric literals:");
    let decimal = 22_000;
    let hex = 0xFF;
    let octal = 0o77;
    let binary = 0b11;
    let byte = b'A';

    println!("decimal = {}", decimal);
    println!("hex = {}", hex);
    println!("octal = {}", octal);
    println!("binary = {}", binary);
    println!("byte = {}", byte);

    let mut float = 3.0;
    println!("float = {}", float);
    float = 3.2;
    println!("float = {}", float);

    let some_char ='A';
    let unicode_char = '❦';
    println!("some_char = {}", some_char);
    println!("unicode_char = {}", unicode_char);
}

fn print_array(arr: &mut [i32], msg: &String)
{
    println!("{}", msg);
    for el in arr {
        print!("{} ", el);
    }
}

fn compound_data_types()
{
    println!("\n\n\nCompound data types:");
    let good_tuple: (i32, f32, bool) = (-123_33, 3.43, true);
    println!("good_tuple.i32 = {}", good_tuple.0);
    println!("good_tuple.f32 = {}", good_tuple.1);
    println!("good_tuple.bool = {}", good_tuple.2);

    let (integer, float, boolean) = good_tuple;
    println!("integer = {}", integer);
    println!("float = {}", float);
    println!("boolean = {}", boolean);

    let mut int_arr = [1, 2, 3, 4];
    print_array(&mut int_arr, &String::from("Array:"));
}

fn conditions()
{
    println!("\n\n\nConditions:");
    const const_value:i32 = 122;
    let value = if const_value % 2 == 0 { 1 } else { 10 };
    println!("const_value = {} value = {}", const_value, value);

    let mut value = 0;
    let mut loop_result = loop {
        println!("value = {}", value);
        if value == 2 {
            break value;
        }
        value = value + 1;
    };
    println!("loop_result = {}", loop_result);

    value = 0;
    while value != 2 {
        println!{"value = {}", value};
        value = value + 1;
    };
}

fn takes_string_and_return(s: String) -> (String, usize)
{
    let len = s.len();
    (s, len)
}

fn read_line()
{
    let mut file_path = String::new();
    
    print!("Enter file path: ");
    std::io::stdout().flush(); //This is very weird behavior of std(in/out) need to read more about this
    io::stdin().read_line(&mut file_path).expect("Failed to read line!"); //Expects works with std::Result thing

    println!("Your fila path {}", file_path);
}

use std::fs::File;
use std::io::prelude;
use std::env;

fn render(file_path: &String) -> std::io::Result<()>
{
    let current_dir = env::current_dir().unwrap();
    let path = current_dir.into_os_string();
    let p = path.to_str();
    match p {
        Some(string) => {
            println!("current_dir = {}", string);
            let mut file = File::create(file_path)?;
            file.write_all(b"Hello world")?;
        },
        None => {
            
        }
    }
    Ok(())
}

fn main() {
    compound_data_types();
    numeric_literals();
    conditions();

    let some_name = String::from("SomeWeirdName");
    let result_len = takes_string_and_return(some_name);
    println!("some_name = {}, result_len = {}", result_len.0, result_len.1);

    let file_path = String::from("rendered/output.txt");
    let result = render(&file_path);
    //You must prcess this result
    //readLine();
}
