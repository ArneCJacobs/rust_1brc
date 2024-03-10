use std::collections::HashMap;
use std::io::BufRead;

struct Measurement {
    min: i32,
    max: i32,
    count: i64,
    sum: i32,
}
// used https://sdremthix.medium.com/branchless-programming-why-your-cpu-will-thank-you-5f405d97b0c8
// as a reference

#[inline]
pub fn one_brc(file: &str) -> String {
    let mut measurements: HashMap<String, Measurement> = HashMap::new();
    let file = std::fs::File::open(file).unwrap();
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let (station, value_str) = line.split_once(';').unwrap();

        // branclessly parse value 
        
        let mut value: i32;
        unsafe {
            let mut index: usize = 0;
            let str_slice = value_str.as_bytes();

            let negative = *str_slice.get_unchecked(index) == b'-';
            index = index.wrapping_add(negative as usize);

            value = (*str_slice.get_unchecked(index) - b'0') as i32;
            index += 1;

            // if str_slice[index] != b'.' {
            //   value = value * 10 + (str_slice[index] - b'0') as i32;
            // }
            let is_dot = (*str_slice.get_unchecked(index) == b'.') as i32;
            value += (is_dot - 1) & (9 * value + ((*str_slice.get_unchecked(index)).wrapping_sub(b'0') as i32));

            index += 2 - is_dot as usize;

            value = value * 10 + (*str_slice.get_unchecked(index) - b'0') as i32;

            // if negative {
            //   value = -value; 
            // }
            let mask = negative as i32 - 1;
            value = (value & mask) | (-value & !mask);
        }



        // update measurement
        measurements
            .entry(station.to_string())
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

fn round(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_one_brc() {
        let actual = super::one_brc("/Users/steam/git/1brc/measurements_10_000.txt");
        let expected = read_to_string("./data/sol_10_000.txt").expect("Could not read file");
        assert_eq!(actual, expected);
    }
}
