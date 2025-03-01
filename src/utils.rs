/// Round a number to 4 significant figures
#[inline(always)]
pub fn round_to_4_sig_figs<T: Into<f32>>(value: T) -> f32 {
    let value = value.into();
    if value == 0.0 {
        return 0.0;
    }
    let magnitude = value.abs().log10().floor();
    let scale = 10f32.powi(3 - magnitude as i32);
    (value * scale).round() / scale
}
