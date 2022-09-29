use std::{env, thread};
use std::fs::File;
use std::io::{Read, Write};
use std::process;
use std::time::Duration;

pub fn read_file(path: String) -> String
{
    let mut file = File::open(path).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    let mut content = String::new();

    file.read_to_string(&mut content).expect("Can't read file");
    content
}

fn main()
{

    let args: Vec<String> = env::args().collect();
    let (query, file_name) = Config::new(args.clone()).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    }).extract();


    if query == "r"
    {
        println!("Reading file...");
        thread::sleep(Duration::from_millis(4000));
        println!("{}", read_file(file_name.clone()));
    }

    if query == "c"
    {
        let mut file = File::create(file_name.clone()).expect("TODO: panic message");
        file.write_all(b"Hello World!").unwrap();
    }
}

struct Config
{
    pub file_name: String,
    pub query: String
}

impl Config
{
    pub fn new(args: Vec<String>) -> Result<Self, String> {

        if args.len() < 3
        {
            return Err("No enough data".to_string());
        }

       Ok(Self {
            query: args[2].clone(),
            file_name: args[1].clone()
        })
    }

    pub fn extract(&self) -> (String, String)
    {
        return (self.file_name.clone(), self.query.clone());
    }
}