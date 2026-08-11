//! Base units for different systems.

use std::str::FromStr;

/// Enumeration for all base units at the SI.
///
/// All the members of this enumeration are named in singular and after the names
/// stated at the international [BIPM SI Brochure](https://www.bipm.org/en/publications/si-brochure).
/// Any unique feature is explained at the member's description.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaseSIUnit {
    /// Unit for time, represented with the unit symbol `s`.
    Second,
    /// Unit for length, represented with the unit the symbol `m`.
    /// The notation is defined as "metre" and not "meter" as that is the way defined in the
    /// [BIPM SI Brochure](https://www.bipm.org/en/publications/si-brochure).
    Metre,
    /// Unit for mass, represented with the unit the symbol `g`.
    /// Gram is used instead of 'Kilogram' (SI defined base unit for mass in the
    /// [BIPM SI Brochure](https://www.bipm.org/en/publications/si-brochure)) for
    /// simplification purposes.
    Gram,
    /// Unit for electric current, represented with the unit the symbol `A`.
    Ampere,
    /// Unit for thermodynamic temperature, represented with the unit the symbol `T`.
    Kelvin,
    /// Unit for amount of substance, represented with the unit the symbol `mol`.
    Mole,
    /// Unit for luminous intensity (luminosity), represented with the unit the symbol `cd`.
    Candela,
    /// Unit for adimensional quantities, without a symbol to represent.
    /// This member cannot be defined together with any prefix.
    One,
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
/// `"Second"` -> [`BaseSIUnit::Second`]
/// `"time"` -> [`BaseSIUnit::Second`]
///
/// # Errors
/// Returns [`ParseUnitError`] if `s` does not match with either
impl FromStr for BaseSIUnit {
    type Err = ParseUnitError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "second" | "time" => Ok(BaseSIUnit::Second),
            "metre" | "length" => Ok(BaseSIUnit::Metre),
            "gram" | "mass" => Ok(BaseSIUnit::Gram),
            "ampere" | "current" => Ok(BaseSIUnit::Ampere),
            "kelvin" | "temperature" => Ok(BaseSIUnit::Kelvin),
            "mole" | "substance" => Ok(BaseSIUnit::Mole),
            "candela" | "luminosity" => Ok(BaseSIUnit::Candela),
            "one" | "adimensional" => Ok(BaseSIUnit::One),
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
            let base_unit: BaseSIUnit = unit.parse().unwrap();
            let base_quantity = BaseSIUnit::from_str(quantity).unwrap();
            assert_eq!(base_unit, base_quantity);
        }

        let result = BaseSIUnit::from_str("meter");
        assert!(result.is_err());
    }
}
