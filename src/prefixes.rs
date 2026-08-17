//! Nominal (base-10) and binary (base-2) unit prefixes, and the shared [`ScaleFactor`]
//! trait used to compute their numeric scale.

use std::str::FromStr;

/// Error returned when a parsed string does not match any known prefix name.
#[derive(Debug)]
pub struct ParsePrefixError;

// -------------------------------------------------------------------
//                      Enums defining all prefixes
// -------------------------------------------------------------------

/// Nominal (SI) prefixes defined with base-10.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NominalPrefix {
    /// Value of 10^30
    Quetta,
    /// Value of 10^27
    Ronna,
    /// Value of 10^24
    Yotta,
    /// Value of 10^21
    Zetta,
    /// Value of 10^18
    Exa,
    /// Value of 10^15
    Peta,
    /// Value of 10^12
    Tera,
    /// Value of 10^9
    Giga,
    /// Value of 10^6
    Mega,
    /// Value of 10^3
    Kilo,
    /// Value of 100
    Hecto,
    /// Value of 10
    Deca,
    /// Value of 0.1
    Deci,
    /// Value of 0.01
    Centi,
    /// Value of 10^-3
    Milli,
    /// Value of 10^-6
    Micro,
    /// Value of 10^-9
    Nano,
    /// Value of 10^-12
    Pico,
    /// Value of 10^-15
    Femto,
    /// Value of 10^-18
    Atto,
    /// Value of 10^-21
    Zepto,
    /// Value of 10^-24
    Yocto,
    /// Value of 10^-27
    Ronto,
    /// Value of 10^-30
    Quecto,
}

/// Binary prefixes defined with base 2 (IEC 60027-2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryPrefix {
    /// Value of 2^10 = 1024
    Kibi,
    /// Value of 2^20 = 1024^2
    Mebi,
    /// Value of 2^30 = 1024^3
    Gibi,
    /// Value of 2^40 = 1024^4
    Tebi,
    /// Value of 2^50 = 1024^5
    Pebi,
    /// Value of 2^60 = 1024^6
    Exbi,
    /// Value of 2^70 = 1024^7
    Zebi,
    /// Value of 2^80 = 1024^8
    Yobi,
    /// Value of 2^90 = 1024^9
    Robi,
    /// Value of 2^100 = 1024^10
    Quebi,
}

// -------------------------------------------------------------------
//                        Extra functions per enums
// -------------------------------------------------------------------

impl NominalPrefix {
    /// Return the name of the member as a reference to a `str`.
    pub fn as_str(&self) -> &str {
        match &self {
            NominalPrefix::Quetta => "quetta",
            NominalPrefix::Ronna => "ronna",
            NominalPrefix::Yotta => "yotta",
            NominalPrefix::Zetta => "zetta",
            NominalPrefix::Exa => "exa",
            NominalPrefix::Peta => "peta",
            NominalPrefix::Tera => "tera",
            NominalPrefix::Giga => "giga",
            NominalPrefix::Mega => "mega",
            NominalPrefix::Kilo => "kilo",
            NominalPrefix::Hecto => "hecto",
            NominalPrefix::Deca => "deca",
            NominalPrefix::Deci => "deci",
            NominalPrefix::Centi => "centi",
            NominalPrefix::Milli => "milli",
            NominalPrefix::Micro => "micro",
            NominalPrefix::Nano => "nano",
            NominalPrefix::Pico => "pico",
            NominalPrefix::Femto => "femto",
            NominalPrefix::Atto => "atto",
            NominalPrefix::Zepto => "zepto",
            NominalPrefix::Yocto => "yocto",
            NominalPrefix::Ronto => "ronto",
            NominalPrefix::Quecto => "quecto",
        }
    }
}

impl BinaryPrefix {
    /// Return the name of the member as a reference to a `str`.
    pub fn as_str(&self) -> &str {
        match &self {
            BinaryPrefix::Kibi => "kibi",
            BinaryPrefix::Mebi => "mebi",
            BinaryPrefix::Gibi => "gibi",
            BinaryPrefix::Tebi => "tebi",
            BinaryPrefix::Pebi => "pebi",
            BinaryPrefix::Exbi => "exbi",
            BinaryPrefix::Zebi => "zebi",
            BinaryPrefix::Yobi => "yobi",
            BinaryPrefix::Robi => "robi",
            BinaryPrefix::Quebi => "quebi",
        }
    }
}

// -------------------------------------------------------------------
//                        Custom trait definitions
// -------------------------------------------------------------------

/// Trait allowing a prefix to be converted into a numeric scale factor.
///
/// Implemented by both [`NominalPrefix`] and [`BinaryPrefix`] to allow both enums to be
/// handled generically.
pub trait ScaleFactor {
    type ReturnType;
    /// Returns the multiplicative scale as an `f64`.
    ///
    /// E.G.: `Kilo.factor()` -> `1000.0` and `Kibi.factor()` -> `1024.0`
    fn factor(&self) -> Self::ReturnType;
}

// -------------------------------------------------------------------
//                  Traits implementation for the enums
// -------------------------------------------------------------------

/// Parses a prefix name (case-insensitive), e.g. `"kilo"` or `"KILO"` -> [`NominalPrefix::Kilo`].
///
/// # Errors
/// Returns [`ParsePrefixError`] if `s` does not match any known nominal prefix.
impl FromStr for NominalPrefix {
    type Err = ParsePrefixError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "quetta" => Ok(NominalPrefix::Quetta),
            "ronna" => Ok(NominalPrefix::Ronna),
            "yotta" => Ok(NominalPrefix::Yotta),
            "zetta" => Ok(NominalPrefix::Zetta),
            "exa" => Ok(NominalPrefix::Exa),
            "peta" => Ok(NominalPrefix::Peta),
            "tera" => Ok(NominalPrefix::Tera),
            "giga" => Ok(NominalPrefix::Giga),
            "mega" => Ok(NominalPrefix::Mega),
            "kilo" => Ok(NominalPrefix::Kilo),
            "hecto" => Ok(NominalPrefix::Hecto),
            "deca" => Ok(NominalPrefix::Deca),
            "deci" => Ok(NominalPrefix::Deci),
            "centi" => Ok(NominalPrefix::Centi),
            "milli" => Ok(NominalPrefix::Milli),
            "micro" => Ok(NominalPrefix::Micro),
            "nano" => Ok(NominalPrefix::Nano),
            "pico" => Ok(NominalPrefix::Pico),
            "femto" => Ok(NominalPrefix::Femto),
            "atto" => Ok(NominalPrefix::Atto),
            "zepto" => Ok(NominalPrefix::Zepto),
            "yocto" => Ok(NominalPrefix::Yocto),
            "ronto" => Ok(NominalPrefix::Ronto),
            "quecto" => Ok(NominalPrefix::Quecto),
            _ => Err(ParsePrefixError),
        }
    }
}

/// Parses a prefix name (case-insensitive), e.g. `"kibi"` or `"KIBI"` -> [`BinaryPrefix::Kibi`].
///
/// # Errors
/// Returns [`ParsePrefixError`] if `s` does not match any known binary prefix.
impl FromStr for BinaryPrefix {
    type Err = ParsePrefixError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kibi" => Ok(BinaryPrefix::Kibi),
            "mebi" => Ok(BinaryPrefix::Mebi),
            "gibi" => Ok(BinaryPrefix::Gibi),
            "tebi" => Ok(BinaryPrefix::Tebi),
            "pebi" => Ok(BinaryPrefix::Pebi),
            "exbi" => Ok(BinaryPrefix::Exbi),
            "zebi" => Ok(BinaryPrefix::Zebi),
            "yobi" => Ok(BinaryPrefix::Yobi),
            _ => Err(ParsePrefixError),
        }
    }
}

impl ScaleFactor for NominalPrefix {
    type ReturnType = f64;
    fn factor(&self) -> f64 {
        match self {
            NominalPrefix::Quetta => 1e30,
            NominalPrefix::Ronna => 1e27,
            NominalPrefix::Yotta => 1e24,
            NominalPrefix::Zetta => 1e21,
            NominalPrefix::Exa => 1e18,
            NominalPrefix::Peta => 1e15,
            NominalPrefix::Tera => 1e12,
            NominalPrefix::Giga => 1e9,
            NominalPrefix::Mega => 1e6,
            NominalPrefix::Kilo => 1e3,
            NominalPrefix::Hecto => 1e2,
            NominalPrefix::Deca => 1e1,
            NominalPrefix::Deci => 1e-1,
            NominalPrefix::Centi => 1e-2,
            NominalPrefix::Milli => 1e-3,
            NominalPrefix::Micro => 1e-6,
            NominalPrefix::Nano => 1e-9,
            NominalPrefix::Pico => 1e-12,
            NominalPrefix::Femto => 1e-15,
            NominalPrefix::Atto => 1e-18,
            NominalPrefix::Zepto => 1e-21,
            NominalPrefix::Yocto => 1e-24,
            NominalPrefix::Ronto => 1e-27,
            NominalPrefix::Quecto => 1e-30,
        }
    }
}

impl ScaleFactor for BinaryPrefix {
    type ReturnType = u128;
    fn factor(&self) -> u128 {
        match self {
            BinaryPrefix::Kibi => 1024,
            BinaryPrefix::Mebi => 1024_u128.pow(2),
            BinaryPrefix::Gibi => 1024_u128.pow(3),
            BinaryPrefix::Tebi => 1024_u128.pow(4),
            BinaryPrefix::Pebi => 1024_u128.pow(5),
            BinaryPrefix::Exbi => 1024_u128.pow(6),
            BinaryPrefix::Zebi => 1024_u128.pow(7),
            BinaryPrefix::Yobi => 1024_u128.pow(8),
            BinaryPrefix::Robi => 1024_u128.pow(9),
            BinaryPrefix::Quebi => 1024_u128.pow(10),
        }
    }
}

// -------------------------------------------------------------------
//                                 Tests
// -------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Testing all methods are fully implemented and no member is left without defining.
    // The purpose of this test is just to check all prefixes were defined correctly.
    #[test]
    fn nominal_prefix_completeness() {
        // "prefix_data" was manually created out of the SI brochure, section 3, table 7.
        let prefix_data: [(&str, f64); 24] = [
            ("deca", 1e1),
            ("hecto", 1e2),
            ("kilo", 1e3),
            ("mega", 1e6),
            ("giga", 1e9),
            ("tera", 1e12),
            ("peta", 1e15),
            ("exa", 1e18),
            ("zetta", 1e21),
            ("yotta", 1e24),
            ("ronna", 1e27),
            ("quetta", 1e30),
            ("deci", 1e-1),
            ("centi", 1e-2),
            ("milli", 1e-3),
            ("micro", 1e-6),
            ("nano", 1e-9),
            ("pico", 1e-12),
            ("femto", 1e-15),
            ("atto", 1e-18),
            ("zepto", 1e-21),
            ("yocto", 1e-24),
            ("ronto", 1e-27),
            ("quecto", 1e-30),
        ];

        for (name, factor) in prefix_data {
            let prefix = NominalPrefix::from_str(name);
            assert!(
                prefix.is_ok(),
                "Failed at prefix {}. Trait: `FromStr`",
                name.to_uppercase()
            );
            let prefix = prefix.unwrap();
            assert_eq!(
                prefix.as_str(),
                name,
                "Failed at prefix {}. Method: `as_str` not implemented",
                name.to_uppercase()
            );
            if factor > 1.0 {
                assert_eq!(
                    prefix.factor() as u128,
                    factor as u128,
                    "Failed at prefix {}. Trait `ScaleFactor`",
                    name.to_uppercase()
                )
            } else {
                let res = prefix.factor() / factor; // Result should be 1.0
                assert!(
                    (res - 1.0).abs() < 1e-6,
                    "Failed at prefix {}. Trait `ScaleFactor`",
                    name.to_uppercase()
                );
            }
        }
    }
}
