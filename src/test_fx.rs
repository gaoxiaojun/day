#[cfg(test)]
pub mod tests {
    use crate::{bar::Bar, fractal::{Fractal, FractalType}};
    use crate::time::*;
    use chrono::prelude::*;
    use csv;


#[derive(Debug, Clone)]
    pub struct Fx {
        pub time:Time,
        pub ftype:FractalType,
        pub price:f64,
        pub high:f64,
        pub low:f64
    }
    impl Fx {
        pub fn new(time:Time, ftype:FractalType, price:f64, high:f64, low:f64) -> Self {
            Self {
                time,ftype,price,high,low
            }
        }
    }

    pub fn load_fx() -> Vec<Fx> {
        let csv = include_str!("../tests/dump_fx.csv");
        load_fx_from_csv(&csv)
    }

    pub fn load_fx_from_csv(csv: &str) -> Vec<Fx> {
        let mut fxs: Vec<Fx> = Vec::new();
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(csv.as_bytes());
        for record in reader.records() {
            let record = record.unwrap();
            let timestr: &str = AsRef::<str>::as_ref(&record[0]);
            let dt = NaiveDateTime::parse_from_str(timestr, "%Y-%m-%d %H:%M:%S").unwrap();
            let datetime: DateTime<Utc> = DateTime::from_utc(dt, Utc);
            let time = datetime.timestamp_millis();
            let type_str = AsRef::<str>::as_ref(&record[1]);
            let ftype=if type_str == "Top" {FractalType::Top}else {FractalType::Bottom};
            let price = AsRef::<str>::as_ref(&record[2]).parse::<f64>().unwrap();
            let high = AsRef::<str>::as_ref(&record[3]).parse::<f64>().unwrap();
            let low = AsRef::<str>::as_ref(&record[4]).parse::<f64>().unwrap();
            let fx = Fx::new(time,ftype,price,high,low);
            fxs.push(fx);
        }
        fxs
    }
}