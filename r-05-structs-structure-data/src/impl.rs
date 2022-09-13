

#[derive(Debug)]
pub struct Square {
    pub height: i128,
    pub name: String,
    pub color: Color
}

impl Square {

    pub fn create(height: i128, name: String, color: Color) -> Square {
        Square {
            height,
            name,
            color
        }
    }

    pub fn get_area(&self) -> i128 {

        return self.height * self.height;
    }

}