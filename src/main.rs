use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use gpio_cdev::{Chip, LineHandle, LineRequestFlags};

slint::include_modules!();

const GPIO_CHIP_PATH: &str = "/dev/gpiochip0";
// One GPIO output per slider; adjust to match the target board's wiring.
const SLIDER_GPIO_LINES: &[u32] = &[0, 1];

struct GpioController {
    handles: Vec<LineHandle>,
}

impl GpioController {
    fn new(chip_path: &str, line_numbers: &[u32]) -> Result<Self, gpio_cdev::Error> {
        let mut chip = Chip::new(chip_path)?;
        let mut handles = Vec::with_capacity(line_numbers.len());

        for &line_number in line_numbers {
            let handle =
                chip.get_line(line_number)?
                    .request(LineRequestFlags::OUTPUT, 0, "slint-gpio")?;
            handles.push(handle);
        }

        Ok(Self { handles })
    }

    fn write_from_slider(
        &mut self,
        index: usize,
        slider_value: f32,
    ) -> Result<(), gpio_cdev::Error> {
        let Some(handle) = self.handles.get_mut(index) else {
            return Ok(());
        };

        let level = if slider_value >= 0.5 { 1 } else { 0 };
        handle.set_value(level)?;
        Ok(())
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let gpio_controller = match GpioController::new(GPIO_CHIP_PATH, SLIDER_GPIO_LINES) {
        Ok(controller) => Some(controller),
        Err(err) => {
            eprintln!("GPIO disabled: {err}");
            None
        }
    };
    let gpio_controller = Rc::new(RefCell::new(gpio_controller));

    let main_window = MainWindow::new()?;

    let gpio_for_slider = Rc::clone(&gpio_controller);
    main_window.on_slider_position_changed(move |index, value| {
        println!("Slider {index} changed to {value:.3}");

        let Ok(index) = usize::try_from(index) else {
            eprintln!("Invalid slider index received from UI: {index}");
            return;
        };

        let mut controller = gpio_for_slider.borrow_mut();
        if let Some(controller) = controller.as_mut() {
            if let Err(err) = controller.write_from_slider(index, value) {
                eprintln!("Failed to write to GPIO line {index}: {err}");
            }
        }
    });

    main_window.run()
}
