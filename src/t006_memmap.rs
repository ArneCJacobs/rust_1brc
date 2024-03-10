use std::collections::HashMap;

use memmap2::Mmap;

#[derive(Debug)]
struct Measurement {
    min: f64,
    max: f64,
    count: i64,
    sum: f64,
}

#[inline]
pub fn one_brc(file: &str) -> String {
    let mut measurements: HashMap<&str, Measurement> = HashMap::new();
    let file = std::fs::File::open(file).unwrap();
    let mmap = unsafe { Mmap::map(&file).expect("Whoopsy doodles")  };
    let mut mmap_index = 0;

    while mmap_index < mmap.len() {

        let name_start_index = mmap_index;
        while mmap[mmap_index] != b';' {
            mmap_index += 1;
        }
        let station = std::str::from_utf8(&mmap[name_start_index..mmap_index]).unwrap();
        // print!("{} ", station);
        mmap_index += 1;

        let value_start_index = mmap_index;

        while mmap[mmap_index] != b'\n' {
            mmap_index += 1;
        }
        let value_str = std::str::from_utf8(&mmap[value_start_index..mmap_index]).unwrap();
        let value = value_str.parse::<f64>().unwrap();
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
                measurement.min,
                round(round(measurement.sum) / measurement.count as f64),
                measurement.max
            )
        })
        .fold(String::new(), |acc, x| acc +  &x);
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
