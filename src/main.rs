mod sdr;

// const PRELUDE: &str = "111000111000111000111000";

fn main() {
    let mut data:Vec<f32> = sdr::from_file("./assets/data.8")
        .into_iter()
        .collect();

    data = sdr::decimate(&data, 5);
    data = sdr::convolve(&data, 100);

    let output = sdr::parse_message(&data, 0.6);
}
