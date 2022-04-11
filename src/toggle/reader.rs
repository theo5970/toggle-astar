use flate2::read::DeflateDecoder;

use std::io::Read;

use crate::toggle::{core::ToggleLevel, utils::BitArray};

pub fn parse(text: String) -> ToggleLevel {
    let text_trim = text.trim();
    let comp_input = base64::decode(text_trim).unwrap();

    let mut comp_output: Vec<u8> = Vec::new();
    let mut deflater = DeflateDecoder::new(&comp_input[..]);

    deflater.read_to_end(&mut comp_output).unwrap();

    parse_toggle(&comp_output)
}

pub fn parse_toggle(x: &Vec<u8>) -> ToggleLevel {
    let mut result = ToggleLevel::new();

    const WIDTH_DEF: u8 = 0x01;
    const HEIGHT_DEF: u8 = 0x02;
    const SUBTYPES_DEF: u8 = 0x03;
    const STATES_DEF: u8 = 0x04;
    const MINIMUM_CLICK_DEF: u8 = 0x05;
    const CREATOR_DEF: u8 = 0x06;

    let mut i = 0;

    let mut width = 1;
    let mut height = 1;
    let mut total_buttons = 1;
    i += 1;

    let mut should_escape = false;

    while i < x.len() {
        match x[i] {
            WIDTH_DEF => {
                i += 1;
                width = x[i];
            }
            HEIGHT_DEF => {
                i += 1;
                height = x[i];
            }
            SUBTYPES_DEF => {
                total_buttons = width * height;

                let start = i + 1;
                let end = start + (total_buttons as usize);
                let subtypes_range = &x[start..end];

                i += total_buttons as usize;

                result.subtypes.extend_from_slice(subtypes_range);
            }
            STATES_DEF => {
                let num_bytes: u32 = u32::from((total_buttons - 1) / 8 + 1);

                let start = i + 1;
                let end = start + (num_bytes as usize);
                let states_range = &x[start..end];

                result.states = BitArray::from(states_range);

                i += num_bytes as usize;
            }
            MINIMUM_CLICK_DEF => {
                let start = i + 1;
                let end = start + 4;

                let integer_bytes: [u8; 4] = (&x[start..end]).try_into().unwrap();

                let a = i32::from_le_bytes(integer_bytes);
                result.min_clicks = u32::try_from(a).unwrap();
                i += 4;
            }

            CREATOR_DEF => {
                should_escape = true;
            }

            _ => {}
        }
        i += 1;

        if should_escape {
            break;
        }
    }

    result.width = u32::from(width);
    result.height = u32::from(height);

    result
}
