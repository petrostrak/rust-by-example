use std::fmt;

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if self.lon >= 0.0 {'E'} else {'W'};

        write!(f, "{}: {:.3}° {}, {:.3}° {}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB ({}, {}, {}) 0x{:0>2X}{:0>2X}{:0>2X}", self.red, self.green, self.blue, self.red, self.green, self.blue)
    }
}

fn main() {
    for city in [
        City{name: "Athens", lat: 37.983810, lon: 23.727539},
        City{name: "Hamburg", lat: 53.551086, lon: 9.993682},
        City{name: "Tokyo", lat: 35.652832, lon: 139.839478},
        City{name: "Alaska", lat: 66.160507, lon: -153.369141},
    ].iter() {
        println!("{}", city);
    }

    for color in [
        Color{red: 128, green: 255, blue: 90},
        Color{red: 0, green: 3, blue: 254},
        Color{red: 0, green: 0, blue: 0},
    ].iter() {
        println!("{}", color);
    }
}
