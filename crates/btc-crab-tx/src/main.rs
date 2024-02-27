fn main() {
    let bytes3 = "010000000269adb42422fb021f38da0ebe12a8d2a14c0fe484bcb0b7cb365841871f2d5e24000000006a4730440220199a6aa56306cebcdacd1eba26b55eaf6f92eb46eb90d1b7e7724bacbe1d19140220101c0d46e033361c60536b6989efdd6fa692265fcda164676e2f49885871038a0121039ac8bac8f6d916b8a85b458e087e0cd07e6a76a6bfdde9bb766b17086d9a5c8affffffff69adb42422fb021f38da0ebe12a8d2a14c0fe484bcb0b7cb365841871f2d5e24010000006b48304502210084ec4323ed07da4af6462091b4676250c377527330191a3ff3f559a88beae2e2022077251392ec2f52327cb7296be89cc001516e4039badd2ad7bbc950c4c1b6d7cc012103b9b554e25022c2ae549b0c30c18df0a8e0495223f627ae38df0992efb4779475ffffffff0118730100000000001976a9140ce17649c1306c291ca9e587f8793b5b06563cea88ac00000000";
    let bytes3 = [
        1, 0, 0, 0, 2, 105, 173, 180, 36, 34, 251, 2, 31, 56, 218, 14, 190, 18, 168, 210, 161, 76,
        15, 228, 132, 188, 176, 183, 203, 54, 88, 65, 135, 31, 45, 94, 36, 0, 0, 0, 0, 106, 71, 48,
        68, 2, 32, 25, 154, 106, 165, 99, 6, 206, 188, 218, 205, 30, 186, 38, 181, 94, 175, 111,
        146, 235, 70, 235, 144, 209, 183, 231, 114, 75, 172, 190, 29, 25, 20, 2, 32, 16, 28, 13,
        70, 224, 51, 54, 28, 96, 83, 107, 105, 137, 239, 221, 111, 166, 146, 38, 95, 205, 161, 100,
        103, 110, 47, 73, 136, 88, 113, 3, 138, 1, 33, 3, 154, 200, 186, 200, 246, 217, 22, 184,
        168, 91, 69, 142, 8, 126, 12, 208, 126, 106, 118, 166, 191, 221, 233, 187, 118, 107, 23, 8,
        109, 154, 92, 138, 255, 255, 255, 255, 105, 173, 180, 36, 34, 251, 2, 31, 56, 218, 14, 190,
        18, 168, 210, 161, 76, 15, 228, 132, 188, 176, 183, 203, 54, 88, 65, 135, 31, 45, 94, 36,
        1, 0, 0, 0, 107, 72, 48, 69, 2, 33, 0, 132, 236, 67, 35, 237, 7, 218, 74, 246, 70, 32, 145,
        180, 103, 98, 80, 195, 119, 82, 115, 48, 25, 26, 63, 243, 245, 89, 168, 139, 234, 226, 226,
        2, 32, 119, 37, 19, 146, 236, 47, 82, 50, 124, 183, 41, 107, 232, 156, 192, 1, 81, 110, 64,
        57, 186, 221, 42, 215, 187, 201, 80, 196, 193, 182, 215, 204, 1, 33, 3, 185, 181, 84, 226,
        80, 34, 194, 174, 84, 155, 12, 48, 193, 141, 240, 168, 224, 73, 82, 35, 246, 39, 174, 56,
        223, 9, 146, 239, 180, 119, 148, 117, 255, 255, 255, 255, 1, 24, 115, 1, 0, 0, 0, 0, 0, 25,
        118, 169, 20, 12, 225, 118, 73, 193, 48, 108, 41, 28, 169, 229, 135, 248, 121, 59, 91, 6,
        86, 60, 234, 136, 172, 0, 0, 0, 0,
    ];

    dbg!(&bytes3.len());
    println!("{:#?}", BtcTx::from_hex_bytes(&bytes3).unwrap());
}

mod finite_fields;
pub use finite_fields::*;

mod errors;
pub use errors::*;

mod elliptic_curve;
pub use elliptic_curve::*;

mod sec;
pub use sec::*;

mod txs;
use sha2::{Digest, Sha256};
pub use txs::*;

mod varints;
pub use varints::*;

mod transactions;
pub use transactions::*;

mod version;
pub use version::*;

mod constants;
pub use constants::*;
