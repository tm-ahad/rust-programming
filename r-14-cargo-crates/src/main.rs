use crate::yaml_parser::{YamlValues, YamlObj};

mod yaml_parser
{

    #[derive(Clone)]
    #[derive(Debug)]
    pub struct YamlObj
    {
        pairs: Vec<(Option<String>, Option<YamlValues>)>
    }

    impl YamlObj
    {
        pub fn new(pair: Vec<(Option<String>, Option<YamlValues>)>) -> Self
        {
            Self
            {
                pairs: pair
            }
        }

        // pub fn get(&self, k: String)
        // {
        //
        //     return match self.clone().pairs
        //     {
        //         Some(v) =>
        //             {
        //                 for pairs in v.iter()
        //                 {
        //                     Some(YamlValues::String("".to_string()));
        //                 }
        //                 Some(YamlValues::String("".to_string()));
        //             },
        //         None => None
        //     }
        // }

        pub fn insert(&mut self, k: String, val: YamlValues)
        {
            self.pairs.push((Some(k), Some(val)));
        }
        //
        // pub fn del(&mut self, k: String) -> bool
        // {
        //     return match self.clone().pairs
        //     {
        //         Some(mut v) =>
        //             {
        //                 v.retain(|&x| x.0 != k);
        //                 true
        //             },
        //         None => false
        //     }
        // }
        //
        // pub fn replace(&self, k: String, new_val: YamlValues)
        //
        // {
        //     let refer = &self.get(k);
        //
        //     println!(refer);
        // }
    }

    #[derive(Clone)]
    #[derive(Debug)]
    pub enum YamlValues
    {
        Int8(i8),
        Int16(i16),
        Int32(i32),
        Int64(i64),
        Int128(i128),
        Float32(f32),
        Float64(f64),
        String(String),
        Array(Box<[YamlValues]>),
        YamlObject(YamlObj)
    }
}

fn main() {
    let mut obj = YamlObj::new(vec![]);

    obj.insert("c".to_string(), YamlValues::Int8(5));

    println!("{:?}", obj);
}