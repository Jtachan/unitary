//! Base units for different systems.

use std::str::FromStr;

#[derive(Debug)]
pub struct ParseUnitError;

// -------------------------------------------------------------------
//                      Enums defining all units
// -------------------------------------------------------------------

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
    /// The notation is defined as "meter" and not "metre" (as defined in the
    /// [BIPM SI Brochure](https://www.bipm.org/en/publications/si-brochure)) to avoid
    /// possible confusions while using the package.
    Meter,
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

/// Enumeration for all derived units.
///
/// `unitary` consider a "derived unit" as any unit which can be simplified as a group of one
/// or more base units, joined together through mathematical operations.
///
/// **Examples**
/// `liter` -> cubic decimeter (`\deci\meter\tothe{3}`)
/// `byte`  -> four (4) bits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DerivedUnit {
    // ------------ Accepted SI units ------------
    /// Derived unit (SI) for **solid angles**, defined as a radian `rad`.
    /// Represented with the symbol `sr`.
    /// While radians and steradians are numerically equal, the SI defines the steradian as the
    /// default unit for any 3D measured angles.
    Steradian,
    /// Derived unit (SI) for **frequency**, defined as inverse second `s^-1`.
    /// Represented with the symbol `Hz`.
    Hertz,
    /// Derived unit (SI) for **force**, defined as `kg * m * s^-2`.
    /// Represented with the symbol `N`.
    Newton,
    /// Derived unit (SI) for **pressure**, defined as Newtons per square meter `N / m^2`
    /// (or `kg * m^-1 * s^-2`).
    /// Represented with the symbol `Pa`.
    Pascal,
    /// Derived unit (SI) for **energy** (amount of heat), defined as Newtons time meters `N * m`
    /// (or `kg * m^2 * s^-2`).
    /// Represented with the symbol `J`.
    Joule,
    /// Derived unit (SI) for **power**, defined as Joules per second `J / s`
    /// (or `kg * m^2 * s^-3`).
    /// Represented with the symbol `W`.
    Watt,
    /// Derived unit (SI) for **electric charge**, defined as ampere times second `A * s`
    /// Represented with the symbol `C`.
    Coulomb,
    /// Derived unit (SI) for **electric potential difference**, defined as watt per ampere `W / A`
    /// (or `kg * m^2 * s^-3 * A^-1`).
    /// Represented with the symbol `V`.
    Volt,
    /// Derived unit (SI) for **capacitance**, defined as coulomb per volt `C / V`
    /// (or `kg^-1 * m^-2 * s^4 * A^2`).
    /// Represented with the symbol `F`.
    Farad,
    /// Derived unit (SI) for **electric resistance**, defined as volt per ampere `V / A`
    /// (or `kg * m^2 * s^-3 * A^-2`).
    /// Represented with the symbol `Ω` (omega).
    Ohm,
    /// Derived unit (SI) for **electric conductance**, defined as ampere per volt `A / V`
    /// (or `kg^-1 * m^-2 * s^3 * A^2`).
    /// Represented with the symbol `S`.
    Siemens,
    /// Derived unit (SI) for **magnetic flux**,  defined as volts times second `V * s`
    /// (or `kg * m^2 * s^-2 * A^-1`).
    /// Represented with the symbol `Wb`.
    Weber,
    /// Derived unit (SI) for **magnetic flux density**, defined as weber per square meter `Wb / m^2`
    /// (or `kg * s^-2 * A^-1`).
    /// Represented with the symbol `T`.
    Tesla,
    /// Derived unit (SI) for **inductance**, defined as weber per ampere `Wb / A`
    /// (or `kg * m^2 * s^-2 * A^-2`).
    /// Represented with the symbol `H`.
    Henry,
    /// Derived unit (SI) for **temperature**, defined as a difference (offset) of 273.15 Kelvin `273.15 K`.
    /// Represented with the symbol `°C`.
    Celsius,
    /// Derived unit (SI) for **luminous flux**, defined as candela times steradian `cd * sr`.
    /// Represented with the symbol `lm`.
    Lumen,
    /// Derived unit (SI) for **illuminance**, defined as lumen per square meter `lm / m^s`
    /// (or `cd * sr * m^-2`).
    /// Represented with the symbol `lm`.
    Lux,
    /// Derived unit (SI) for **activity referred to a radiounuclide**, defined as inverse seconds `s^-1`.
    /// Represented with the symbol `Bq`.
    /// This quantity is sometimes incorrectly called as _radioactivity_.
    Becquerel,
    /// Derived unit (SI) for **absorbed dose** (_kerna_), defined as joule per kilogram `J / kg`
    /// (or `m^2 * s^-2`).
    /// Represented with the symbol `Bq`.
    Gray,
    /// Derived unit (SI) for **dose equivalent**, defined as joule per kilogram `J / kg`
    /// (or `m^2 * s^-2`).
    /// Represented with the symbol `Sv`.
    Sievert,
    /// Derived unit (SI) for **catalytic activity**, defined as mole per second `mol / s`
    /// Represented with the symbol `kat`.
    Katal,

    // todo: integrate the following units
    // ------------ Non-SI units ------------
    // /// Derived unit for **time**, defined as 60 seconds `60 s`.
    // /// Represented with the symbol `min`.
    // Minute,
    // /// Derived unit for **time**, defined as 60 minutes `60 min` (or `3_600 s`).
    // /// Represented with the symbol `h`.
    // Hour,
    // /// Derived unit for **time**, defined as 24 hours `24 h` (or `86_400 s`).
    // /// Represented with the symbol `d`.
    // Day,
    // /// Derived unit for **phase angle**, defined as pi/180 radians,
    // /// Represented with the symbol `°`.
    // Degree,
    // /// Derived unit for **area**, defined as one square hectometre `hm^2` (or `10^4 m^2`).
    // /// Represented with the symbol `ha`.
    // Hectare,
    // /// Derived unit for **volume**, defined as a cubic decimeter `dm^3` (10^-3 cubic meters) and
    // /// represented with the symbol `L`.
    // /// The SI Brochure accepts both symbols `l` and `L` to define liters. In order to avoid
    // /// confusion, `unitary` uses the symbol `L` to diferenciate liters from the numeral one (`1`)
    // /// and the capital letter `I`, which might look like the lowercase `l` at some fonts.
    // Liter,
    // /// Derived unit for **mass**, defined as `10^3 kg`.
    // /// Represented with the symbol `t`.
    // Tonne,
    // ------------ Binary units ------------
    /// Derived unit for **binary information**, defined as four bits (`2^2 bit`).
    /// Represented with the symbol `B`.
    Byte,
}

// -------------------------------------------------------------------
//                        Extra functions per enums
// -------------------------------------------------------------------

impl BaseUnit {
    /// Return the name of the member as a reference to a `str`.
    pub fn as_str(&self) -> &str {
        match &self {
            BaseUnit::Second => "second",
            BaseUnit::Meter => "meter",
            BaseUnit::Gram => "gram",
            BaseUnit::Ampere => "ampere",
            BaseUnit::Kelvin => "kelvin",
            BaseUnit::Mole => "mole",
            BaseUnit::Candela => "candela",
            BaseUnit::One => "one",
            BaseUnit::Radian => "radian",
            BaseUnit::Bit => "bit",
        }
    }
}

impl DerivedUnit {
    /// Return the name of the member as a reference to a `str`.
    /// The capitalisation of each member is based on the SI brochure.
    pub fn as_str(&self) -> &str {
        match &self {
            DerivedUnit::Steradian => "steradian",
            DerivedUnit::Hertz => "hertz",
            DerivedUnit::Newton => "newton",
            DerivedUnit::Pascal => "pascal",
            DerivedUnit::Joule => "joule",
            DerivedUnit::Watt => "watt",
            DerivedUnit::Coulomb => "coulomb",
            DerivedUnit::Volt => "volt",
            DerivedUnit::Farad => "farad",
            DerivedUnit::Ohm => "ohm",
            DerivedUnit::Siemens => "siemens",
            DerivedUnit::Weber => "weber",
            DerivedUnit::Tesla => "tesla",
            DerivedUnit::Henry => "henry",
            DerivedUnit::Celsius => "Celsius",
            DerivedUnit::Lumen => "lumen",
            DerivedUnit::Lux => "lux",
            DerivedUnit::Becquerel => "becquerel",
            DerivedUnit::Gray => "gray",
            DerivedUnit::Sievert => "sievert",
            DerivedUnit::Katal => "katal",
            DerivedUnit::Byte => "byte",
        }
    }
}

// -------------------------------------------------------------------
//                          Custom trait definitions
// -------------------------------------------------------------------

/// Trait to simplify any unit to its base unit.
/// The result of the simplification is the member of the [`BaseUnit`] enum together with
/// the scale factor from the original unit to its base unit.
// todo: modify for those derived units that use multiple base units.
pub trait UnitSimplify {
    fn to_base_unit(&self) -> (BaseUnit, f64);
}

// -------------------------------------------------------------------
//                  Traits implementation for the enums
// -------------------------------------------------------------------

/// Parses a string (either unit name or type of quantity for the unit) as case-insensitive.
///
/// # Examples
/// `"Second"` -> [`BaseUnit::Second`]
///
/// # Errors
/// Returns [`ParseUnitError`] if `s` does not match any name.
impl FromStr for BaseUnit {
    type Err = ParseUnitError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "second" => Ok(BaseUnit::Second),
            "meter" => Ok(BaseUnit::Meter),
            "gram" => Ok(BaseUnit::Gram),
            "ampere" => Ok(BaseUnit::Ampere),
            "kelvin" => Ok(BaseUnit::Kelvin),
            "mole" => Ok(BaseUnit::Mole),
            "candela" => Ok(BaseUnit::Candela),
            "one" => Ok(BaseUnit::One),
            "radian" => Ok(BaseUnit::Radian),
            "bit" => Ok(BaseUnit::Bit),
            _ => Err(ParseUnitError),
        }
    }
}

/// Parses a string (either unit name or type of quantity for the unit) as case-insensitive.
///
/// # Examples
/// `"Hertz"` -> [`DerivedUnit::Hertz`]
///
/// # Errors
/// Returns [`ParseUnitError`] if `s` does not match any name.
impl FromStr for DerivedUnit {
    type Err = ParseUnitError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "steradian" => Ok(DerivedUnit::Steradian),
            "hertz" => Ok(DerivedUnit::Hertz),
            "newton" => Ok(DerivedUnit::Newton),
            "pascal" => Ok(DerivedUnit::Pascal),
            "joule" => Ok(DerivedUnit::Joule),
            "watt" => Ok(DerivedUnit::Watt),
            "coulomb" => Ok(DerivedUnit::Coulomb),
            "volt" => Ok(DerivedUnit::Volt),
            "farad" => Ok(DerivedUnit::Farad),
            "ohm" => Ok(DerivedUnit::Ohm),
            "siemens" => Ok(DerivedUnit::Siemens),
            "weber" => Ok(DerivedUnit::Weber),
            "tesla" => Ok(DerivedUnit::Tesla),
            "henry" => Ok(DerivedUnit::Henry),
            "celsius" => Ok(DerivedUnit::Celsius),
            "lumen" => Ok(DerivedUnit::Lumen),
            "lux" => Ok(DerivedUnit::Lux),
            "becquerel" => Ok(DerivedUnit::Becquerel),
            "gray" => Ok(DerivedUnit::Gray),
            "sievert" => Ok(DerivedUnit::Sievert),
            "katal" => Ok(DerivedUnit::Katal),
            "byte" => Ok(DerivedUnit::Byte),
            _ => Err(ParseUnitError),
        }
    }
}

// -------------------------------------------------------------------
//                                 Tests
// -------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_unit_completeness() {
        let si_unit_names: [&str; 10] = [
            "second", "meter", "gram", "ampere", "kelvin", "mole", "candela", "one", "radian",
            "bit",
        ];

        for unit_name in si_unit_names.iter() {
            let base_unit = BaseUnit::from_str(unit_name);
            assert!(
                base_unit.is_ok(),
                "[Trait `FromStr`] Failed at unit {}",
                unit_name.to_uppercase()
            );
            let base_unit = base_unit.unwrap();
            assert_eq!(
                base_unit.as_str(),
                *unit_name,
                "[Method `as_str`] Failed at unit {}.",
                unit_name.to_uppercase()
            );
        }

        let result = BaseUnit::from_str("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn derived_unit_completeness() {
        let derived_unit_names: [&str; 22] = [
            "steradian",
            "hertz",
            "newton",
            "pascal",
            "joule",
            "watt",
            "coulomb",
            "volt",
            "farad",
            "ohm",
            "siemens",
            "weber",
            "tesla",
            "henry",
            "Celsius",
            "lumen",
            "lux",
            "becquerel",
            "gray",
            "sievert",
            "katal",
            "byte",
        ];

        for unit_name in derived_unit_names.iter() {
            let unit = DerivedUnit::from_str(unit_name);
            assert!(
                unit.is_ok(),
                "[Trait `FromStr`] Failed at unit {}",
                unit_name.to_uppercase()
            );
            let unit = unit.unwrap();
            assert_eq!(
                unit.as_str(),
                *unit_name,
                "[Method `as_str`] Failed at unit {}.",
                unit_name.to_uppercase()
            );
        }
    }
}
