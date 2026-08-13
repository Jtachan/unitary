//! Base units for different systems.

use std::str::FromStr;

/// Enumeration for all base units.
///
/// `unitary` defines as "base unit" any that cannot be simplified in terms of other units.
/// All other derived units can then be simplified related to the units in this enumeration,
/// holding only SI units (defined at the international
/// [BIPM SI Brochure](https://www.bipm.org/en/publications/si-brochure)).
///
/// All members of this enumeration are named in singular and after the names stated at
/// the [BIPM SI Brochure](https://www.bipm.org/en/publications/si-brochure).
/// Any unique feature is explained at the member's description.
///
/// This enumeration holds more units than the base units defined at the SI brochure, expanding
/// the definition of some units as "base unit" as a unit that cannot be simplified anymore or
/// its simplification might lack of further meaning.
/// The 'radian' (with the simplification `rad = m/m`) is the example of one of them, where its
/// simplification without further meaning would not bring more clarification to the unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaseUnit {
    /// SI unit for **time**, represented with the unit symbol `s`.
    Second,
    /// SI unit for **length**, represented with the unit the symbol `m`.
    /// The notation is defined as "metre" and not "meter" as that is the way defined in the
    /// [BIPM SI Brochure](https://www.bipm.org/en/publications/si-brochure).
    Metre,
    /// SI unit for **mass**, represented with the unit the symbol `g`.
    /// Gram is used instead of 'Kilogram' (SI defined base unit for mass in the
    /// [BIPM SI Brochure](https://www.bipm.org/en/publications/si-brochure)) for
    /// simplification purposes.
    Gram,
    /// SI unit for **electric current**, represented with the unit the symbol `A`.
    Ampere,
    /// SI unit for **thermodynamic temperature**, represented with the unit the symbol `T`.
    Kelvin,
    /// SI unit for **amount of substance**, represented with the unit the symbol `mol`.
    Mole,
    /// SI unit for **luminous intensity (luminosity)**, represented with the unit the symbol `cd`.
    Candela,
    /// Unit for **adimensional quantities**, without a symbol to represent.
    /// This member cannot be defined together with any prefix.
    One,
    /// Unit for **phase angles**, represented with the symbol `rad`.
    Radian,
    /// Unit for **binary information**, represented with the symbol `bit`.
    /// The binary information is the only quantity that can be expressed with `NominalPrefixes`
    /// (kilo, milli, mega, etc.) as well as with `BinaryPrefixes`
    Bit,
}

#[derive(Debug)]
pub struct ParseUnitError {
    _message: String,
}

/// Parses a string (either unit name or type of quantity for the unit) as case-insensitive.
///
/// The only valid quantity names to be parsed are:
/// - `"time"` for second
/// - `"length"` for meter
/// - `"mass"` for gram
/// - `"current"` for ampere
/// - `"temperature"` for kelvin
/// - `"substance"` for mole
/// - `"luminosity"` for candela
/// - `"adimensional"` for one
///
/// # Examples
/// `"Second"` -> [`BaseUnit::Second`]
/// `"time"` -> [`BaseUnit::Second`]
///
/// # Errors
/// Returns [`ParseUnitError`] if `s` does not match with either
impl FromStr for BaseUnit {
    type Err = ParseUnitError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "second" | "time" => Ok(BaseUnit::Second),
            "metre" | "length" => Ok(BaseUnit::Metre),
            "gram" | "mass" => Ok(BaseUnit::Gram),
            "ampere" | "current" => Ok(BaseUnit::Ampere),
            "kelvin" | "temperature" => Ok(BaseUnit::Kelvin),
            "mole" | "substance" => Ok(BaseUnit::Mole),
            "candela" | "luminosity" => Ok(BaseUnit::Candela),
            "one" | "adimensional" => Ok(BaseUnit::One),
            _ => Err(ParseUnitError {
                // Todo: Use the error for panicking at user level.
                _message: format!(
                    "'{}' is not a valid SI base unit. Make sure your unit has no typos \
                    and it is in singular. E.G.: 'second' instead of 'seconds' or 'metre' \
                    instead of 'meter'. ",
                    s
                ),
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_si_unit_parsing() {
        let si_unit_names = [
            ("second", "time"),
            ("metre", "length"),
            ("gram", "mass"),
            ("ampere", "current"),
            ("kelvin", "temperature"),
            ("mole", "substance"),
            ("candela", "luminosity"),
            ("one", "adimensional"),
        ];

        for (unit, quantity) in si_unit_names.iter() {
            let base_unit: BaseUnit = unit.parse().unwrap();
            let base_quantity = BaseUnit::from_str(quantity).unwrap();
            assert_eq!(base_unit, base_quantity);
        }

        let result = BaseUnit::from_str("meter");
        assert!(result.is_err());
    }
}
