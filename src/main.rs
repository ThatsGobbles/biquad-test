use biquad::ToHertz;
use biquad::Coefficients;
use biquad::Type;
use biquad::Q_BUTTERWORTH_F32;

fn main() {
    // Cutoff frequency
    let f0 = 10.hz();

    // Sampling frequency
    let fs = 1.khz();

    // Create coefficients
    let coeffs = Coefficients::<f32>::from_params(Type::LowPass, fs, f0, Q_BUTTERWORTH_F32);
}
