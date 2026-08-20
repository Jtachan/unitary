use super::prefixes::{BinaryPrefix, NominalPrefix, ScaleFactor};
use super::units::{BaseUnit, DerivedUnit, ParseUnitError};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NamedUnit {
    Base(BaseUnit),
    Derived(DerivedUnit),
}

impl FromStr for NamedUnit {
    type Err = ParseUnitError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let unit = BaseUnit::from_str(s);

        if unit.is_ok() {
            Ok(NamedUnit::Base(unit?))
        } else {
            let unit = DerivedUnit::from_str(s);
            if unit.is_ok() {
                Ok(NamedUnit::Derived(unit?))
            } else {
                Err(ParseUnitError)
            }
        }
    }
}

impl NamedUnit {
    pub fn as_str(&self) -> &str {
        match self {
            NamedUnit::Base(p) => p.as_str(),
            NamedUnit::Derived(p) => p.as_str(),
        }
    }
}

/// Enumeration wrapping over all prefix types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitPrefix {
    Nominal(NominalPrefix),
    Binary(BinaryPrefix),
}

impl UnitPrefix {
    pub fn as_str(&self) -> &str {
        match self {
            UnitPrefix::Nominal(p) => p.as_str(),
            UnitPrefix::Binary(p) => p.as_str(),
        }
    }
}

impl ScaleFactor for UnitPrefix {
    fn factor(&self) -> f64 {
        match self {
            UnitPrefix::Nominal(p) => p.factor(),
            UnitPrefix::Binary(p) => p.factor(),
        }
    }
}

pub struct UnitNode {
    prefix: Option<UnitPrefix>,
    unit: NamedUnit,
    exponent: i16,
}
