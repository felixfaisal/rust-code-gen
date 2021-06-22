pub enum Felix {
    ICMP = 1,
    IGP = 3,
}
impl Felix {
    pub fn String(&self) {
        match self {
            Felix::ICMP => println!("ICMP"),
            Felix::IGP => println!("IGP"),
            _ => println!("Protocol does not exist"),
        }
    }
    pub fn value(&self) {
        match self {
            Felix::ICMP => println!("{}", Felix::ICMP as u32),
            Felix::IGP => println!("{}", Felix::IGP as u32),
            _ => println!("Protocol does not exist"),
        }
    }
    pub fn new(u: u32) -> Felix {
        match u {
            1 => Felix::ICMP,
            3 => Felix::IGP,
            _ => Felix::IGP,
        }
    }
}
