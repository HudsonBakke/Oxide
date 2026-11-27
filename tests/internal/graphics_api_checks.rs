use super::validate_exposure;

#[test]
fn exposure_accepts_black_and_bright_values_but_rejects_invalid_inputs() {
    for valid in [0.0, 1.0, 8.0, f32::MAX] {
        assert!(validate_exposure(valid).is_ok());
    }
    for invalid in [-1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert!(validate_exposure(invalid).is_err());
    }
}
