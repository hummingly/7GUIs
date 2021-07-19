use std::ops::Range;

use eframe::egui::{self, TextBuffer as _};

pub fn temperature_converter(ui: &mut egui::Ui, state: &mut TemperatureConverterModel) {
    ui.columns(2, |columns| {
        columns[0].label("Celsius");
        let celsius_response = columns[0].text_edit_singleline(&mut state.celsius);
        if celsius_response.changed() {
            state.update_fahrenheit();
        }

        columns[1].label("Fahrenheit");
        let fahrenheit_response = columns[1].text_edit_singleline(&mut state.fahrenheit);
        if fahrenheit_response.changed() {
            state.update_celsius();
        }
    })
}

#[derive(Default)]
pub struct TemperatureConverterModel {
    celsius: NumberText,
    fahrenheit: NumberText,
}

impl TemperatureConverterModel {
    fn update_fahrenheit(&mut self) {
        if let Ok(celsius) = self.celsius.as_str().parse::<f64>() {
            let fahrenheit = celsius * 1.8 + 32.0;
            self.fahrenheit.0 = fahrenheit.round().to_string().into_bytes();
        }
    }

    fn update_celsius(&mut self) {
        if let Ok(fahrenheit) = self.fahrenheit.as_str().parse::<f64>() {
            let celsius = (fahrenheit - 32.0) / 1.8;
            self.celsius.0 = celsius.round().to_string().into_bytes();
        }
    }
}

/// Allows only numbers as input and sign.
#[derive(Default)]
struct NumberText(Vec<u8>);

impl AsRef<str> for NumberText {
    fn as_ref(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl From<NumberText> for String {
    fn from(value: NumberText) -> Self {
        unsafe { String::from_utf8_unchecked(value.0) }
    }
}

impl egui::TextBuffer for NumberText {
    fn insert_text(&mut self, text: &str, ch_idx: usize) -> usize {
        if self.0.is_empty() {
            self.replace(text);
            return self.0.len();
        }

        // Prepend text
        if ch_idx == 0 {
            let mut beginning = Vec::new();
            input_number(&mut beginning, text);
            if beginning.is_empty() {
                return 0;
            }
            // Overwrite previous sign with last digit of beginning of input
            if let b'+' | b'-' = self.0[0] {
                self.0[0] = beginning.pop().unwrap();
            }
            let count = beginning.len();
            beginning.append(&mut self.0);
            std::mem::swap(&mut self.0, &mut beginning);
            count
            // Append text
        } else if ch_idx == self.0.len() {
            let old_count = self.0.len();
            self.0.extend(
                text.chars()
                    .filter_map(|number| number.is_ascii_digit().then(|| number as u8)),
            );
            self.0.len() - old_count
            // Insert text
        } else {
            let mut ending = self.0.split_off(ch_idx);
            let old_count = self.0.len();
            self.0.extend(
                text.chars()
                    .filter_map(|number| number.is_ascii_digit().then(|| number as u8)),
            );
            let count = self.0.len() - old_count;
            self.0.append(&mut ending);
            count
        }
    }

    fn delete_char_range(&mut self, ch_range: Range<usize>) {
        self.0.drain(ch_range);
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn replace(&mut self, text: &str) {
        self.0.clear();
        input_number(&mut self.0, text)
    }

    fn take(&mut self) -> String {
        let bytes = std::mem::take(&mut self.0);
        unsafe { String::from_utf8_unchecked(bytes) }
    }
}

fn input_number(ouput: &mut Vec<u8>, text: &str) {
    let mut chars = text.chars();
    if let Some(first_char @ ('+' | '-' | '0'..='9')) = chars.next() {
        ouput.push(first_char as u8);
    }
    ouput.extend(chars.filter_map(|number| number.is_ascii_digit().then(|| number as u8)))
}
