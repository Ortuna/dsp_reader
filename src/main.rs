mod data_parse;

fn main() {
    let data:Vec<f32> = data_parse::from_file("./assets/data.8")
        .into_iter()
        .filter(|&amp| { amp > 0.6 })
        .collect();

    for i in 0..data.len() {
        print!("{} ", data[i]);
    }
}
