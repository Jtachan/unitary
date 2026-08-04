pub trait ScaleFactor {
    type Output;
    fn factor(self) -> Self::Output;
}

/// All SI prefixes defined with base 10
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Prefix {
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
    type Output = f32;
    fn factor(self) -> f32 {
        match self {
            Prefix::Quetta => 10.0_f32.powi(30),
            Prefix::Ronna => 10.0_f32.powi(27),
            Prefix::Yotta => 10.0_f32.powi(24),
            Prefix::Zetta => 10.0_f32.powi(21),
            Prefix::Exa => 10.0_f32.powi(18),
            Prefix::Peta => 10.0_f32.powi(15),
            Prefix::Tera => 10.0_f32.powi(12),
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
            Prefix::Pico => 10.0_f32.powi(-12),
            Prefix::Femto => 10.0_f32.powi(-15),
            Prefix::Atto => 10.0_f32.powi(-18),
            Prefix::Zepto => 10.0_f32.powi(-21),
            Prefix::Ronto => 10.0_f32.powi(-27),
            Prefix::Quecto => 10.0_f32.powi(-30),
        }
    }
}

/// All binary prefixes defined with base 2
enum BinaryPrefix {
    Kibi, // 2^10 = 1024
    Mebi, // 2^20 = 1024^2
    Gibi, // 2^30
    Tebi, // 2^40
    Pebi, // 2^50
    Exbi, // 2^60
    Zebi, // 2^70
    Yobi, // 2^80
}

impl ScaleFactor for BinaryPrefix {
    type Output = u128;
    fn factor(self) -> u128 {
        match self {
            BinaryPrefix::Kibi => 1024,
            BinaryPrefix::Mebi => 1024_u128.pow(2),
            BinaryPrefix::Gibi => 1024_u128.pow(3),
            BinaryPrefix::Tebi => 1024_u128.pow(4),
            BinaryPrefix::Pebi => 1024_u128.pow(5),
            BinaryPrefix::Exbi => 1024_u128.pow(6),
            BinaryPrefix::Zebi => 1024_u128.pow(7),
            BinaryPrefix::Yobi => 1024_u128.pow(8),
        }
    }
}
