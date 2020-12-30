use crypto::digest::Digest;
use crypto::md5::Md5;
use rdcl_aoc_helpers::args::get_args;

const PW_LENGTH: usize = 8;
const MD5_LENGTH: usize = 16;

fn main() {
    let args = get_args(&["<door id>"], 1);
    let door_id = &args[1];

    let (first_password, second_password) = find_passwords(door_id);
    println!("The first password for {} is {}.", door_id, first_password);
    println!(
        "The second password for {} is {}.",
        door_id, second_password
    );
}

fn find_passwords(door_id: &str) -> (String, String) {
    let mut hasher = Md5::new();
    let key = door_id.as_bytes();
    let mut first_password = String::new();
    let mut second_password = ['_'; PW_LENGTH];
    let mut second_password_len = 0;
    let mut idx: usize = 0;

    while first_password.len() < PW_LENGTH || second_password_len < PW_LENGTH {
        hasher.input(key);
        hasher.input(idx.to_string().as_bytes());
        idx += 1;

        let mut output = [0; MD5_LENGTH];
        hasher.result(&mut output);

        if starts_with_n_zeroes(output, 5) {
            if first_password.len() < PW_LENGTH {
                first_password.push(as_hex_char(output, 5));
            }

            if second_password_len < PW_LENGTH {
                let pos = output[2] as usize;
                if pos < PW_LENGTH && second_password[pos] == '_' {
                    second_password_len += 1;
                    second_password[pos] = as_hex_char(output, 6);
                }
            }
        }

        hasher.reset();
    }

    let second_password = second_password.iter().fold(String::new(), |mut acc, &ch| {
        acc.push(ch);
        acc
    });
    (first_password, second_password)
}

fn starts_with_n_zeroes(output: [u8; MD5_LENGTH], n: usize) -> bool {
    output.iter().take(n / 2).all(|o| *o == 0) && ((n % 2 == 0) || (output[n / 2] >> 4) == 0)
}

fn as_hex_char(output: [u8; MD5_LENGTH], idx: usize) -> char {
    let nr = if idx % 2 == 0 {
        output[idx / 2] >> 4
    } else {
        output[idx / 2] % 16
    };
    (nr + if nr < 10 { b'0' } else { b'a' - 10 }) as char
}

// FIXME: This test takes way too long, unless you run it with --release.
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_find_passwords() {
//         assert_eq!(
//             find_passwords("abc"),
//             ("18f47a30".to_string(), "05ace8e3".to_string())
//         );
//     }
// }
