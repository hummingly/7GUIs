use std::{
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc, RwLock,
    },
    time::{Duration, Instant},
};

use eframe::egui;

use crate::util::DispatcherTimer;

pub fn timer(ui: &mut egui::Ui, state: &mut TimerModel) {
    let elapsed_time = state.elapsed_time.get();
    let duration = state.duration.get();

    if elapsed_time < duration {
        ui.ctx().request_repaint();
    }

    ui.label(format!(
        "Elapsed Time: {:.1}s",
        f64::from(elapsed_time) / 1000.0
    ));

    ui.label(format!("Duration: {}s", f64::from(duration) / 1000.0));

    let slider = egui::Slider::from_get_set(0.0..=30.0, |value| {
        if let Some(v) = value {
            let duration = (v.trunc() * 1000.0) as u16;
            state.duration.set(duration);
            if elapsed_time >= duration {
                state.elapsed_time.set(duration);
            } else {
                state.restart_timer();
            }
            return v;
        }
        (state.duration.get() as f64) / 1000.0
    })
    .fixed_decimals(0)
    .clamp_to_range(true);
    ui.add(slider);

    if ui.button("Reset Timer").clicked() {
        state.reset_timer();
    }
}

pub struct TimerModel {
    duration: Milliseconds,
    elapsed_time: Milliseconds,
    stop_watch: Arc<RwLock<Instant>>,
    timer: DispatcherTimer,
}

impl Default for TimerModel {
    fn default() -> Self {
        Self::new()
    }
}

impl TimerModel {
    fn new() -> Self {
        let duration = Milliseconds::from(15000);
        let elapsed_time = Milliseconds::from(0);
        let stop_watch = Arc::new(RwLock::new(Instant::now()));

        let thread_duration = duration.clone();
        let thread_elapsed_time = elapsed_time.clone();
        let thread_stop_watch = Arc::clone(&stop_watch);

        let timer = DispatcherTimer::new(100, move || {
            match thread_stop_watch.read() {
                Ok(lock) => thread_elapsed_time.set(lock.elapsed().as_millis() as u16),
                Err(_) => return thread_elapsed_time >= thread_duration,
            }

            if thread_elapsed_time >= thread_duration {
                thread_elapsed_time.set(thread_duration.get());
                return true;
            }
            false
        });
        timer.start();

        Self {
            duration,
            elapsed_time,
            stop_watch,
            timer,
        }
    }

    fn restart_timer(&self) {
        if !self.timer.is_enabled() {
            if let Ok(mut stop_watch) = self.stop_watch.try_write() {
                *stop_watch =
                    Instant::now() - Duration::from_millis(u64::from(self.elapsed_time.get()));
                drop(stop_watch);
                self.timer.start();
            }
        }
    }

    fn reset_timer(&self) {
        self.elapsed_time.set(0);
        if self.timer.stop() && self.duration.get() != 0 {
            if let Ok(mut stop_watch) = self.stop_watch.try_write() {
                *stop_watch = Instant::now();
                drop(stop_watch);
                self.timer.start();
            }
        }
    }
}

/// Threadsafe type to represent duration and elapsed time of the timer.
struct Milliseconds(Arc<AtomicU16>);

impl Milliseconds {
    fn get(&self) -> u16 {
        self.0.load(Ordering::Acquire)
    }

    fn set(&self, value: u16) {
        self.0.store(value, Ordering::Release)
    }
}

impl Clone for Milliseconds {
    fn clone(&self) -> Self {
        Milliseconds(Arc::clone(&self.0))
    }
}

impl Ord for Milliseconds {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get().cmp(&other.get())
    }
}

impl PartialOrd for Milliseconds {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Milliseconds {}

impl PartialEq for Milliseconds {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl From<u16> for Milliseconds {
    fn from(value: u16) -> Self {
        Milliseconds(Arc::new(AtomicU16::new(value)))
    }
}
