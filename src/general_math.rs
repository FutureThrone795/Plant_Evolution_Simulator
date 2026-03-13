#[inline]
pub fn sigmoid(x: f32) -> f32 {
    return 1.0 - 1.0 / (1.0 + x.exp());
}

pub fn softplus(x: f32) -> f32 {
    return (1.0 + x.exp()).ln();
}