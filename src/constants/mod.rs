pub mod rhythm {
    // Binary rhythms
    pub const DEMI_SEMIQUAVER: f64 = 0.125;
    pub const DOTTED_DEMI_SEMIQUAVER: f64 = 0.1825;
    pub const SEMIQUAVER: f64 = 0.25;
    pub const DOTTED_SEMIQUAVER: f64 = 0.375;
    pub const QUAVER: f64 = 0.5;
    pub const DOTTED_QUAVER: f64 = 0.75;
    pub const CROTCHET: f64 = 1.;
    pub const DOTTED_CROTCHET: f64 = 1.5;
    pub const MINIM: f64 = 2.;
    pub const DOTTED_MINIM: f64 = 3.;
    pub const SEMIBREVE: f64 = 4.;
    pub const BREVE: f64 = 8.;

    // Ternary rhythms
    pub const TER_SEMIQUAVER: f64 = 1. / 6.; // 0.166
    pub const TER_DOTTED_SEMIQUAVER: f64 = 0.25;
    pub const TER_QUAVER: f64 = 1. / 3.; // 0.33
    pub const TER_DOTTED_QUAVER: f64 = 0.5;
    pub const TER_CROTCHET: f64 = 2. / 3.; // 0.66
    pub const TER_DOTTED_CROTCHET: f64 = 1.;
    pub const TER_MINIM: f64 = 4. / 3.; // 1.33
    pub const TER_DOTTED_MINIM: f64 = 2.;
    pub const TER_SEMIBREVE: f64 = 8. / 3.; // 2.66
    pub const TER_DOTTED_SEMIBREVE: f64 = 4.;
}

pub mod dynamic {
    use midly::num::u7;

    pub const SILENT: u7 = u7::new(0);
    pub const PPP: u7 = u7::new(10);
    pub const PP: u7 = u7::new(25);
    pub const P: u7 = u7::new(50);
    pub const MP: u7 = u7::new(60);
    pub const MF: u7 = u7::new(70);
    pub const F: u7 = u7::new(85);
    pub const FF: u7 = u7::new(100);
    pub const FFF: u7 = u7::new(120);
}
