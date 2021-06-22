#[derive(Debug, Clone, Copy)]
pub struct union {
    pub XXX_unionData: [u8; 16],
}
impl union {
    pub fn new() -> union {
        union {
            XXX_unionData: [0; 16],
        }
    }
    pub fn write(&mut self) {
        for (pos, e) in self.XXX_unionData.iter().enumerate() {
            println!("{} {}", pos, e);
        }
    }
    pub fn update(&mut self) {
        for i in 0..self.XXX_unionData.len() {
            self.XXX_unionData[i] = 70;
        }
    }
    pub fn SetIP4(&mut self) {
        println!("Setting IP4");
        for i in 0..4 {
            match i {
                0 => self.XXX_unionData[i] = 192,
                1 => self.XXX_unionData[i] = 226,
                2 => self.XXX_unionData[i] = 120,
                3 => self.XXX_unionData[i] = 20,
                _ => println!("Reached end of the array"),
            }
        }
    }
    pub fn GetIP4(&self) {
        println!("Fetching IP4");
        for i in 0..4 {
            println!("{}", self.XXX_unionData[i]);
        }
    }
    pub fn SetIP6(&mut self) {
        println!("Setting IP6");
        for i in 0..self.XXX_unionData.len() {
            self.XXX_unionData[i] = 10;
        }
    }
    pub fn GetIP6(&self) {
        println!("Fetching IP6");
        for i in 0..self.XXX_unionData.len() {
            println!("{}", self.XXX_unionData[i]);
        }
    }
}
