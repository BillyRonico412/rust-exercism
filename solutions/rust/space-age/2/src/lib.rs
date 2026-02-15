// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const NB_SECOND_EARTH_YEAR: f64 = 31_557_600.0;

#[derive(Debug)]
pub struct Duration {
    s: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! planet_macro {
    ($planet_name:ident, $ratio_year_by_earth:expr) => {
        pub struct $planet_name;
        impl Planet for $planet_name {
            fn years_during(d: &Duration) -> f64 {
                let nb_second_by_year = $ratio_year_by_earth * NB_SECOND_EARTH_YEAR;
                (d.s as f64) / nb_second_by_year
            }
        }
    };
}

planet_macro!(Mercury, 0.2408467);
planet_macro!(Venus, 0.61519726);
planet_macro!(Earth, 1.0);
planet_macro!(Mars, 1.8808158);
planet_macro!(Jupiter, 11.862615);
planet_macro!(Saturn, 29.447498);
planet_macro!(Uranus, 84.016846);
planet_macro!(Neptune, 164.79132);
