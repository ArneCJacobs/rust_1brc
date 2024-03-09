use std::collections::HashMap;
use std::io::BufRead;

struct Measurement {
    min: f32,
    max: f32,
    count: i64,
    sum: f32,
}

#[inline]
pub fn one_brc(file: &str) -> String {
    let mut measurements: HashMap<String, Measurement> = HashMap::new();
    let file = std::fs::File::open(file).unwrap();
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let (station, value) = line.split_once(';').unwrap();
        let value: f32 = value.parse().unwrap();

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
                sum: 0.0,
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
                measurement.sum / measurement.count as f32,
                measurement.max
            )
        })
        .fold(String::new(), |acc, x| acc + ", " + &x);
}
