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
    None,   // 1
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

/// All binary prefixes defined with base 2
enum BinaryPrefix {
    Kibi,   // 2^10 = 1024
    Mebi,   // 2^20
    Gibi,   // 2^30
    Tebi,   // 2^40
    Pebi,   // 2^50
    Exbi,   // 2^60
    Zebi,   // 2^70
    Yobi,   // 2^80
}
