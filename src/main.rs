slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    main_window.on_slider_position_changed(|index, value| {
        println!("Slider {index} changed to {value:.3}");
    });

    main_window.run()
}
