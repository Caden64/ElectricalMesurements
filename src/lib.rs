mod units;

use units::ohm::Ohm;
use units::amp::Amp;
use units::volt::Volt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ohms_eq_ohms() {
        assert_eq!(Ohm::new(5.0).value, 5.0)
    }

    #[test]
    fn ohms_add() {
        assert_eq!(Ohm::new(5.0) + Ohm::new(3.0), Ohm::new(8.0))
    }

    #[test]
    fn ohms_add_neg() {
        assert_eq!(Ohm::new(-5.0) + Ohm::new(3.0), Ohm::new(-2.0))
    }

    #[test]
    fn ohms_add_assign() {
        let mut test_value = Ohm::new(5.0);
        test_value += Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(8.0))
    }

    #[test]
    fn ohms_add_assign_neg() {
        let mut test_value = Ohm::new(-5.0);
        test_value += Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(-2.0))
    }

    #[test]
    fn ohms_sub() {
        assert_eq!(Ohm::new(5.0) - Ohm::new(3.0), Ohm::new(2.0))
    }

    #[test]
    fn ohms_sub_neg() {
        assert_eq!(Ohm::new(-5.0) - Ohm::new(3.0), Ohm::new(-8.0))
    }

    #[test]
    fn ohms_sub_assign() {
        let mut test_value = Ohm::new(5.0);
        test_value -= Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(2.0))
    }

    #[test]
    fn ohms_sub_assign_neg() {
        let mut test_value = Ohm::new(-5.0);
        test_value -= Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(-8.0))
    }

    #[test]
    fn ohms_mul() {
        assert_eq!(Ohm::new(5.0) * Ohm::new(3.0), Ohm::new(15.0))
    }

    #[test]
    fn ohms_mul_neg() {
        assert_eq!(Ohm::new(-5.0) * Ohm::new(3.0), Ohm::new(-15.0))
    }

    #[test]
    fn ohms_mul_assign() {
        let mut test_value = Ohm::new(5.0);
        test_value *= Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(15.0))
    }

    #[test]
    fn ohms_mul_assign_neg() {
        let mut test_value = Ohm::new(-5.0);
        test_value *= Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(-15.0))
    }

    #[test]
    fn ohms_div() {
        assert_eq!(Ohm::new(5.0) / Ohm::new(3.0), Ohm::new(5.0 / 3.0))
    }

    #[test]
    fn ohms_div_neg() {
        assert_eq!(Ohm::new(-5.0) / Ohm::new(3.0), Ohm::new(-5.0 / 3.0))
    }

    #[test]
    fn ohms_div_assign() {
        let mut test_value = Ohm::new(5.0);
        test_value /= Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(5.0 / 3.0))
    }

    #[test]
    fn ohms_div_assign_neg() {
        let mut test_value = Ohm::new(-5.0);
        test_value /= Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(-5.0 / 3.0))
    }

    #[test]
    fn ohms_rem() {
        assert_eq!(Ohm::new(5.0) % Ohm::new(3.0), Ohm::new(2.0))
    }

    #[test]
    fn ohms_rem_neg() {
        assert_eq!(Ohm::new(-5.0) % Ohm::new(3.0), Ohm::new(-2.0))
    }

    #[test]
    fn ohms_rem_assign() {
        let mut test_value = Ohm::new(5.0);
        test_value %= Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(2.0))
    }

    #[test]
    fn ohms_rem_assign_neg() {
        let mut test_value = Ohm::new(-5.0);
        test_value %= Ohm::new(3.0);
        assert_eq!(test_value, Ohm::new(-2.0))
    }

    #[test]
    fn amps_eq_amps() {
        assert_eq!(Amp::new(5.0).value, 5.0)
    }

    #[test]
    fn amps_add() {
        assert_eq!(Amp::new(5.0) + Amp::new(3.0), Amp::new(8.0))
    }

    #[test]
    fn amps_add_neg() {
        assert_eq!(Amp::new(-5.0) + Amp::new(3.0), Amp::new(-2.0))
    }

    #[test]
    fn amps_add_assign() {
        let mut test_value = Amp::new(5.0);
        test_value += Amp::new(3.0);
        assert_eq!(test_value, Amp::new(8.0))
    }

    #[test]
    fn amps_add_assign_neg() {
        let mut test_value = Amp::new(-5.0);
        test_value += Amp::new(3.0);
        assert_eq!(test_value, Amp::new(-2.0))
    }

    #[test]
    fn amps_sub() {
        assert_eq!(Amp::new(5.0) - Amp::new(3.0), Amp::new(2.0))
    }

    #[test]
    fn amps_sub_neg() {
        assert_eq!(Amp::new(-5.0) - Amp::new(3.0), Amp::new(-8.0))
    }

    #[test]
    fn amps_sub_assign() {
        let mut test_value = Amp::new(5.0);
        test_value -= Amp::new(3.0);
        assert_eq!(test_value, Amp::new(2.0))
    }

    #[test]
    fn amps_sub_assign_neg() {
        let mut test_value = Amp::new(-5.0);
        test_value -= Amp::new(3.0);
        assert_eq!(test_value, Amp::new(-8.0))
    }

    #[test]
    fn amps_mul() {
        assert_eq!(Amp::new(5.0) * Amp::new(3.0), Amp::new(15.0))
    }

    #[test]
    fn amps_mul_neg() {
        assert_eq!(Amp::new(-5.0) * Amp::new(3.0), Amp::new(-15.0))
    }

    #[test]
    fn amps_mul_assign() {
        let mut test_value = Amp::new(5.0);
        test_value *= Amp::new(3.0);
        assert_eq!(test_value, Amp::new(15.0))
    }

    #[test]
    fn amps_mul_assign_neg() {
        let mut test_value = Amp::new(-5.0);
        test_value *= Amp::new(3.0);
        assert_eq!(test_value, Amp::new(-15.0))
    }

    #[test]
    fn amps_div() {
        assert_eq!(Amp::new(5.0) / Amp::new(3.0), Amp::new(5.0 / 3.0))
    }

    #[test]
    fn amps_div_neg() {
        assert_eq!(Amp::new(-5.0) / Amp::new(3.0), Amp::new(-5.0 / 3.0))
    }

    #[test]
    fn amps_div_assign() {
        let mut test_value = Amp::new(5.0);
        test_value /= Amp::new(3.0);
        assert_eq!(test_value, Amp::new(5.0 / 3.0))
    }

    #[test]
    fn amps_div_assign_neg() {
        let mut test_value = Amp::new(-5.0);
        test_value /= Amp::new(3.0);
        assert_eq!(test_value, Amp::new(-5.0 / 3.0))
    }

    #[test]
    fn amps_rem() {
        assert_eq!(Amp::new(5.0) % Amp::new(3.0), Amp::new(2.0))
    }

    #[test]
    fn amps_rem_neg() {
        assert_eq!(Amp::new(-5.0) % Amp::new(3.0), Amp::new(-2.0))
    }

    #[test]
    fn amps_rem_assign() {
        let mut test_value = Amp::new(5.0);
        test_value %= Amp::new(3.0);
        assert_eq!(test_value, Amp::new(2.0))
    }

    #[test]
    fn amps_rem_assign_neg() {
        let mut test_value = Amp::new(-5.0);
        test_value %= Amp::new(3.0);
        assert_eq!(test_value, Amp::new(-2.0))
    }

    #[test]
    fn volts_eq_volts() {
        assert_eq!(Volt::new(5.0).value, 5.0)
    }

    #[test]
    fn volts_add() {
        assert_eq!(Volt::new(5.0) + Volt::new(3.0), Volt::new(8.0))
    }

    #[test]
    fn volts_add_neg() {
        assert_eq!(Volt::new(-5.0) + Volt::new(3.0), Volt::new(-2.0))
    }

    #[test]
    fn volts_add_assign() {
        let mut test_value = Volt::new(5.0);
        test_value += Volt::new(3.0);
        assert_eq!(test_value, Volt::new(8.0))
    }

    #[test]
    fn volts_add_assign_neg() {
        let mut test_value = Volt::new(-5.0);
        test_value += Volt::new(3.0);
        assert_eq!(test_value, Volt::new(-2.0))
    }

    #[test]
    fn volts_sub() {
        assert_eq!(Volt::new(5.0) - Volt::new(3.0), Volt::new(2.0))
    }

    #[test]
    fn volts_sub_neg() {
        assert_eq!(Volt::new(-5.0) - Volt::new(3.0), Volt::new(-8.0))
    }

    #[test]
    fn volts_sub_assign() {
        let mut test_value = Volt::new(5.0);
        test_value -= Volt::new(3.0);
        assert_eq!(test_value, Volt::new(2.0))
    }

    #[test]
    fn volts_sub_assign_neg() {
        let mut test_value = Volt::new(-5.0);
        test_value -= Volt::new(3.0);
        assert_eq!(test_value, Volt::new(-8.0))
    }

    #[test]
    fn volts_mul() {
        assert_eq!(Volt::new(5.0) * Volt::new(3.0), Volt::new(15.0))
    }

    #[test]
    fn volts_mul_neg() {
        assert_eq!(Volt::new(-5.0) * Volt::new(3.0), Volt::new(-15.0))
    }

    #[test]
    fn volts_mul_assign() {
        let mut test_value = Volt::new(5.0);
        test_value *= Volt::new(3.0);
        assert_eq!(test_value, Volt::new(15.0))
    }

    #[test]
    fn volts_mul_assign_neg() {
        let mut test_value = Volt::new(-5.0);
        test_value *= Volt::new(3.0);
        assert_eq!(test_value, Volt::new(-15.0))
    }

    #[test]
    fn volts_div() {
        assert_eq!(Volt::new(5.0) / Volt::new(3.0), Volt::new(5.0 / 3.0))
    }

    #[test]
    fn volts_div_neg() {
        assert_eq!(Volt::new(-5.0) / Volt::new(3.0), Volt::new(-5.0 / 3.0))
    }

    #[test]
    fn volts_div_assign() {
        let mut test_value = Volt::new(5.0);
        test_value /= Volt::new(3.0);
        assert_eq!(test_value, Volt::new(5.0 / 3.0))
    }

    #[test]
    fn volts_div_assign_neg() {
        let mut test_value = Volt::new(-5.0);
        test_value /= Volt::new(3.0);
        assert_eq!(test_value, Volt::new(-5.0 / 3.0))
    }

    #[test]
    fn volts_rem() {
        assert_eq!(Volt::new(5.0) % Volt::new(3.0), Volt::new(2.0))
    }

    #[test]
    fn volts_rem_neg() {
        assert_eq!(Volt::new(-5.0) % Volt::new(3.0), Volt::new(-2.0))
    }

    #[test]
    fn volts_rem_assign() {
        let mut test_value = Volt::new(5.0);
        test_value %= Volt::new(3.0);
        assert_eq!(test_value, Volt::new(2.0))
    }

    #[test]
    fn volts_rem_assign_neg() {
        let mut test_value = Volt::new(-5.0);
        test_value %= Volt::new(3.0);
        assert_eq!(test_value, Volt::new(-2.0))
    }

    #[test]
    fn amp_eq_volt_div_ohm() {
        assert_eq!(Amp::new(5.0), Volt::new(5.0) / Ohm::new(1.0))
    }

    #[test]
    fn volt_eq_amp_mul_ohm() {
        assert_eq!(Volt::new(5.0), Amp::new(5.0) * Ohm::new(1.0))
    }

    #[test]
    fn ohm_eq_volt_div_amp() {
        assert_eq!(Ohm::new(5.0), Volt::new(5.0) / Amp::new(1.0))
    }
}
