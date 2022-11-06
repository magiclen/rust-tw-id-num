use rand::Rng;

use crate::location_values;

/// 性別。
///
/// * 國民身分證統一編號：男性為**1**；女性為**2**。
/// * 新式外來人口統一證號：男性為**8**；女性為**9**。
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Sex {
    /// 男性
    Male,
    /// 女性
    Female,
}

fn generate_2_to_9_and_create_string<R: Rng>(mut buffer: Vec<u8>, mut rng: R) -> String {
    for e in buffer[2..=8].iter_mut() {
        *e = rng.gen_range(b'0'..=b'9');
    }

    let mut sum = location_values::LOCATION_VALUES[(buffer[0] - b'A') as usize] as u16;

    for (i, e) in buffer[1..=8].iter().enumerate() {
        sum += ((e - b'0') * (8 - i as u8)) as u16;
    }

    buffer[9] = ((10 - sum % 10) % 10) as u8 + b'0';

    unsafe { String::from_utf8_unchecked(buffer) }
}

#[inline]
fn generate_continue_national<R: Rng>(mut buffer: Vec<u8>, mut rng: R) -> String {
    buffer[0] = rng.gen_range(b'A'..=b'Z');

    generate_2_to_9_and_create_string(buffer, rng)
}

#[inline]
fn generate_continue_resident<R: Rng>(mut buffer: Vec<u8>, mut rng: R) -> String {
    buffer[0] = {
        let mut rnd = rng.gen_range(b'A'..(b'A' + 22));

        if rnd >= b'A' + 21 {
            rnd += 4;
        } else if rnd >= b'A' + 16 {
            rnd += 3;
        } else if rnd >= b'A' + 11 {
            rnd += 1;
        }

        rnd
    };

    generate_2_to_9_and_create_string(buffer, rng)
}

/// 產生國民身分證統一編號。
pub fn generate_national_with_rng<R: Rng>(sex: Option<Sex>, mut rng: R) -> String {
    let mut buffer = Vec::with_capacity(10);

    #[allow(clippy::uninit_vec)]
    unsafe {
        buffer.set_len(10);
    }

    buffer[1] = match sex {
        Some(Sex::Male) => b'1',
        Some(Sex::Female) => b'2',
        None => rng.gen_range(b'1'..=b'2'),
    };

    generate_continue_national(buffer, rng)
}

/// 產生國民身分證統一編號。
#[inline]
pub fn generate_national(sex: Option<Sex>) -> String {
    generate_national_with_rng(sex, rand::thread_rng())
}

/// 產生新式外來人口統一證號。
pub fn generate_resident_with_rng<R: Rng>(sex: Option<Sex>, mut rng: R) -> String {
    let mut buffer = Vec::with_capacity(10);

    #[allow(clippy::uninit_vec)]
    unsafe {
        buffer.set_len(10);
    }

    buffer[1] = match sex {
        Some(Sex::Male) => b'8',
        Some(Sex::Female) => b'9',
        None => rng.gen_range(b'8'..=b'9'),
    };

    generate_continue_resident(buffer, rng)
}

/// 產生新式外來人口統一證號。
#[inline]
pub fn generate_resident(sex: Option<Sex>) -> String {
    generate_resident_with_rng(sex, rand::thread_rng())
}

/// 產生國民身分證統一編號或是新式外來人口統一證號。
pub fn generate_with_rng<R: Rng>(sex: Option<Sex>, mut rng: R) -> String {
    let mut buffer = Vec::with_capacity(10);

    #[allow(clippy::uninit_vec)]
    unsafe {
        buffer.set_len(10);
    }

    buffer[1] = match sex {
        Some(Sex::Male) => {
            let rnd = rng.gen_range(1u8..=2);

            if rnd == 1 {
                b'1'
            } else {
                b'8'
            }
        }
        Some(Sex::Female) => {
            let rnd = rng.gen_range(1u8..=2);

            if rnd == 1 {
                b'2'
            } else {
                b'9'
            }
        }
        None => {
            let rnd = rng.gen_range(1u8..=4);

            match rnd {
                1 => b'1',
                2 => b'2',
                3 => b'8',
                4 => b'9',
                _ => unreachable!(),
            }
        }
    };

    if buffer[1] <= b'2' {
        generate_continue_national(buffer, rng)
    } else {
        generate_continue_resident(buffer, rng)
    }
}

/// 產生國民身分證統一編號或是新式外來人口統一證號。
#[inline]
pub fn generate(sex: Option<Sex>) -> String {
    generate_with_rng(sex, rand::thread_rng())
}
