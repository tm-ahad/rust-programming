

struct Cache
{
    calculation: fn(u32) -> u32,
    value: Option<u32>
}

impl Cache
    {
    pub fn new(closure: fn(u32) -> u32) -> Self
    {
        Self
        {
            calculation: closure,
            value: None
        }
    }


    pub fn calculate(&mut self, arg: u32)
    {
        (self.calculation)(arg);
        self.value = Some((self.calculation)(arg));
    }

    pub fn calculate_to(&mut self, arg: u32, closure: fn(u32) -> u32 ) -> Option<u32>
    {
        self.calculate(arg);
        return match self.value

        {
            Some(v) => Some(closure(v)),
            None => None
        }
    }

}
fn main() {
    const EXPENSIVE: fn(u32) -> u32 = |v: u32| {
        let mut i: u32 = 0;
        while i != (600000000 + v) { i += 1 }
        i
    };

    let mut cache1: Cache = Cache::new(EXPENSIVE);
    cache1.calculate(0);

    let val = cache1.value;

    print!("{:?}/n", val);
    println!("{}", "rendering...");
    print!("{:?}", val);
}