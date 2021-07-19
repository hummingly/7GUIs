use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

use eframe::egui;
use time::{util::days_in_year_month, Date, Month, OffsetDateTime};

pub fn flight_booker(ui: &mut egui::Ui, state: &mut FlightBookerModel) {
    egui::ComboBox::from_label("Flight Type")
        .selected_text(state.kind)
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut state.kind, FlightKind::OneWay, "one-way flight");
            ui.selectable_value(&mut state.kind, FlightKind::Return, "return flight");
        });

    // This is really ugly but I am not bothered enough to find a prettier way to generate ids.
    let mut id = 0;

    ui.label("Departure Date");
    if date_input(
        ui,
        &mut id,
        &mut state.departure_date_input,
        &state.last_now,
    )
    .changed()
    {
        state.update_departure_date();
    }

    ui.label("Return Date");
    if ui
        .scope(|ui| {
            ui.set_enabled(state.kind == FlightKind::Return);
            date_input(
                ui,
                &mut id,
                &mut state.return_date_input,
                &state.departure_date,
            )
        })
        .inner
        .changed()
    {
        state.update_return_date();
    }

    let popup_id = ui.make_persistent_id("flight_booking_message");

    let response = ui.button("Book");

    egui::popup::popup_below_widget(ui, popup_id, &response, |ui| {
        ui.set_min_width(200.0);
        let text = match state.kind {
            FlightKind::OneWay => format!(
                "You have booked a one-way flight on {}!",
                state.departure_date
            ),
            FlightKind::Return => format!(
                "You have booked a return flight for the time from {} to {}!",
                state.departure_date, state.return_date
            ),
        };
        ui.label(text);
    });

    if response.clicked() {
        ui.memory().toggle_popup(popup_id);
    }
}

pub struct FlightBookerModel {
    kind: FlightKind,
    last_now: Date,
    departure_date: Date,
    return_date: Date,
    departure_date_input: DateInput,
    return_date_input: DateInput,
}

impl FlightBookerModel {
    fn update_departure_date(&mut self) {
        self.departure_date = Date::from(self.departure_date_input);
    }

    fn update_return_date(&mut self) {
        self.return_date = Date::from(self.return_date_input);
    }
}

impl Default for FlightBookerModel {
    fn default() -> Self {
        let now = OffsetDateTime::now_local()
            .unwrap_or_else(|_| OffsetDateTime::now_utc())
            .date();

        Self {
            kind: FlightKind::default(),
            last_now: now,
            departure_date: now,
            return_date: now,
            departure_date_input: DateInput::from(now),
            return_date_input: DateInput::from(now),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum FlightKind {
    OneWay,
    Return,
}

impl FlightKind {
    fn as_str(&self) -> &str {
        match self {
            FlightKind::OneWay => "one-way flight",
            FlightKind::Return => "return flight",
        }
    }
}

impl Display for FlightKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Default for FlightKind {
    fn default() -> Self {
        FlightKind::OneWay
    }
}

#[derive(Clone, Copy)]
struct DateInput {
    year: i32,
    month: Month,
    day: u8,
}

impl DateInput {
    fn set_date(&mut self, date: &Date) {
        self.year = date.year();
        self.month = date.month();
        self.day = date.day();
    }

    fn cmp(&self, other: &Date) -> Ordering {
        self.year
            .cmp(&other.year())
            .then((self.month as u8).cmp(&(other.month() as u8)))
            .then(self.day.cmp(&other.day()))
    }
}

impl From<Date> for DateInput {
    fn from(date: Date) -> Self {
        Self {
            year: date.year(),
            month: date.month(),
            day: date.day(),
        }
    }
}

impl From<DateInput> for Date {
    fn from(input: DateInput) -> Self {
        Date::from_calendar_date(input.year, input.month, input.day).unwrap()
    }
}

impl PartialEq<Date> for DateInput {
    fn eq(&self, other: &Date) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Date> for DateInput {
    fn partial_cmp(&self, other: &Date) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn date_input(
    ui: &mut egui::Ui,
    id: &mut u8,
    state: &mut DateInput,
    min_date: &Date,
) -> egui::Response {
    let mut has_changed = false;
    if state.cmp(min_date) == Ordering::Less {
        state.set_date(min_date);
        has_changed = true;
    }
    let mut response = ui.horizontal(|ui| {
        let min_year = min_date.year();
        let is_current_year = state.year == min_year;
        let year_response = year_picker(ui, *id, &mut state.year, min_year);
        *id += 1;
        let month_response = month_picker(
            ui,
            *id,
            &mut state.month,
            if is_current_year {
                min_date.month()
            } else {
                Month::January
            },
        );
        *id += 1;

        let is_current_month = state.month == min_date.month();
        let day = state.day;
        let day_response = day_picker(
            ui,
            *id,
            &mut state.day,
            if is_current_year && is_current_month {
                day
            } else {
                1
            },
            days_in_year_month(state.year, state.month),
        );
        *id += 1;

        year_response.union(month_response).union(day_response)
    });
    if has_changed {
        response.inner.mark_changed();
    }
    response.inner
}

fn year_picker(ui: &mut egui::Ui, id: u8, year: &mut i32, min_year: i32) -> egui::Response {
    picker_from_iter(ui, id, year, min_year..=min_year + 100)
}

fn month_picker(ui: &mut egui::Ui, id: u8, month: &mut Month, min_month: Month) -> egui::Response {
    let mut has_changed = false;
    if (*month as u8) < min_month as u8 {
        *month = min_month;
        has_changed = true;
    }
    let mut response = picker_from_iter(ui, id, month, iter_from_month(min_month));
    if has_changed {
        response.mark_changed();
    }
    response
}

fn day_picker(ui: &mut egui::Ui, id: u8, day: &mut u8, min_day: u8, max_day: u8) -> egui::Response {
    let mut has_changed = false;
    if *day < min_day {
        *day = min_day;
        has_changed = true;
    } else if *day > max_day {
        *day = max_day;
        has_changed = true;
    }
    let mut response = picker_from_iter(ui, id, day, min_day..=max_day);
    if has_changed {
        response.mark_changed();
    }
    response
}

fn picker_from_iter<V: fmt::Display + PartialEq>(
    ui: &mut egui::Ui,
    id: u8,
    selection: &mut V,
    values: impl Iterator<Item = V>,
) -> egui::Response {
    let mut has_changed = false;
    let mut response = egui::ComboBox::from_id_source(id)
        .selected_text(selection.to_string())
        .show_ui(ui, |ui| {
            for value in values {
                let is_selected = *selection == value;
                let response = ui.add(egui::SelectableLabel::new(is_selected, value.to_string()));
                if !is_selected && response.clicked() {
                    *selection = value;
                    has_changed = true
                }
            }
        });
    if has_changed {
        response.mark_changed();
    }
    response
}

fn iter_from_month(start_month: Month) -> impl Iterator<Item = Month> {
    let mut is_finished = false;
    let mut month = start_month;
    std::iter::from_fn(move || {
        if !is_finished {
            is_finished = month == Month::December;
            let next = Some(month);
            month = month.next();
            return next;
        }
        None
    })
}
