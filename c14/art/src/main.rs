// Another crate that uses the art library without `pub use`
// use art::kinds::PrimaryColor;
// use art::utils;
//
// fn main() {
//     let red = PrimaryColor::Red;
//     let yellow = PrimaryColor::Yellow;
//     println!("{:?}", utils::mix(red, yellow))
// }

use art::mix;
use art::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    println!("{:?}", mix(red, yellow))
}
