use eframe::egui;

pub fn crud(ui: &mut egui::Ui, state: &mut CrudModel) {
    ui.horizontal(|ui| {
        ui.label("Filter prefix");
        ui.text_edit_singleline(&mut state.query);
    });

    ui.columns(2, |columns| {
        let row_height = columns[0].fonts().row_height(egui::TextStyle::Button);
        let listbox = egui::ScrollArea::from_max_height(row_height * 5.0);
        listbox.show(&mut columns[0], |ui| {
            if state.query.is_empty() {
                user_list(ui, state.db.users.iter(), &mut state.selection);
            } else {
                user_list(ui, state.db.query(&state.query), &mut state.selection)
            }
        });

        columns[1].label("Name");
        columns[1].text_edit_singleline(&mut state.name_input);
        columns[1].label("Surname");
        columns[1].text_edit_singleline(&mut state.surname_input);
    });

    ui.horizontal(|ui| {
        if ui.button("Create").clicked() {
            state.db.create(&state.name_input, &state.surname_input);
        }

        let update_button = egui::Button::new("Update").enabled(state.selection.is_some());
        if ui.add(update_button).clicked() {
            if let Some(id) = state.selection {
                state.db.update(id, &state.name_input, &state.surname_input);
            }
        }

        let delete_button = egui::Button::new("Delete").enabled(state.selection.is_some());
        if ui.add(delete_button).clicked() {
            if let Some(id) = state.selection {
                if state.db.delete(id) {
                    state.selection.take();
                }
            }
        }
    });
}

fn user_list<'a>(
    ui: &mut egui::Ui,
    users: impl Iterator<Item = &'a User>,
    selection: &mut Option<UserId>,
) {
    for user in users {
        if ui
            .add(egui::SelectableLabel::new(
                Some(user.id) == *selection,
                format!("{}, {}", user.surname, user.name),
            ))
            .clicked()
        {
            selection.replace(user.id);
        }
    }
}

#[derive(Default)]
pub struct CrudModel {
    db: Db,
    query: String,
    selection: Option<UserId>,
    name_input: String,
    surname_input: String,
}

struct Db {
    id_counter: UserId,
    users: Vec<User>,
}

impl Db {
    fn new() -> Self {
        Self {
            id_counter: 3,
            users: vec![
                User::new(0, "Hans", "Emil"),
                User::new(1, "Max", "Mustermann"),
                User::new(2, "Roman", "Tisch"),
            ],
        }
    }

    fn create(&mut self, name: &str, surname: &str) {
        let id = self.id_counter;
        self.users.push(User::new(id, name, surname));
        self.id_counter += 1;
    }

    fn update(&mut self, id: UserId, name: &str, surname: &str) {
        if let Some(user) = self.users.iter_mut().find(|u| u.id == id) {
            user.update(name, surname);
        }
    }

    fn delete(&mut self, id: UserId) -> bool {
        if let Some(index) = self.users.iter().position(|u| u.id == id) {
            self.users.remove(index);
            return true;
        }
        false
    }

    fn query(&self, query: &str) -> impl Iterator<Item = &'_ User> {
        let query = query.to_lowercase();
        self.users.iter().filter(move |u| {
            u.name.to_lowercase().contains(&query) || u.surname.to_lowercase().contains(&query)
        })
    }
}

impl Default for Db {
    fn default() -> Self {
        Self::new()
    }
}

type UserId = u32;

struct User {
    id: UserId,
    name: Box<str>,
    surname: Box<str>,
}

impl User {
    fn new(id: UserId, name: &str, surname: &str) -> Self {
        Self {
            id,
            name: Box::from(name),
            surname: Box::from(surname),
        }
    }

    fn update(&mut self, name: &str, surname: &str) {
        self.name = Box::from(name);
        self.surname = Box::from(surname);
    }
}
