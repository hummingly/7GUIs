use eframe::egui;

pub fn circle_drawer(ui: &mut egui::Ui, state: &mut CircleDrawerModel, viewport: egui::Rect) {
    ui.horizontal(|ui| {
        let undo_button = egui::Button::new("Undo").enabled(!state.history.is_empty());
        if ui.add(undo_button).clicked() {
            state.undo();
        }

        let redo_button = egui::Button::new("Redo").enabled(!state.history.is_full());
        if ui.add(redo_button).clicked() {
            state.redo();
        }
    });

    let desired_size = egui::vec2(600.0, 400.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    let offset = viewport.min; // TODO: Something is wrong
    let visuals = ui.style().visuals.widgets.inactive;

    let mut child_ui = ui.child_ui(rect, *ui.layout());
    child_ui.set_clip_rect(rect);

    let mut is_over_circle = false;
    let mut popup_id = None;
    let mut updated_circle = None;

    for circle in state.circles.iter_mut() {
        let (is_hit, position) = circle_button(&mut child_ui, circle, offset);

        if !is_over_circle && is_hit {
            is_over_circle = true;
            popup_id = Some(ui.make_persistent_id(circle.id));
            state.old_radius = circle.radius;
        }

        if editor_popup(ui, circle, position) {
            updated_circle = Some((circle.id, circle.radius));
        }
    }

    ui.painter()
        .rect_stroke(rect, visuals.corner_radius, visuals.fg_stroke);

    // Opened editor
    if is_over_circle && response.clicked() {
        if let Some(id) = popup_id {
            ui.memory().open_popup(id);
        }
    } else if response.clicked() {
        // Clicked on free space
        if let Some(pos) = response.interact_pointer_pos() {
            dbg!(pos, offset, rect.min);
            state.create_circle(egui::pos2(pos.x, pos.y)); // TODO: Fix offset
            ui.ctx().request_repaint();
        }
    } else if let Some((id, radius)) = updated_circle {
        // Editor was closed
        state.update_circle(id, radius);
    }
}

fn circle_button(ui: &mut egui::Ui, circle: &mut Circle, offset: egui::Pos2) -> (bool, egui::Pos2) {
    fn hit_test(circle: &Circle, pos: egui::Pos2) -> bool {
        let p = circle.center - pos;
        p.x.mul_add(p.x, p.y * p.y) <= circle.radius.powi(2)
    }

    let rect = circle.to_rect(offset);
    let response = ui.allocate_rect(rect, egui::Sense::click());
    let hit_position = response
        .hover_pos()
        .or_else(|| response.interact_pointer_pos());

    let (visuals, hit_response) = match hit_position {
        Some(pos) if hit_test(circle, pos) => (ui.style().interact(&response), response.hovered()),
        _ => (&ui.style().visuals.widgets.inactive, false),
    };

    ui.painter().circle(
        circle.center,
        circle.radius,
        visuals.bg_fill,
        visuals.fg_stroke,
    );

    (hit_response, response.rect.left_bottom())
}

fn editor_popup(ui: &egui::Ui, circle: &mut Circle, position: egui::Pos2) -> bool {
    let popup_id = ui.make_persistent_id(circle.id);
    if ui.memory().is_popup_open(popup_id) {
        let response = egui::Area::new(popup_id)
            .order(egui::Order::Foreground)
            .fixed_pos(position)
            .show(ui.ctx(), |ui| {
                let frame = egui::Frame::popup(ui.style());
                frame.show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        ui.set_min_width(200.0);
                        ui.label("Adjust Diameter");
                        let slider = egui::Slider::from_get_set(2.0..=100.0, |v| {
                            if let Some(value) = v {
                                circle.radius = (value / 2.0) as f32;
                                return value;
                            }
                            f64::from(circle.radius * 2.0)
                        })
                        .clamp_to_range(true)
                        .fixed_decimals(0);
                        ui.add(slider);
                    });
                });
            });

        if ui.input().key_pressed(egui::Key::Escape) || response.clicked_elsewhere() {
            ui.memory().close_popup();
            return true;
        }
    }
    false
}

#[derive(Debug, Default)]
pub struct CircleDrawerModel {
    id_counter: CircleId,
    history: CircleDrawerHistory,
    circles: Vec<Circle>,
    old_radius: f32,
}

impl CircleDrawerModel {
    fn create_circle(&mut self, center: egui::Pos2) {
        let circle = Circle::new(self.id_counter, center);
        self.circles.push(circle.clone());
        self.history.create(circle);
        self.id_counter += 1;
    }

    fn update_circle(&mut self, id: CircleId, radius: f32) {
        if self.old_radius == radius {
            return;
        }

        if let Some(circle) = self.circles.iter().find(|c| c.id == id) {
            self.history.update(circle.clone());
        }
    }

    fn undo(&mut self) {
        if let Some(change) = self.history.undo() {
            match change {
                Change::Create(data) => {
                    let id = data.id;
                    if let Some(index) = self.circles.iter().position(|c| c.id == id) {
                        self.circles.remove(index);
                    }
                }
                Change::Update(data) => {
                    let id = data.id;
                    if let Some(previous_radius) = self
                        .history
                        .current_changes()
                        .iter()
                        .rev()
                        .find_map(|c| (c.data().id == id).then(|| c.data().radius))
                    {
                        if let Some(circle) = self.circles.iter_mut().find(|c| c.id == id) {
                            circle.radius = previous_radius;
                        }
                    }
                }
            }
        }
    }

    fn redo(&mut self) {
        if let Some(change) = self.history.redo() {
            match change {
                Change::Create(data) => {
                    self.circles.push(data.clone());
                }
                Change::Update(data) => {
                    let id = data.id;
                    if let Some(circle) = self.circles.iter_mut().find(|c| c.id == id) {
                        circle.radius = data.radius;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct CircleDrawerHistory {
    changes: Vec<Change>,
    applied_changes: usize,
}

impl CircleDrawerHistory {
    /// No changes are currently applied.
    fn is_empty(&self) -> bool {
        self.applied_changes == 0
    }

    /// All recorded changes are currently applied.
    fn is_full(&self) -> bool {
        self.changes.len() == self.applied_changes
    }

    fn current_changes(&self) -> &[Change] {
        &self.changes[0..self.applied_changes]
    }

    fn create(&mut self, data: Circle) {
        self.push(Change::Create(data));
    }

    fn update(&mut self, data: Circle) {
        self.push(Change::Update(data));
    }

    fn push(&mut self, change: Change) {
        self.changes.truncate(self.applied_changes);
        self.changes.push(change);
        self.applied_changes = self.changes.len();
    }

    fn undo(&mut self) -> Option<&Change> {
        if self.applied_changes > 0 {
            self.applied_changes -= 1;
            return Some(&self.changes[self.applied_changes]);
        }
        None
    }

    fn redo(&mut self) -> Option<&Change> {
        if self.applied_changes < self.changes.len() {
            let change = Some(&self.changes[self.applied_changes]);
            self.applied_changes += 1;
            return change;
        }
        None
    }
}

#[derive(Debug)]
enum Change {
    Create(Circle),
    Update(Circle),
}

impl Change {
    fn data(&self) -> &Circle {
        match self {
            Change::Create(data) => data,
            Change::Update(data) => data,
        }
    }
}

type CircleId = u32;

#[derive(Debug, Clone)]
struct Circle {
    id: CircleId,
    center: egui::Pos2,
    radius: f32,
}

impl Circle {
    fn new(id: CircleId, center: egui::Pos2) -> Self {
        Self {
            id,
            center,
            radius: 15.0,
        }
    }

    fn to_rect(&self, _offset: egui::Pos2) -> egui::Rect {
        let diameter = self.radius * 2.0;
        let center = egui::pos2(self.center.x, self.center.y);
        egui::Rect::from_center_size(center, egui::vec2(diameter, diameter))
    }
}
