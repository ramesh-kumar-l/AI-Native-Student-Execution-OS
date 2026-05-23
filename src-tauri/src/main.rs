#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    cognition_daemon::run()
}
