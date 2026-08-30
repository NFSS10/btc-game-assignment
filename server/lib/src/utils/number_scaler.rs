use anyhow::Result;

/// ScaledNumber is a type alias for i64, representing a scaled integer version of a floating point number
pub type ScaledNumber = i64;

/// Helper to scale floating point numbers to integers for storage and comparison
pub struct NumberScaler {
    scale_factor: u64,
    scale_factor_f64: f64,

    /// Maximum absolute value that can be scaled without overflowing i64
    max_absolute_value: f64,
}
impl NumberScaler {
    /// Create a new NumberScaler with the given scale factor.
    /// The scale factor must be greater than 0.
    pub fn try_new(factor: u64) -> Result<Self> {
        if factor == 0 {
            return Err(anyhow::anyhow!("Scale factor must be greater than 0"));
        }

        // compute the maximum absolute value that can be scaled without overflowing i64
        let max_abs_value: f64 = (i64::MAX as f64 / factor as f64).abs();

        Ok(Self {
            scale_factor: factor,
            scale_factor_f64: factor as f64,
            max_absolute_value: max_abs_value,
        })
    }

    /// Scale a floating point number to an integer
    pub fn to_scaled_number(&self, value: f64) -> Result<ScaledNumber> {
        if !value.is_finite() {
            return Err(anyhow::anyhow!("Value must be finite"));
        }

        if value.abs() > self.max_absolute_value {
            return Err(anyhow::anyhow!(
                "Value overflows when scaled. Max absolute value: {}",
                self.max_absolute_value
            ));
        }

        let scaled: ScaledNumber = (value * self.scale_factor_f64).round() as i64;
        Ok(scaled)
    }

    /// Converts a scaled integer back to a floating point number
    pub fn from_scaled_number(&self, scaled_value: ScaledNumber) -> Result<f64> {
        let value = scaled_value as f64 / self.scale_factor_f64;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_new_rejects_zero_factor() {
        let result = NumberScaler::try_new(0);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Scale factor must be greater than 0"
        );
    }

    #[test]
    fn try_new_accepts_positive_factor() {
        let result = NumberScaler::try_new(1_000);
        assert!(result.is_ok());
    }

    #[test]
    fn to_scaled_number_scales_and_rounds() {
        let scaler = NumberScaler::try_new(100).unwrap();
        assert_eq!(scaler.to_scaled_number(12.345).unwrap(), 1235); // 1234.5 -> 1235
        assert_eq!(scaler.to_scaled_number(12.344).unwrap(), 1234); // 1234.4 -> 1234
    }

    #[test]
    fn to_scaled_number_handles_negative_values() {
        let scaler = NumberScaler::try_new(100).unwrap();
        assert_eq!(scaler.to_scaled_number(-1.234).unwrap(), -123);
        assert_eq!(scaler.to_scaled_number(-1.235).unwrap(), -124); // half away from zero
    }

    #[test]
    fn to_scaled_number_rejects_positive_overflow() {
        let scaler = NumberScaler::try_new(1_000_000).unwrap();

        // greater than max allowed input for this scale factor
        let too_large = scaler.max_absolute_value + 1.0;
        let result = scaler.to_scaled_number(too_large);

        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("Value overflows when scaled")
        );
    }

    #[test]
    fn from_scaled_number_unscales_correctly() {
        let scaler = NumberScaler::try_new(1_000).unwrap();

        let scaled_value: ScaledNumber = 12_345;
        let value: f64 = scaler.from_scaled_number(scaled_value).unwrap();
        assert!((value - 12.345).abs() < 0.000001);
    }
}
