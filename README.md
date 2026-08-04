# unitary - Digital Units Operations

The `unitary` project's goal is to define a system, based on scientific definitions, to define and convert units digitally.

> [!Important]
> Currently only units at the SI metric system are considered.
> Other systems might be integrated in the future.

## Definition of a Digital Unit

This project defines a **digital unit** as a combination of different unit nodes, where each node points of a unit in the real world.
Each node is composed of three parts:

- _Prefix_: Such as 'kilo', 'milli', 'micro', etc.
- _Base_: Corresponding to the unit, as 'meter', 'liter', 'second', etc. The base is always in singular form.
- _Exponent_: Special part to define an exponent to the last prefix + base.

Where the raw string defining each part is tokenized with the `\` character.
For example, the unit "cubic decimeter" is composed by the prefix `\deci`, the base `\metre` and the exponent `\tothe{3}`.

To create a composed unit it is required to either:

- _multiply_ it with other unit node. This requires no additional part, defining the composed unit "newton meter" is per se the unit "newton" multiplied by "meter"
- _divide_ it over other unit node. For this purpose, the part "per" is introduced. 

### Examples

- liter (L) -> `"\liter"`
- kilonewton (kN) -> `"\kilo\newton"`
- square meter (m^2) -> `"\metre\tothe{2}"`
- meters per second (m/s) -> `"\metre\per\second"`
- Newton meter (Nm) -> `"\newton\metre"`

> [!Note]
> The base part for meter is the French `metre`, as that is how it is specified within the SI Brochure.

## External Links

- [**SI brochure**](https://www.bipm.org/en/publications/si-brochure): It contains the latest rules regarding the SI units.
- [`dsiUnits`](https://gitlab1.ptb.de/digitaldynamicmeasurement/dcc-and-dsi/dsiUnits): A python package with the goal to establish a digital library for SI units.
