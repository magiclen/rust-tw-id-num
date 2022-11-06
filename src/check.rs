use crate::location_values;

#[inline]
fn check_2_to_9(bytes: &[u8], mut sum: u16) -> bool {
    for (i, e) in bytes[2..=8].iter().enumerate() {
        match e {
            b'0'..=b'9' => {
                sum += ((e - b'0') * (7 - i as u8)) as u16;
            }
            _ => return false,
        }
    }

    sum += (bytes[9] - b'0') as u16;

    sum % 10 == 0
}

/**
 * 檢查國民身分證統一編號。
 */
pub fn check_national<S: AsRef<str>>(text: S) -> bool {
    let bytes = text.as_ref().as_bytes();

    if bytes.len() != 10 {
        return false;
    }

    let mut sum = 0u16;

    let location = bytes[0];

    match location {
        b'A'..=b'Z' => {
            sum += location_values::LOCATION_VALUES[(location - b'A') as usize] as u16;
        }
        _ => return false,
    }

    let sex = bytes[1];

    match sex {
        b'1' | b'2' => {
            sum += ((sex - b'0') * 8) as u16;
        }
        _ => return false,
    }

    check_2_to_9(bytes, sum)
}

/**
 * 檢查新式外來人口統一證號。
 */
pub fn check_resident<S: AsRef<str>>(text: S) -> bool {
    let bytes = text.as_ref().as_bytes();

    if bytes.len() != 10 {
        return false;
    }

    let mut sum = 0u16;

    let location = bytes[0];

    match location {
        b'A'..=b'K' | b'M'..=b'Q' | b'T'..=b'X' | b'Z' => {
            sum += location_values::LOCATION_VALUES[(location - b'A') as usize] as u16;
        }
        _ => return false,
    }

    let sex = bytes[1];

    match sex {
        b'8' | b'9' => {
            sum += ((sex - b'0') * 8) as u16;
        }
        _ => return false,
    }

    check_2_to_9(bytes, sum)
}

/**
 * 檢查國民身分證統一編號或是新式外來人口統一證號。
 */
pub fn check<S: AsRef<str>>(text: S) -> bool {
    let bytes = text.as_ref().as_bytes();

    if bytes.len() != 10 {
        return false;
    }

    let mut sum = 0u16;

    let location = bytes[0];

    match location {
        b'A'..=b'K' | b'M'..=b'Q' | b'T'..=b'X' | b'Z' => {
            sum += location_values::LOCATION_VALUES[(location - b'A') as usize] as u16;

            let sex = bytes[1];

            match sex {
                b'1' | b'2' | b'8' | b'9' => {
                    sum += ((sex - b'0') * 8) as u16;
                }
                _ => return false,
            }
        }
        b'L' | b'R' | b'S' | b'Y' => {
            sum += location_values::LOCATION_VALUES[(bytes[0] - b'A') as usize] as u16;

            let sex = bytes[1];

            match sex {
                b'1' | b'2' => {
                    sum += ((sex - b'0') * 8) as u16;
                }
                _ => return false,
            }
        }
        _ => return false,
    }

    check_2_to_9(bytes, sum)
}
