sixtyfps::include_modules!();
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Keyboard {}

fn main() {
    // Get the components for the window
    let main_window = Configurator::new();
    let main_window_weak = main_window.as_weak();
    let main_window_clone = main_window_weak.clone();

    // Wwhen the main configure button is clicked
    main_window.on_configure(move || {
        let status = main_window_clone.unwrap().get_edit_mode();
        if status {
            main_window_clone.unwrap().set_viewport_height(0.0);
        } else {
            main_window_clone.unwrap().set_viewport_height(600.0);
        }
        main_window_clone.unwrap().set_edit_mode(!status);
    });
    let main_window_clone = main_window_weak.clone();
    main_window.on_reset(move || {
        main_window_clone.unwrap().set_edit_mode(false);
    });

    // Run the window
    main_window.run();
}
