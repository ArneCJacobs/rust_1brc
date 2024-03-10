use std::collections::BTreeMap;

use memmap2::Mmap;

#[derive(Debug)]
struct Measurement {
    min: i32,
    max: i32,
    count: i64,
    sum: i32,
}

#[inline]
pub fn one_brc(file: &str) -> String {
    let mut measurements: BTreeMap<&str, Measurement> = BTreeMap::new();
    let file = std::fs::File::open(file).unwrap();
    let mmap = unsafe { Mmap::map(&file).expect("Whoopsy doodles") };
    let mut mmap_index = 0;
    let mmap_len = mmap.len();

    while mmap_index < mmap.len() {
        let name_start_index = mmap_index;
        while mmap[mmap_index] != b';' {
            mmap_index += 1;
        }
        let station = std::str::from_utf8(&mmap[name_start_index..mmap_index]).unwrap();
        mmap_index += 1;

        let value_start_index = mmap_index;

        while mmap[mmap_index] != b'\n' {
            mmap_index += 1;
        }
        let value = if value_start_index + 8 < mmap_len {
            let value_array: &[u8; 8] = mmap[value_start_index..value_start_index+8].try_into().unwrap();
            let value_i64 = i64::from_le_bytes(*value_array);
            parse_data_point(value_i64) as i32
        } else {
            let value_str = std::str::from_utf8(&mmap[value_start_index..mmap_index]).unwrap();
            (value_str.parse::<f64>().unwrap() * 10.0) as i32
        };


        measurements
            .entry(station)
            .and_modify({
                |measurement| {
                    measurement.min = measurement.min.min(value);
                    measurement.max = measurement.max.max(value);
                    measurement.count += 1;
                    measurement.sum += value;
                }
            })
            .or_insert(Measurement {
                min: value,
                max: value,
                count: 1,
                sum: value,
            });
        mmap_index += 1;
    }

    let mut measurements_list: Vec<_> = measurements.iter().collect();
    measurements_list.sort_unstable_by_key(|(station, _)| *station);
    return measurements_list
        .iter()
        .map(|(station, measurement)| {
            format!(
                "{}={:.1}/{:.1}/{:.1}\n",
                station,
                (measurement.min as f64 / 10.0),
                round((measurement.sum as f64 / 10.0) / measurement.count as f64),
                (measurement.max as f64 / 10.0),
            )
        })
        .fold(String::new(), |acc, x| acc + &x);
}

const DOT_DETECTOR: i64 = 0x10101000;
const ASCII_TO_DIGIT_MASK: i64 = 0x0F000F0F00;
const MAGIC_MULTIPLIER: i64 = 100 * 0x1000000 + 10 * 0x10000 + 1;

// adapted from :
// https://questdb.io/blog/1brc-merykittys-magic-swar/
// https://github.com/gunnarmorling/1brc/blob/dfec2cdbe6a0334cff054f333ec4b4d9e4d775cf/src/main/java/dev/morling/onebrc/CalculateAverage_merykitty.java#L165-L195
#[inline]
fn parse_data_point(data: i64) -> i64 {
    let negated_input = !data;
    let broadcast_sign = (negated_input << 59) >> 63;
    //     long maskToRemoveSign = ~(broadcastSign & 0xFF);
    let mask_to_remove_sign = !(broadcast_sign & 0xFF);
    //     long withSignRemoved = inputData & maskToRemoveSign;
    let with_sign_removed = data & mask_to_remove_sign;
    //     int dotPos = Long.numberOfTrailingZeros(negatedInput & DOT_DETECTOR);
    let dot_pos = (negated_input & DOT_DETECTOR).trailing_zeros();
    //     long alignedToTemplate = withSignRemoved << (28 - dotPos);
    let aligned_to_template = with_sign_removed << (28 - dot_pos);
    //     long digits = alignedToTemplate & ASCII_TO_DIGIT_MASK;
    let digits = aligned_to_template & ASCII_TO_DIGIT_MASK;
    //     long absValue = ((digits * MAGIC_MULTIPLIER) >>> 32) & 0x3FF;
    let abs_value = ((digits.wrapping_mul(MAGIC_MULTIPLIER)) as u64 >> 32) as i64 & 0x3FF;
    //     long temperature = (absValue ^ broadcastSign) - broadcastSign;
    (abs_value ^ broadcast_sign) - broadcast_sign
    //     long nextLineStart = (dotPos >>> 3) + 3;
    //     return (int) temperature;
}

fn round(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::fs::read_to_string;

    #[test]
    fn test_one_brc() {
        let actual = super::one_brc("/Users/steam/git/1brc/measurements_10_000.txt");
        let expected = read_to_string("./data/sol_10_000.txt").expect("Could not read file");
        assert_eq!(actual, expected);
    }
}
