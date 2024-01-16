use std::fmt::Debug;
use std::{str::FromStr, io::{Stdout, Stdin, Write}};

pub fn input<'a, T: FromStr>(mut stdout: &Stdout, stdin: &Stdin, prompt: &'a str) -> T
where 
    <T as FromStr>::Err: Debug,
{
    let mut buf = String::new();
    print!("{}", prompt);
    let _ = stdout.flush().unwrap();
    let _ = stdin.read_line(&mut buf).unwrap();
    buf.trim().parse::<T>().unwrap()
}