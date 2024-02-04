#[derive(Default)]
pub struct Converter {
    raw_input: String,
    input: f64,
    calculated: f64,
}

pub enum Mode {
    FarToCel,
    CelToFar,
}

impl Converter {
    pub fn input_char(&mut self, input: char) {
        if input >= '0' && input <= '9' {
            if self.raw_input.len() <= 10 {
                self.raw_input.push(input);
            }
        }
    }

    pub fn clear(&mut self) {
        self.raw_input.clear();
    }

    pub fn calculate(&mut self, mode: &Mode) {
        match self.raw_input.parse() {
            Ok(input) => self.input = input,
            Err(_) => {
                self.input = 0.0;
            }
        }
        match mode {
            Mode::FarToCel => self.calculated = farenheit_to_celsius(self.input),
            Mode::CelToFar => self.calculated = celsius_to_farenheit(self.input),
        };
    }

    pub fn raw_input(&self) -> String {
        return format!("{}°", self.raw_input.clone());
    }

    pub fn calculated(&self) -> String {
        return format!("{:.1}°", self.calculated);
    }
}

fn farenheit_to_celsius(f: f64) -> f64 {
    return (f - 32.0) * 5.0 / 9.0;
}

pub fn celsius_to_farenheit(c: f64) -> f64 {
    return c * 9.0 / 5.0 + 32.0;
}
