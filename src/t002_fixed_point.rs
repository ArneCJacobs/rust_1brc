use std::collections::HashMap;
use std::io::BufRead;

struct Measurement {
    min: i32,
    max: i32,
    count: i64,
    sum: i32,
}

#[inline]
pub fn one_brc(file: &str) -> String {
    let mut measurements: HashMap<String, Measurement> = HashMap::new();
    let file = std::fs::File::open(file).unwrap();
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let (station, value_str) = line.split_once(';').unwrap();

        // parse value
        let mut index: usize = 0;
        let str_slice = value_str.as_bytes();
        let mut negative = false;

        if str_slice[index] == b'-' {
            negative = true;
            index += 1;
        } 
        let mut value: i32 = (str_slice[index] - b'0') as i32;
        index += 1;
        if str_slice[index] != b'.' {
          value = value * 10 + (str_slice[index] - b'0') as i32;
        }
        index += 1;
        value = value * 10 + (str_slice[index] - b'0') as i32;
        if negative {
          value = -value; 
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
                sum: 0,
            });
    }

    let mut measurements_list: Vec<_> = measurements.iter().collect();
    measurements_list.sort_unstable_by_key(|(station, _)| *station);
    return measurements_list
        .iter()
        .map(|(station, measurement)| {
            format!(
                "{}={}/{}/{}",
                station,
                measurement.min,
                (measurement.sum as f32 / 10.0) / measurement.count as f32,
                measurement.max
            )
        })
        .fold(String::new(), |acc, x| acc + ", " + &x);
}
