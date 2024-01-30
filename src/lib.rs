#![feature(type_alias_impl_trait, const_async_blocks)]
#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::undocumented_unsafe_blocks,
    rust_2018_idioms
)]
extern crate alloc;

use alloc::string::ToString;
use asr::{
    deep_pointer::{DeepPointer, DerefType}, future::{next_tick, retry}, print_message, settings::{gui::Title, Gui}, time::Duration, timer::{self, TimerState}, watcher::Watcher, Address, Address64, Process
};
use field_names::FieldNames;
asr::async_main!(nightly);

const PROCESS_NAMES: &[&str] = &["Refunct-Win32-Shipping.exe", "Refunct-Linux-Shipping"];
static mut HAS_SPLIT: bool = false;

async fn main() {
    let mut settings = Settings::register();
    asr::set_tick_rate(60.);
    let mut watchers = Watchers::default();
    loop {
        let process = retry(|| PROCESS_NAMES.iter().find_map(|name| Process::attach(name))).await;
        let memory = Memory::init(&process).await;
        process.until_closes(async {
            loop {
                settings.update();
                update_loop(&process, &memory, &mut watchers);
                if reset(&watchers, &settings) {
                    timer::reset();
                }
                if timer::state() == TimerState::NotRunning && start(&watchers, &settings) {
                    timer::start();
                }
                if split(&watchers, &settings) {
                    timer::split();
                }
                game_time(&watchers, &settings);
                next_tick().await;
            }
        }).await;
    }
}

#[derive(Gui, FieldNames)]
struct Settings {
    _general_settings: Title,
    #[default = true]
    /// Enable auto-start
    start: bool,
    /// Enable auto-reset
    #[default = true]
    reset: bool,
    /// Split on button press
    split_on_bpress: Title,
    #[default = true]
    /// Button 1
    b1: bool,
    #[default = true]
    /// Button 2
    b2: bool,
    #[default = true]
    /// Button 3
    b3: bool,
    #[default = true]
    /// Button 4
    b4: bool,
    #[default = true]
    /// Button 5
    b5: bool,
    #[default = true]
    /// Button 6
    b6: bool,
    #[default = true]
    /// Button 7.1
    b7: bool,
    #[default = true]
    /// Button 7.2
    b8: bool,
    #[default = true]
    /// Button 8
    b9: bool,
    #[default = true]
    /// Button 9
    b10: bool,
    #[default = true]
    /// Button 10.1
    b11: bool,
    #[default = true]
    /// Button 10.2
    b12: bool,
    #[default = true]
    /// Button 11
    b13: bool,
    #[default = true]
    /// Button 12
    b14: bool,
    #[default = true]
    /// Button 13
    b15: bool,
    #[default = true]
    /// Button 14
    b16: bool,
    #[default = true]
    /// Button 15
    b17: bool,
    #[default = true]
    /// Button 16
    b18: bool,
    #[default = true]
    /// Button 17
    b19: bool,
    #[default = true]
    /// Button 18.1
    b20: bool,
    #[default = true]
    /// Button 18.2
    b21: bool,
    #[default = true]
    /// Button 19
    b22: bool,
    #[default = true]
    /// Button 20
    b23: bool,
    #[default = true]
    /// Button 21
    b24: bool,
    #[default = true]
    /// Button 22
    b25: bool,
    #[default = true]
    /// Button 23
    b26: bool,
    #[default = true]
    /// Button 24
    b27: bool,
    #[default = true]
    /// Button 25
    b28: bool,
    #[default = true]
    /// Button 26.1
    b29: bool,
    #[default = true]
    /// Button 26.2
    b30: bool,
    #[default = true]
    /// Button 26.3
    b31: bool,
    #[default = true]
    /// Button 27
    b32: bool,
    #[default = true]
    /// Button 28.1
    b33: bool,
    #[default = true]
    /// Button 28.2
    b34: bool,
    #[default = true]
    /// Button 29
    b35: bool,
    #[default = true]
    /// Button 30
    b36: bool,
    #[default = true]
    /// Button 31
    b37: bool,
    /// Split on cube collection
    split_on_cubecol: Title,
    #[default = false]
    /// Cube 1
    c1: bool,
    #[default = false]
    /// Cube 2
    c2: bool,
    #[default = false]
    /// Cube 3
    c3: bool,
    #[default = false]
    /// Cube 4
    c4: bool,
    #[default = false]
    /// Cube 5
    c5: bool,
    #[default = false]
    /// Cube 6
    c6: bool,
    #[default = false]
    /// Cube 7
    c7: bool,
    #[default = false]
    /// Cube 8
    c8: bool,
    #[default = false]
    /// Cube 9
    c9: bool,
    #[default = false]
    /// Cube 10
    c10: bool,
    #[default = false]
    /// Cube 11
    c11: bool,
    #[default = false]
    /// Cube 12
    c12: bool,
    #[default = false]
    /// Cube 13
    c13: bool,
    #[default = false]
    /// Cube 14
    c14: bool,
    #[default = false]
    /// Cube 15
    c15: bool,
    #[default = false]
    /// Cube 16
    c16: bool,
    #[default = false]
    /// Cube 17
    c17: bool,
    #[default = false]
    /// Cube 18
    c18: bool,
}

// Define a trait for accessing boolean fields
trait BoolFieldAccess {
    fn get_bool_field_by_name(&self, field_name: &str) -> Option<bool>;
}

// Implement the trait for MyStruct
impl BoolFieldAccess for Settings {
    fn get_bool_field_by_name(&self, name: &str) -> Option<bool> {
        match name {
            "b1" => Some(self.b1),
            "b2" => Some(self.b2),
            "b3" => Some(self.b3),
            "b4" => Some(self.b4),
            "b5" => Some(self.b5),
            "b6" => Some(self.b6),
            "b7" => Some(self.b7),
            "b8" => Some(self.b8),
            "b9" => Some(self.b9),
            "b10" => Some(self.b10),
            "b11" => Some(self.b11),
            "b12" => Some(self.b12),
            "b13" => Some(self.b13),
            "b14" => Some(self.b14),
            "b15" => Some(self.b15),
            "b16" => Some(self.b16),
            "b17" => Some(self.b17),
            "b18" => Some(self.b18),
            "b19" => Some(self.b19),
            "b20" => Some(self.b20),
            "b21" => Some(self.b21),
            "b22" => Some(self.b22),
            "b23" => Some(self.b23),
            "b24" => Some(self.b24),
            "b25" => Some(self.b25),
            "b26" => Some(self.b26),
            "b27" => Some(self.b27),
            "b28" => Some(self.b28),
            "b29" => Some(self.b29),
            "b30" => Some(self.b30),
            "b31" => Some(self.b31),
            "b32" => Some(self.b32),
            "b33" => Some(self.b33),
            "b34" => Some(self.b34),
            "b35" => Some(self.b35),
            "b36" => Some(self.b36),
            "b37" => Some(self.b37),
            "c1" => Some(self.c1),
            "c2" => Some(self.c2),
            "c3" => Some(self.c3),
            "c4" => Some(self.c4),
            "c5" => Some(self.c5),
            "c6" => Some(self.c6),
            "c7" => Some(self.c7),
            "c8" => Some(self.c8),
            "c9" => Some(self.c9),
            "c10" => Some(self.c10),
            "c11" => Some(self.c11),
            "c12" => Some(self.c12),
            "c13" => Some(self.c13),
            "c14" => Some(self.c14),
            "c15" => Some(self.c15),
            "c16" => Some(self.c16),
            "c17" => Some(self.c17),
            "c18" => Some(self.c18),
            _ => None,
        }
    }
}

#[derive(Default)]
struct Watchers {
    cubes: Watcher<i32>,
    buttons: Watcher<i32>,
    resets: Watcher<i32>,
    start_seconds: Watcher<i32>,
    start_partial: Watcher<f64>,
    finish_seconds: Watcher<i32>,
    finish_partial: Watcher<f64>,
}

struct Memory {
    cubes: DeepPointer<5>,
    buttons: DeepPointer<5>,
    resets: DeepPointer<5>,
    start_seconds: DeepPointer<5>,
    start_partial: DeepPointer<5>,
    finish_seconds: DeepPointer<5>,
    finish_partial: DeepPointer<5>,
}

impl Memory {
    async fn init(game: &Process) -> Self {
        let mut cubes_ptr = DeepPointer::default();
        let mut buttons_ptr = DeepPointer::default();
        let mut resets_ptr = DeepPointer::default();
        let mut start_seconds_ptr = DeepPointer::default();
        let mut start_partial_ptr = DeepPointer::default();
        let mut finish_seconds_ptr = DeepPointer::default();
        let mut finish_partial_ptr = DeepPointer::default();
        match asr::get_os().map(|array_string| array_string.as_str().to_string()) {
            Ok(string) => {
                print_message(&format!("ASR: Operating system is {}.", string));
                if string == "windows" {
                    let base_address = retry(|| game.get_module_address("Refunct-Win32-Shipping.exe")).await;
                    cubes_ptr = DeepPointer::new(base_address, DerefType::Bit32, &[0x1FBF9EC, 0xC0, 0xA4]);
                    buttons_ptr = DeepPointer::new(base_address, DerefType::Bit32, &[0x1FBF9EC, 0xC0, 0xA8]);
                    resets_ptr = DeepPointer::new(base_address, DerefType::Bit32, &[0x1FBF9EC, 0xC0, 0xAC]);
                    start_seconds_ptr = DeepPointer::new(base_address, DerefType::Bit32, &[0x1FBF9EC, 0xC0, 0xB0]);
                    start_partial_ptr = DeepPointer::new(base_address, DerefType::Bit32, &[0x1FBF9EC, 0xC0, 0xB4]);
                    finish_seconds_ptr = DeepPointer::new(base_address, DerefType::Bit32, &[0x1FBF9EC, 0xC0, 0xB8]);
                    finish_partial_ptr = DeepPointer::new(base_address, DerefType::Bit32, &[0x1FBF9EC, 0xC0, 0xBC]);
                } else if string == "linux" {
                    // TODO
                    let base_address = retry(|| game.get_module_address("Refunct-Linux-Shipping")).await;
                    cubes_ptr = DeepPointer::new(base_address + 0x5068838, DerefType::Bit64, &[0x138, 0x13C]);
                    buttons_ptr = DeepPointer::new(base_address + 0x5068838, DerefType::Bit64, &[0x138, 0x140]);
                    resets_ptr = DeepPointer::new(base_address + 0x5068838, DerefType::Bit64, &[0x138, 0x148]);
                    start_seconds_ptr = DeepPointer::new(base_address + 0x5068838, DerefType::Bit64, &[0x138, 0x148]);
                    start_partial_ptr = DeepPointer::new(base_address + 0x5068838, DerefType::Bit64, &[0x138, 0x14C]);
                    finish_seconds_ptr = DeepPointer::new(base_address + 0x5068838, DerefType::Bit64, &[0x138, 0x150]);
                    finish_partial_ptr = DeepPointer::new(base_address + 0x5068838, DerefType::Bit64, &[0x138, 0x154]);
                }
            }
            Err(err) => {
                print_message("ASR Error: Could not get operating system.");
            }
        }
        asr::print_limited::<24>(&"  => Autosplitter ready!");
        Self { cubes: cubes_ptr, buttons: buttons_ptr, resets: resets_ptr, start_seconds: start_seconds_ptr, start_partial: start_partial_ptr,
            finish_seconds: finish_seconds_ptr, finish_partial: finish_partial_ptr}
    }
}

fn update_loop(game: &Process, addresses: &Memory, watchers: &mut Watchers) {
    watchers.cubes.update_infallible(addresses.cubes.deref(game).unwrap_or_default());
    watchers.buttons.update_infallible(addresses.buttons.deref(game).unwrap_or_default());
    watchers.resets.update_infallible(addresses.resets.deref(game).unwrap_or_default());
    watchers.start_seconds.update_infallible(addresses.start_seconds.deref(game).unwrap_or_default());
    watchers.start_partial.update_infallible(addresses.start_partial.deref(game).unwrap_or_default());
    watchers.finish_seconds.update_infallible(addresses.finish_seconds.deref(game).unwrap_or_default());
    watchers.finish_partial.update_infallible(addresses.finish_partial.deref(game).unwrap_or_default());
}

fn start(watchers: &Watchers, settings: &Settings) -> bool {
    settings.start && watchers.resets.pair.is_some_and(|val| val.increased())
}

fn split(watchers: &Watchers, settings: &Settings) -> bool {
    let buttons_pair = watchers.buttons.pair.unwrap_or_default();
    let cubes_pair = watchers.cubes.pair.unwrap_or_default();

    if buttons_pair.current > 0 || cubes_pair.current > 0 {
        unsafe {
            if HAS_SPLIT && (buttons_pair.increased() || cubes_pair.increased()) {
                let buttons_string = format!("b{}", buttons_pair.current);
                let cubes_string = format!("c{}", cubes_pair.current);

                if settings.get_bool_field_by_name(&buttons_string).is_some()
                    || settings.get_bool_field_by_name(&cubes_string).is_some()
                {
                    HAS_SPLIT = true;
                    return buttons_pair.increased() && settings.get_bool_field_by_name(&buttons_string).unwrap_or_default()
                        || cubes_pair.increased() && settings.get_bool_field_by_name(&cubes_string).unwrap_or_default();
                } else {
                    return false;
                }
            }
        }
    }
    false
}

fn reset(watchers: &Watchers, settings: &Settings) -> bool {
    settings.reset && watchers.resets.pair.is_some_and(|val| val.increased())
        && watchers.buttons.pair.is_some_and(|val: asr::watcher::Pair<i32>| val.changed_to(&0))
}

fn game_time(_watchers: &Watchers, _settings: &Settings) {
    if let (Some(finish), Some(start), Some(finish_partial), Some(start_partial)) = (
        _watchers.finish_seconds.pair, _watchers.start_seconds.pair, _watchers.finish_partial.pair, _watchers.start_partial.pair,
    ) {
        if finish.current > start.current && timer::state() == TimerState::Running {
            let s = finish.current - start.current;
            let ms = finish_partial.current - start_partial.current;
            let time = s as f64 + ms;
            timer::set_game_time(Duration::seconds_f64(time));
        }
    }
}
