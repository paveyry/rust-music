pub mod rhythm {
    // Binary rhythms
    pub static DEMI_SEMIQUAVER: f64 = 0.125;
    pub static DOTTED_DEMI_SEMIQUAVER: f64 = 0.1825;
    pub static SEMIQUAVER: f64 = 0.25;
    pub static DOTTED_SEMIQUAVER: f64 = 0.375;
    pub static QUAVER: f64 = 0.5;
    pub static DOTTED_QUAVER: f64 = 0.75;
    pub static CROTCHET: f64 = 1.;
    pub static DOTTED_CROTCHET: f64 = 1.5;
    pub static MINIM: f64 = 2.;
    pub static DOTTED_MINIM: f64 = 3.;
    pub static SEMIBREVE: f64 = 4.;
    pub static BREVE: f64 = 8.;

    // Ternary rhythms
    pub static TER_SEMIQUAVER: f64 = 1. / 6.; // 0.166
    pub static TER_DOTTED_SEMIQUAVER: f64 = 0.25;
    pub static TER_QUAVER: f64 = 1. / 3.; // 0.33
    pub static TER_DOTTED_QUAVER: f64 = 0.5;
    pub static TER_CROTCHET: f64 = 2. / 3.; // 0.66
    pub static TER_DOTTED_CROTCHET: f64 = 1.;
    pub static TER_MINIM: f64 = 4. / 3.; // 1.33
    pub static TER_DOTTED_MINIM: f64 = 2.;
    pub static TER_SEMIBREVE: f64 = 8. / 3.; // 2.66
    pub static TER_DOTTED_SEMIBREVE: f64 = 4.;
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
