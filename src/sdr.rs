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

pub fn decimate(data: &Vec<f32>, scale: usize) -> Vec<f32> {
  let mut output = Vec::new();

  for i in (0..data.len() - scale).step_by(scale) {
    let sum: f32 = data[i..i+scale].iter().sum();

    output.push(sum / scale as f32);
  }

  output
}

pub fn convolve(data: &Vec<f32>, window_size: i32) -> Vec<f32> {
  (0..data.len()).map(|i| {
    data
      .iter()
      .skip(i)
      .zip(0..window_size)
      .map(|(x, _)| { x * 0.01 })
      .sum()
  }).collect()
}

pub fn parse_message(data: &Vec<f32>, min_level: f32) -> String {
  let mut output = String::new();
  let mut count = 0;
  let mut temp:i32 = 0;

  for &d in data {
    if d > min_level {
      count += 1;
    } else {
      if count > 50 {
        if count > 300 {
          if temp != 0 {
            println!("{:b} {0}", (((temp >> 13) & 0b1111111111) - 500) / 10);
          }
          temp = 0;
        } else if count > 200 {
          temp = (temp << 1) | 0x01;
        } else if count > 100 {
          temp = temp << 1;
        }

//        output.push_str(&format!("{} ", &count.to_string()).to_string());
      }

      count = 0;
    }
  }

  output
}
