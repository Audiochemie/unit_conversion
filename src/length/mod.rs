//* This module offers access to length conversion functions.
//* Conversion factors were taken from [here](https://physics.nist.gov/cuu/Constants/)

use lazy_static::lazy_static;
use std::collections::HashMap;

const BOHR_RADIUS_TO_METRES: f64 = 5.291_772_109_03e-11;
const CENTI: f64 = 1e-2;
const MILLI: f64 = 1e-3;
const MIKRO: f64 = 1e-6;
const NANO: f64 = 1e-9;
const PICO: f64 = 1e-12;

type Callback = fn(f64, prefix: &str) -> f64;

/// Unit function. Will simply return the value.
fn unity(v: f64, prefix: &str) -> f64 {
    v
}

/// Function to convert bohr to metres.
/// # Arguments
///  * `b` - value in bohr to convert
///  * `prefix` - can be one of [`pm`,`nm`, `mu`, `mm`, `cm`, `m`]. Will scale the value in metres to
///  pico-, nano-, micro-, milli-, centi-metres or not at all
///
fn bohr_to_metres(b: f64, prefix: &str) -> f64 {
    match prefix {
        "pm" => b * BOHR_RADIUS_TO_METRES / PICO,
        "nm" => b * BOHR_RADIUS_TO_METRES / NANO,
        "mu" => b * BOHR_RADIUS_TO_METRES / MIKRO,
        "mm" => b * BOHR_RADIUS_TO_METRES / MILLI,
        "cm" => b * BOHR_RADIUS_TO_METRES / CENTI,
        "m" => b * BOHR_RADIUS_TO_METRES,
        _ => panic!("Unkown prefix {}", prefix),
    }
}

/// Function to convert bohr to angstroem
/// # Arguments
///  * `b` - value in bohr to convert
///  * `prefix` - default: 'ang'. Can pass anything you like.
///
fn bohr_to_ang(b: f64, prefix: &str) -> f64 {
    bohr_to_metres(b, "nm") * 10.0
}

lazy_static! {
    pub static ref CONVERT_BOHR_TO_METRES: HashMap<&'static str, Callback> = {
        let mut t = HashMap::new();
        t.insert("bohr", unity as Callback);
        t.insert("m", bohr_to_metres as Callback);
        t
    };
    pub static ref CONVERT_BOHR_TO_ANG: HashMap<&'static str, Callback> = {
        let mut t = HashMap::new();
        t.insert("bohr", unity as Callback);
        t.insert("ang", bohr_to_ang as Callback);
        t
    };
}

#[cfg(test)]
mod unit_tests {
    use crate::length::CONVERT_BOHR_TO_ANG;

    use super::CONVERT_BOHR_TO_METRES;
    #[test]
    fn convert_bohr_to_metres() {
        let converted = CONVERT_BOHR_TO_METRES["m"](1.0_f64, "m");
        let expected = 5.291_772_109_03e-11;
        assert_eq!(converted, expected);
        let converted = CONVERT_BOHR_TO_METRES["m"](1.0_f64, "nm");
        let expected = 5.291_772_109_03e-2;
        assert_eq!(converted, expected);
    }

    #[test]
    fn convert_bohr_to_ang() {
        let converted = CONVERT_BOHR_TO_ANG["ang"](1.0_f64, "ang");
        let expected = 5.291_772_109_03e-1;
        assert_eq!(converted, expected);
    }
}
