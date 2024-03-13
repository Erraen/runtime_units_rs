//! Angular acceleration (base UnitDefinition radian per second squared, s⁻²).
use crate::quantity;
quantity! {
    /// Angular acceleration (base UnitDefinition radian per second squared, s⁻²).
    quantity: AngularAcceleration; "angular acceleration";
    /// Dimension of angular acceleration, T⁻² (base UnitDefinition radian per second squared, s⁻²).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::AngleKind);
    units {
        /// Derived UnitDefinition of angular acceleration.
        @radian_per_second_squared: 1.0; "rad/s²", "radian per second squared",
            "radians per second squared";
        @degree_per_second_squared: 1.745_329_251_994_329_5_E-2; "°/s²",
            "degree per second squared", "degrees per second squared";
    }
}

#[cfg(test)]
mod tests {
    use crate::{units::{AngleUnit, AngularAccelerationUnit, TimeUnit}, units_base::UnitDefinition};
    
    #[test]
    fn check_dimension() {
        assert_eq!(AngularAccelerationUnit::unit_base(), AngleUnit::unit_base() / TimeUnit::unit_base().powi(2));
    }

    #[test]
    fn check_units() {
        test_unit(AngularAccelerationUnit::radian_per_second_squared, TimeUnit::second, AngleUnit::radian);
        test_unit(AngularAccelerationUnit::degree_per_second_squared, TimeUnit::second, AngleUnit::degree);

    }
    fn test_unit(value: AngularAccelerationUnit, time: TimeUnit, angle: AngleUnit) {
        assert_eq!(Into::<UnitDefinition>::into(value), Into::<UnitDefinition>::into(angle) / Into::<UnitDefinition>::into(time).powi(2));
    }
}
