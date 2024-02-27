fn main() {
    let tx1_hex = "0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031cCfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a45de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef1000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600";
    let foo2 = "01000000017967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff014baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac00000000";
    let tx1_bytes = [
        1, 0, 0, 0, 1, 129, 63, 121, 1, 26, 203, 128, 146, 93, 254, 105, 179, 222, 243, 85, 254,
        145, 75, 209, 217, 106, 63, 95, 113, 191, 131, 3, 198, 169, 137, 199, 209, 0, 0, 0, 0, 107,
        72, 48, 69, 2, 33, 0, 237, 129, 255, 25, 46, 117, 163, 253, 35, 4, 0, 77, 202, 219, 116,
        111, 165, 226, 76, 80, 49, 204, 252, 242, 19, 32, 176, 39, 116, 87, 201, 143, 2, 32, 122,
        152, 109, 149, 92, 110, 12, 179, 93, 68, 106, 137, 211, 245, 97, 0, 244, 215, 246, 120, 1,
        195, 25, 103, 116, 58, 156, 142, 16, 97, 91, 237, 1, 33, 3, 73, 252, 78, 99, 30, 54, 36,
        164, 93, 227, 248, 159, 93, 134, 132, 199, 184, 19, 139, 217, 75, 221, 83, 29, 46, 33, 59,
        240, 22, 178, 120, 175, 239, 255, 255, 240, 42, 19, 94, 241, 0, 0, 0, 0, 25, 118, 169, 20,
        188, 59, 101, 77, 202, 126, 86, 176, 77, 202, 24, 242, 86, 108, 218, 240, 46, 141, 154,
        218, 136, 172, 153, 195, 152, 0, 0, 0, 0, 0, 25, 118, 169, 20, 28, 75, 199, 98, 221, 84,
        35, 227, 50, 22, 103, 2, 203, 117, 244, 13, 247, 159, 234, 18, 136, 172, 25, 67, 6, 0,
    ];

    let foo_bytes = [
        1, 0, 0, 0, 1, 121, 103, 165, 24, 94, 144, 122, 37, 34, 85, 116, 84, 76, 49, 247, 176, 89,
        193, 161, 145, 214, 91, 83, 220, 193, 85, 77, 51, 156, 79, 158, 252, 1, 0, 0, 0, 106, 71,
        48, 68, 2, 32, 106, 46, 177, 107, 123, 146, 5, 29, 15, 163, 140, 19, 62, 103, 104, 78, 208,
        100, 239, 250, 218, 29, 127, 146, 92, 132, 45, 164, 1, 212, 242, 39, 2, 32, 31, 25, 107,
        16, 230, 228, 180, 169, 255, 249, 72, 229, 197, 215, 30, 197, 218, 83, 233, 5, 41, 200,
        219, 209, 34, 191, 242, 177, 210, 29, 200, 169, 1, 33, 3, 155, 123, 205, 8, 36, 185, 169,
        22, 79, 123, 160, 152, 64, 142, 99, 229, 183, 227, 207, 144, 131, 92, 206, 177, 152, 104,
        245, 79, 137, 97, 168, 37, 255, 255, 255, 255, 1, 75, 175, 33, 0, 0, 0, 0, 0, 25, 118, 169,
        20, 219, 77, 17, 65, 208, 4, 139, 30, 209, 88, 57, 208, 183, 164, 196, 136, 205, 54, 139,
        14, 136, 172, 0, 0, 0, 0,
    ];

    let bytes3 = hex::decode("010000000269adb42422fb021f38da0ebe12a8d2a14c0fe484bcb0b7cb365841871f2d5e24000000006a4730440220199a6aa56306cebcdacd1eba26b55eaf6f92eb46eb90d1b7e7724bacbe1d19140220101c0d46e033361c60536b6989efdd6fa692265fcda164676e2f49885871038a0121039ac8bac8f6d916b8a85b458e087e0cd07e6a76a6bfdde9bb766b17086d9a5c8affffffff69adb42422fb021f38da0ebe12a8d2a14c0fe484bcb0b7cb365841871f2d5e24010000006b48304502210084ec4323ed07da4af6462091b4676250c377527330191a3ff3f559a88beae2e2022077251392ec2f52327cb7296be89cc001516e4039badd2ad7bbc950c4c1b6d7cc012103b9b554e25022c2ae549b0c30c18df0a8e0495223f627ae38df0992efb4779475ffffffff0118730100000000001976a9140ce17649c1306c291ca9e587f8793b5b06563cea88ac00000000").unwrap();

    dbg!(&bytes3.len());
    println!("{:?}", BtcTx::from_hex_bytes(&bytes3).unwrap());
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
