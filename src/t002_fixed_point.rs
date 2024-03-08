use std::collections::HashMap;
use std::io::BufRead;

struct Measurement {
    min: f32,
    max: f32,
    count: i64,
    sum: f32,
}

#[inline]
pub fn one_brc(file: &str) {
    let mut measurements: HashMap<String, Measurement> = HashMap::new();
    let file = std::fs::File::open(file).unwrap();
    let reader = std::io::BufReader::new(file);
    reader.lines().for_each(|line| {
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
    });

    let mut measurements_list: Vec<_> = measurements.iter().collect();
    measurements_list.sort_unstable_by_key(|(station, _)| *station);

    let mut measurements_list: Vec<_> = measurements.iter().collect();
    measurements_list.sort_unstable_by_key(|(station, _)| *station);
    print!("{{");
    let mut iterator = measurements_list.iter();
    let (station, measurement) = iterator.next().unwrap();
    print!(
        "{}={}/{}/{}",
        station,
        measurement.min,
        measurement.sum / measurement.count as f32,
        measurement.max
    );

    for (station, measurement) in iterator {
        print!(
            ", {}={}/{}/{}",
            station,
            measurement.min,
            measurement.sum / measurement.count as f32,
            measurement.max
        );
    }
    println!("}}");

}
