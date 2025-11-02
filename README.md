# Slint Rust on LuckFox Pico Ultra W (armv7l)

## Setup

1. Install Rust and Cargo if not already installed. Follow instructions at [rustup.rs](https://rustup.rs/).
2. Install the required Rust target for cross-compilation:
   ```bash
   rustup target add armv7-unknown-linux-gnueabihf
   ```
3. Install the ARM cross-compiler toolchain:
   ```bash
   sudo apt-get install gcc-arm-linux-gnueabihf
   ```
4. Clone this repository:
   ```bash
   git clone https://github.com/your_username/your_repository.git
   cd your_repository
    ```
6. Update the `.cargo/config.toml` file to ensure it points to the correct linker for your system.
7. Build the project for the ARM target:
   ```bash
   cargo build --release --target=armv7-unknown-linux-gnueabihf
   ```
8. Transfer the compiled binary from `target/armv7-unknown-linux-gnueabihf/release/` to your LuckFox Pico Ultra W device.
9. Run the application on the device:
   ```bash
   ./your_application_binary
   ```
## Notes
