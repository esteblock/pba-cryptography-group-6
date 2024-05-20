use hex::decode;
use hex::encode;
use std::collections::HashMap;

fn main() {
    let message_0 = "160111433b00035f536110435a380402561240555c526e1c0e431300091e4f04451d1d490d1c49010d000a0a4510111100000d434202081f0755034f13031600030d0204040e";
    let message_1 = "050602061d07035f4e3553501400004c1e4f1f01451359540c5804110c1c47560a1415491b06454f0e45040816431b144f0f4900450d1501094c1b16550f0b4e151e03031b450b4e020c1a124f020a0a4d09071f16003a0e5011114501494e16551049021011114c291236520108541801174b03411e1d124554284e141a0a1804045241190d543c00075453020a044e134f540a174f1d080444084e01491a090b0a1b4103570740";
    let message_2 = "000000000000001a49320017071704185941034504524b1b1d40500a0352441f021b0708034e4d0008451c40450101064f071d1000100201015003061b0b444c00020b1a16470a4e051a4e114f1f410e08040554154f064f410c1c00180c0010000b0f5216060605165515520e09560e00064514411304094c1d0c411507001a1b45064f570b11480d001d4c134f060047541b185c";
    let message_3 = "0b07540c1d0d0b4800354f501d131309594150010011481a1b5f11090c0845124516121d0e0c411c030c45150a16541c0a0b0d43540c411b0956124f0609075513051816590026004c061c014502410d024506150545541c450110521a111758001d0607450d11091d00121d4f0541190b45491e02171a0d49020a534f";
    let message_4 = "031a5410000a075f5438001210110a011c5350080a0048540e431445081d521345111c041f0245174a0006040002001b01094914490f0d53014e570214021d00160d151c57420a0d03040b4550020e1e1f001d071a56110359420041000c0b06000507164506151f104514521b02000b0145411e05521c1852100a52411a0054180a1e49140c54071d5511560201491b0944111a011b14090c0e41";
    let message_5 = "0b4916060808001a542e0002101309050345500b00050d04005e030c071b4c1f111b161a4f01500a08490b0b451604520d0b1d1445060f531c48124f1305014c051f4c001100262d38490f0b4450061800004e001b451b1d594e45411d014e004801491b0b0602050d41041e0a4d53000d0c411c41111c184e130a0015014f03000c1148571d1c011c55034f12030d4e0b45150c5c";
    let message_6 = "011b0d131b060d4f5233451e161b001f59411c090a0548104f431f0b48115505111d17000e02000a1e430d0d0b04115e4f190017480c14074855040a071f4448001a050110001b014c1a07024e5014094d0a1c541052110e54074541100601014e101a5c";
    let message_7 = "0c06004316061b48002a4509065e45221654501c0a075f540c42190b165c";
    let message_8 = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

    let messages_hex = vec![
        message_7,
        message_0,
        message_6,
        message_3,
        message_2,
        message_5,
        message_4,
        message_1,
        message_8,
    ];

    let message_1_bytes = hex::decode(message_1).unwrap();
    let message_0_bytes = hex::decode(message_0).unwrap();
    let message_2_bytes = hex::decode(message_2).unwrap();
    let message_3_bytes = hex::decode(message_3).unwrap();
    let message_4_bytes = hex::decode(message_4).unwrap();
    let message_5_bytes = hex::decode(message_5).unwrap();
    let message_6_bytes = hex::decode(message_6).unwrap();
    let message_7_bytes = hex::decode(message_7).unwrap();
    let message_8_bytes = hex::decode(message_8).unwrap();

    let messages = vec![
        message_7_bytes,
        message_0_bytes,
        message_6_bytes,
        message_3_bytes,
        message_2_bytes,
        message_5_bytes,
        message_4_bytes,
        message_1_bytes,
        message_8_bytes,
    ];


    for i in 0..9 {
        let hello_result = print_xor(messages_hex[i], 
        &hex::encode("Bitcoin: a purely peer-to-peer version of electronic cash"));
    }

    /* 
    for i in 0..8 {
        for j in 0..8 {
            if i != j {
                let result = fixed_xor_bytes(&messages[i], &messages[j]);
                println!("{} and {} xored = {}", i, j, result);
            }
        }
    }*/

    
}

/* 
pub fn test(){
    println!("Print XoR");
    println!("Message 0 & 1 :");
    print_xor(
        "160111433b00035f536110435a380402561240555c",
        "050602061d07035f4e3553501400004c1e4f1f0145",
    );

    println!("Message 0 & 2 :");
    print_xor(
        "160111433b00035f536110435a380402561240555c",
        "000000000000001a49320017071704185941034504",
    );

    println!("______________________________________________________________________________________________");
    println!("Fixed XoR");

    let xor = fixed_xor(
        "160111433b00035f536110435a380402561240555c",
        "050602061d07035f4e3553501400004c1e4f1f0145",
    );
    println!("Message 0 & 1 {}", xor);

    let xor = fixed_xor(
        "160111433b00035f536110435a380402561240555c",
        "000000000000001a49320017071704185941034504",
    );
    println!("Message 0 & 2 {}", xor);

    let xor = fixed_xor(
        "160111433b00035f536110435a380402561240555c",
        "0b07540c1d0d0b4800354f501d1313095941500100",
    );
    println!("Message 0 & 3 {}", xor);

    let xor = fixed_xor(
        "050602061d07035f4e3553501400004c1e4f1f0145",
        "000000000000001a49320017071704185941034504",
    );
    println!("Message 1 & 2 {}", xor);

    let xor = fixed_xor(
        "050602061d07035f4e3553501400004c1e4f1f0145",
        "0b07540c1d0d0b4800354f501d1313095941500100",
    );
    println!("Message 1 & 3 {}", xor);
}
*/

pub fn print_xor_bytes(bytes1: &Vec<u8>, bytes2: &Vec<u8>) {
    let xor = fixed_xor_bytes(bytes1, bytes2);
    for i in 0..xor.len() {
        print!("{}", xor[i] as char);
    }
    println!("");
}

pub fn print_xor(hex1: &str, hex2: &str) {
    let xor = fixed_xor(hex1, hex2);
    for i in 0..xor.len() {
        print!("{}", xor[i] as char);
    }
    println!("");
}

pub fn fixed_xor(hex1: &str, hex2: &str) -> Vec<u8> {
    let bytes1 = decode(hex1).unwrap();
    let bytes2 = decode(hex2).unwrap();

    let xor_bytes: Vec<u8> = bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(&b1, &b2)| b1 ^ b2)
        .collect();

    xor_bytes
}

pub fn fixed_xor_bytes(bytes1: &Vec<u8>, bytes2: &Vec<u8>) -> Vec<u8> {
    let xor_bytes: Vec<u8> = bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(&b1, &b2)| b1 ^ b2)
        .collect();

    xor_bytes
}
