use std::collections::HashMap;
use lazy_static::lazy_static;

/// This module offers all sorts of energy unit conversion functions.
/// Conversion factors are taken from [here](https://physics.nist.gov/cuu/Constants/energy.html)

const EV_REC_CENTIMETRES_CONVERSION_FACTOR: f64 = 8_065.543_937;
const REC_CENTIMETRES_EV_CONVERSION_FACTOR: f64 = 1.239_841_984_332 * 10e-8;

type Callback  = fn(f64) -> f64;

fn unity(energy_in_arb: f64 ) -> f64 { energy_in_arb }

fn ev_2_rcm(energy_in_ev: f64) -> f64 {
   energy_in_ev * EV_REC_CENTIMETRES_CONVERSION_FACTOR
}

fn rcm_2_ev(energy_in_rcm: f64) -> f64 {
   energy_in_rcm * REC_CENTIMETRES_EV_CONVERSION_FACTOR
}


lazy_static! {
    pub static ref CONVERT_2_RCM_FROM: HashMap<&'static str, Callback> = {
        let mut t = HashMap::new();
        t.insert("rcm", unity as Callback);
        t.insert("eV", ev_2_rcm as Callback);
    t
    };
}

lazy_static! {
    pub static ref CONVERT_2_EV_FROM: HashMap<&'static str, Callback> = {
        let mut t = HashMap::new();
        t.insert("eV", unity as Callback);
        t.insert("rcm", rcm_2_ev as Callback);
    t
    };
}


#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            2.0_f64 * EV_REC_CENTIMETRES_CONVERSION_FACTOR, CONVERT_2_RCM_FROM["eV"](2.0_f64)
        );
        assert_relative_eq!(
            7.439_051_905_992_01*10e-5, CONVERT_2_EV_FROM["rcm"](6000.0),
        )
    }
}
