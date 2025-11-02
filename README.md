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
5. Update the `.cargo/config.toml` file to ensure it points to the correct linker for your system.
6. Build the project for the ARM target:
   ```bash
   cargo build --release --target=armv7-unknown-linux-gnueabihf
   ```
7. Transfer the compiled binary from `target/armv7-unknown-linux-gnueabihf/release/` to your LuckFox Pico Ultra W device.
8. Run the application on the device:
   ```bash
   ./your_application_binary
   ```
## Notes
- Ensure that all dependencies are compatible with the ARM architecture.

On LuckFox Ubuntut 20.04, you might need to install additional libraries for Slint to work properly.

Increase CMA on cmdline:
```

sudo apt update
sudo apt install --reinstall \
  libgl1-mesa-dri \
  libegl1-mesa \
  libgles2 \
  libgbm1 \
  libdrm2 \
  mesa-utils

sudo apt install kmscube
sudo kmscube