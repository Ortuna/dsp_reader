use std::io::Read;
use std::fs::File;

pub fn from_file(path: &str) -> Vec<f32> {
  let mut amps: Vec<f32> = Vec::new();
  let mut file = File::open(path).unwrap();

  let mut tmp_buffer: [u8; 2] = [0; 2];

  while file.read_exact(&mut tmp_buffer).is_ok() {
    let real = f32::from(tmp_buffer[0]) / (255.0/2.0) - 1.0;
    let imag = f32::from(tmp_buffer[1]) / (255.0/2.0) - 1.0;

    amps.push(f32::sqrt(real * real + imag * imag));
  }

  amps
}
