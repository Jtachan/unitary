pub trait ScaleFactor {
    fn factor(&self) -> f64;
}

/// All SI prefixes defined with base 10
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Prefix {
    Quetta, // 1e30
    Ronna,  // 1e27
    Yotta,  // 1e24
    Zetta,  // 1e21
    Exa,    // 1e18
    Peta,   // 1e15
    Tera,   // 1e12
    Giga,   // 1e9
    Mega,   // 1e6
    Kilo,   // 1e3
    Hecto,  // 100
    Deca,   // 10
    Deci,   // 0.1
    Centi,  // 0.01
    Milli,  // 1e-3
    Micro,  // 1e-6
    Nano,   // 1e-9
    Pico,   // 1e-12
    Femto,  // 1e-15
    Atto,   // 1e-18
    Zepto,  // 1e-21
    Ronto,  // 1e-27
    Quecto, // 1e-30
}

impl ScaleFactor for Prefix {
    fn factor(&self) -> f64 {
        match self {
            Prefix::Quetta => 1_000_000_000_000_000_000_000_000_000_000.0,
            Prefix::Ronna => 1_000_000_000_000_000_000_000_000_000.0,
            Prefix::Yotta => 1_000_000_000_000_000_000_000_000.0,
            Prefix::Zetta => 1_000_000_000_000_000_000_000.0,
            Prefix::Exa => 1_000_000_000_000_000_000.0,
            Prefix::Peta => 1_000_000_000_000_000.0,
            Prefix::Tera => 1_000_000_000_000.0,
            Prefix::Giga => 1_000_000_000.0,
            Prefix::Mega => 1_000_000.0,
            Prefix::Kilo => 1_000.0,
            Prefix::Hecto => 100.0,
            Prefix::Deca => 10.0,
            Prefix::Deci => 0.1,
            Prefix::Centi => 0.01,
            Prefix::Milli => 0.001,
            Prefix::Micro => 0.000_001,
            Prefix::Nano => 0.000_000_001,
            Prefix::Pico => 0.000_000_000_001,
            Prefix::Femto => 0.000_000_000_000_001,
            Prefix::Atto => 0.000_000_000_000_000_001,
            Prefix::Zepto => 0.000_000_000_000_000_000_001,
            Prefix::Ronto => 0.000_000_000_000_000_000_000_001,
            Prefix::Quecto => 0.000_000_000_000_000_000_000_000_001,
        }
    }
}

/// All binary prefixes defined with base 2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryPrefix {
    Kibi, // 2^10 = 1024
    Mebi, // 2^20 = 1024^2
    Gibi, // 2^30 = 1024^3
    Tebi, // 2^40 = 1024^4
    Pebi, // 2^50 = 1024^5
    Exbi, // 2^60 = 1024^6
    Zebi, // 2^70 = 1024^7
    Yobi, // 2^80 = 1024^8
}

impl ScaleFactor for BinaryPrefix {
    fn factor(&self) -> f64 {
        match self {
            BinaryPrefix::Kibi => 1024.0,
            BinaryPrefix::Mebi => 1048576.0,
            BinaryPrefix::Gibi => 1073741824.0,
            BinaryPrefix::Tebi => 1099511627776.0,
            BinaryPrefix::Pebi => 1125899906842624.0,
            BinaryPrefix::Exbi => 1152921504606846976.0,
            BinaryPrefix::Zebi => 1180591620717411303424.0,
            BinaryPrefix::Yobi => 1208925819614629174706176.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;

    #[test]
    fn base_10_scale_factor() {
        assert_relative_eq!(Prefix::Quetta.factor(), 10.0_f64.powi(30));
        assert_relative_eq!(Prefix::Ronna.factor(), 10.0_f64.powi(27));
        assert_relative_eq!(Prefix::Yotta.factor(), 10.0_f64.powi(24));
        assert_relative_eq!(Prefix::Zetta.factor(), 10.0_f64.powi(21));
        assert_relative_eq!(Prefix::Exa.factor(), 10.0_f64.powi(18));
        assert_relative_eq!(Prefix::Peta.factor(), 10.0_f64.powi(15));
        assert_relative_eq!(Prefix::Tera.factor(), 10.0_f64.powi(12));
        assert_relative_eq!(Prefix::Giga.factor(), 10.0_f64.powi(9));
        assert_relative_eq!(Prefix::Mega.factor(), 10.0_f64.powi(6));
        assert_relative_eq!(Prefix::Kilo.factor(), 10.0_f64.powi(3));
        assert_relative_eq!(Prefix::Hecto.factor(), 10.0_f64.powi(2));
        assert_relative_eq!(Prefix::Deca.factor(), 10.0_f64.powi(1));
        assert_relative_eq!(Prefix::Deci.factor(), 10.0_f64.powi(-1));
        assert_relative_eq!(Prefix::Centi.factor(), 10.0_f64.powi(-2));
        assert_relative_eq!(Prefix::Milli.factor(), 10.0_f64.powi(-3));
        assert_relative_eq!(Prefix::Micro.factor(), 10.0_f64.powi(-6));
        assert_relative_eq!(Prefix::Nano.factor(), 10.0_f64.powi(-9));
        assert_relative_eq!(Prefix::Pico.factor(), 10.0_f64.powi(-12));
        assert_relative_eq!(Prefix::Femto.factor(), 10.0_f64.powi(-15));
        assert_relative_eq!(Prefix::Atto.factor(), 10.0_f64.powi(-18));
        assert_relative_eq!(Prefix::Zepto.factor(), 10.0_f64.powi(-21));
        assert_relative_eq!(Prefix::Ronto.factor(), 10.0_f64.powi(-27));
        assert_relative_eq!(Prefix::Quecto.factor(), 10.0_f64.powi(-30));
    }

    #[test]
    fn base_2_scale_factor() {
        assert_relative_eq!(BinaryPrefix::Kibi.factor(), 2.0_f64.powi(10));
        assert_relative_eq!(BinaryPrefix::Mebi.factor(), 2.0_f64.powi(20));
        assert_relative_eq!(BinaryPrefix::Gibi.factor(), 2.0_f64.powi(30));
        assert_relative_eq!(BinaryPrefix::Tebi.factor(), 2.0_f64.powi(40));
        assert_relative_eq!(BinaryPrefix::Pebi.factor(), 2.0_f64.powi(50));
        assert_relative_eq!(BinaryPrefix::Exbi.factor(), 2.0_f64.powi(60));
        assert_relative_eq!(BinaryPrefix::Zebi.factor(), 2.0_f64.powi(70));
        assert_relative_eq!(BinaryPrefix::Yobi.factor(), 2.0_f64.powi(80));
    }
}
