pub struct Polygon {
    pub name: String,
    sides: u32,
    pub visible: bool,
}

impl Polygon {
    pub fn new(name: String) -> Self {
        Self{
            name: name,
            sides: 3,
            visible: true
        }
    }
    pub fn sides(&self) -> u32 {
        self.sides
    }
    pub fn shape(&self) -> String {
        let tmp = if self.sides == 3 {
            "triangle"    
        } else if self.sides == 4{
            "square"
        } else if self.sides == 5 {
            "pentagon"
        } else {
            "polygon"
        };
        tmp.to_string()
    }
    pub fn increment_sides(&mut self){
        self.sides+=1
    }
}