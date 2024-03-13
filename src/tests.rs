#[cfg(test)]
mod test
{   
    #[test]
    #[cfg(all(any(feature="All", all(feature="Time", feature="Length", feature="Velocity", feature="Acceleration")), feature="std", feature="serde"))]
    fn test_unit_serialization()
    {
        use crate::{units::LengthUnit, Length, Quantities, Units};

        assert_eq!(serde_json::json!(Length::meter(10.0)).to_string(), "{\"unit\":\"meter\",\"value\":10.0}");
        assert_eq!(serde_json::json!(Quantities::Length(Length::angstrom(1.0))).to_string(), "{\"Length\":{\"unit\":\"angstrom\",\"value\":1.0}}");

        assert_eq!(serde_json::json!(LengthUnit::centimeter), "centimeter");
        assert_eq!(serde_json::json!(Units::Length(LengthUnit::centimeter)).to_string(), "{\"Length\":\"centimeter\"}");
    }

    #[test]
    #[cfg(all(any(feature="All", all(feature="Time", feature="Pressure"))))]
    fn test_try_convert()
    {
        use crate::{Time, Units, Pressure};         
        let pressure = Pressure::poundal_per_square_foot(1.0);
        let time = Time::second(5.0);
        assert!(pressure.try_convert(Units::Pressure(crate::units::PressureUnit::pascal)).is_ok());
        println!("{:?}", pressure.try_convert(Units::Pressure(crate::units::PressureUnit::psi)));
        assert!(pressure.try_convert(Units::Pressure(crate::units::PressureUnit::psi)).is_ok());
        assert!(time.try_convert(Units::Length(crate::units::LengthUnit::centimeter)).is_err());
    }
    #[test]
    #[cfg(all(any(feature="All", all(feature="Time", feature="Length", feature="Velocity", feature="Acceleration", feature="Force", feature="Mass")), feature="std"))]
    fn check_ops()
    {
        use crate::{quantity::QuantityBase, Acceleration, Force, Length, Mass, Time, Velocity}; 
        // Check equality operator
        assert_eq!(Force::newton(1.0), Force::millinewton(1000.0));
        assert_ne!(Force::nanonewton(1.0), Force::newton(100.0));

        // Test Add trait for both a quantity and Quantity Base
        assert_eq!(Length::meter(1.0) + Length::centimeter(100.0), Length::meter(2.0));
        assert_eq!(Length::meter(1.0) + QuantityBase::from(Length::centimeter(100.0)), Length::centimeter(2.0e2));
        
        // Test Sub trait for Quantity and Quantity Base
        assert_eq!(Length::meter(1.0) - Velocity::meter_per_second(1.0)*Time::second(0.5), Length::centimeter(50.0));

         // Test Sub trait for Quantity and itself
         assert_eq!(Length::meter(1.0) - Length::centimeter(50.0), Length::centimeter(50.0));


        // Test add assign trait for both a quantity and Quantity Base
        let mut length_add_assign = Length::meter(5.0);
        length_add_assign += QuantityBase::from(Length::centimeter(200.0));
        length_add_assign += Length::kilometer(0.003);
        assert_eq!(length_add_assign, Length::meter(10.0));

        // Test Sub trait for both a quantity and Quantity Base
        let mut length_sub_assign = Length::centimeter(100.0);
        length_sub_assign -= QuantityBase::from(Length::meter(0.5));
        length_sub_assign -= Length::centimeter(50.0);
        assert_eq!(length_sub_assign, Length::megameter(0.0));

        // Test DivAssign and MulAssign traits.
        let mut assign_op_length = Length::meter(5.0);
        assign_op_length *= 10.0;
        assign_op_length /= 5.0;
        assert_eq!(assign_op_length, Length::meter(10.0));

        // Test Mul trait
        assert_eq!(Force::newton(1.0), Mass::kilogram(1.0) * Acceleration::meter_per_second_squared(1.0));
        
        // Test Div trait
        assert_eq!(Length::meter(2.0) / Time::second(1.0), Velocity::centimeter_per_second(200.0));

    }


    #[test]
    #[cfg(any(feature="All", feature="Length"))]
    /// Test unit enumeration -> `Units` conversion
    fn convert_unit_length_to_units()
    {
        use crate::{units::LengthUnit, Units};

        let length = LengthUnit::meter;
        let _units: Units = length.into();
    }
    #[test]
    #[cfg(any(feature="All", all(feature="Length", feature="Time")))]
    /// Test `UnitTypes` -> `Units`
    fn test_convert_unit_type_to_units()
    {       
        
        use crate::{quantities::QuantityBase, units::LengthUnit, Length, Time, UnitTypes};

        let unit_type = UnitTypes::Length;
        let _result = QuantityBase::from(Length::meter(5.0)) / QuantityBase::from(Time::second(1.0));
        assert_eq!(unit_type.to_unit("m").unwrap(),crate::Units::Length(LengthUnit::meter));
        assert_eq!(unit_type.to_unit("meter").unwrap(), crate::Units::Length(LengthUnit::meter));
        assert_ne!(unit_type.to_unit("meter").unwrap(), crate::Units::Length(LengthUnit::kilometer));        
    }
    #[test]
    #[cfg(any(feature="All", feature="Length"))]
    fn test_quantities_to_base_quantity()
    {
        use crate::{quantity::QuantityBase, Length, Quantities};

        let quantity = Quantities::Length(Length::meter(10.0));
        assert_eq!(QuantityBase::from(quantity), QuantityBase::from(Length::centimeter(1e3)));
    }
    #[test]
    #[cfg(any(feature="All", feature="Length"))]
    fn available_units()
    {
        for unit in crate::units::LengthUnit::units()
        {
            println!("{unit}");
        }
    }
    #[test]
    #[cfg(feature="serde")]    
    #[cfg(any(feature="All", feature="Length"))]
    fn serialize_units()
    {
        use serde_json::json;
        println!("{}",json!(crate::Length::new(1.0, crate::units::LengthUnit::centimeter)));
    }
}