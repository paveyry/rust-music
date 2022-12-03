pub mod rhythm {
    pub static SEMIQUAVER: f64 = 0.25;
    pub static QUAVER: f64 = 0.5;
    pub static CROTCHET: f64 = 1.;
    pub static MINIM: f64 = 2.;
    pub static SEMIBREVE: f64 = 4.;
    // TODO: Add more shortcuts, especially triplet variants
}

pub mod dynamic {
    pub static SILENT: u8 = 0;
    pub static PPP: u8 = 10;
    pub static PP: u8 = 25;
    pub static P: u8 = 50;
    pub static MP: u8 = 60;
    pub static MF: u8 = 70;
    pub static F: u8 = 85;
    pub static FF: u8 = 100;
    pub static FFF: u8 = 120;
}