//! Electric charge areal density (base UnitDefinition coulomb per square meter, m⁻² · A · s).
use crate::{prefix, quantity};
quantity! {
    ///Electric charge areal density (base UnitDefinition coulomb per square meter, m⁻² · A · s).
    quantity: ElectricChargeArealDensity; "electric charge areal density";
    /// Dimension of electric charge areal density, L⁻²TI (base UnitDefinition coulomb per square meter,
    /// m⁻² · A · s).
    dimension: ISQ<
        N2,     // length
        Z0,     // mass
        P1,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
        kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @coulomb_per_square_meter: prefix!(none); "C/m²", "coulomb per square meter",
            "coulombs per square meter";
        @coulomb_per_square_centimeter: prefix!(none) / prefix!(centi) / prefix!(centi); "C/cm²",
            "coulomb per square centimeter", "coulombs per square centimeter";
    }
}

#[cfg(test)]
mod tests {
use crate::traits::Unit;
    use crate::{unit_definitions::{area::AreaUnit, electric_charge::ElectricChargeUnit}, units::ElectricChargeArealDensityUnit, units_base::UnitDefinition};


    #[test]
    fn check_dimension() {
        assert_eq!(ElectricChargeArealDensityUnit::base(), ElectricChargeUnit::base() / AreaUnit::base());
    }

    #[test]
    fn check_units() {
        test_unit(ElectricChargeUnit::coulomb, AreaUnit::square_meter, ElectricChargeArealDensityUnit::coulomb_per_square_meter);
        test_unit(ElectricChargeUnit::coulomb, AreaUnit::square_centimeter, ElectricChargeArealDensityUnit::coulomb_per_square_centimeter);        
    }
    fn test_unit(charge: ElectricChargeUnit, area: AreaUnit, value: ElectricChargeArealDensityUnit)
    {
        assert_eq!(Into::<UnitDefinition>::into(value), (Into::<UnitDefinition>::into(charge) / Into::<UnitDefinition>::into(area)));
    }
    
}
