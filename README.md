# D-Units (Digital Units)

The `D-Units` project's goal is to define a system, based on scientific definitions, to create and convert units.

> [!Important]
> Currently only units at the SI system are considered.
> Other systems might be integrated in the future.

## Definition of a Digital-Unit

This project defines a digital unit as a structure, composed of different parts in a raw-string, to point of a unit in the real world.
Each part starts with the character `\`, defining one of the following positions in the structure:

- _Prefix_: Such as 'kilo', 'milli', 'micro', etc.
- _Base_: Corresponding to the unit, as 'meter', 'liter', 'second', etc. The base is always in singular form.
- _Exponent_: Special part to define an exponent to the last prefix + base.

These three parts allows to define a simple digital unit.
To create a composed unit it is required to either:

- _multiply_ it with other unit. This requires no aditional part, defining the composed unit "newton meter" is per se the unit "newton" multiplied by "meter"
- _divide_ it over other unit. For this purpose, the part "per" is introduced. 

_**Examples**_

- liter (L) -> `"\liter"`
- kilonewton (kN) -> `"\kilo\newton"`
- square meter (m^2) -> `"\metre\tothe{2}"`
- meters per second (m/s) -> `"\metre\per\second"`
- Newton meter (Nm) -> `"\newton\metre"`

> [!Note]
> The base part for meter is the french `metre`, as that is how it is specified within the SI Brochure.

## External Links

- [**SI brochure**](https://www.bipm.org/en/publications/si-brochure): It contains the latests rules regarding the SI units.
- [`dsiUnits`](https://gitlab1.ptb.de/digitaldynamicmeasurement/dcc-and-dsi/dsiUnits): A python package with the goal to establish a digital library for SI units.
