# livesplit-refunct

An auto splitter for Refunct, created with LiveSplit's `asr`.

## Features

* Starts the timer when "New Game" is pressed.
* Splits when the golden button is hit at the end of the game. Additional splits are available in the settings:
  * upon stepping on any button,
  * upon collecting any cube.
* Resets the timer when "New Game" is pressed.

## Installation

1. Open LiveSplit's layout settings (right-click > `Edit Layout...`).
2. Add the `asr` component by clicking the `+` (plus) button and navigating to `Control` > `Auto Splitting Runtime`.
3. Point the component to the auto splitter's `.wasm` file by opening the `asr` component's settings and changing the "Script Path".
   * Download the `.wasm` file from the most recent release on the right of this repository.
4. Adjust the settings to your liking.

## Building `livesplit-refunct`

1. If necessary, [install Rust](https://rust-lang.org/tools/install) on your machine.
   * On Unix: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`  
   * On Windows: `winget install Rustlang.Rustup`
2. Install the `nightly` toolchain:
   ```
   rustup toolchain install nightly
   ```
3. Install the `rust-src` component:
   ```
   rustup component add rust-src --toolchain nightly
   ```
4. Install the `wasm32` target:
   ```
   rustup target add wasm32-unknown-unknown --toolchain nightly
   ```
5. Build the auto splitter:
   ```
   cargo b --release
   ```
6. Find the output at:
   ```
   target/wasm32-unknown-unknown/release/livesplit-refunct.wasm
   ```

## Development

LiveSplit's `asr` crate's documentation can be found at https://livesplit.org/asr/asr.

You can use the [asr-debugger](https://github.com/LiveSplit/asr-debugger) while
developing the auto splitter to more easily see log messages, statistics,
dump memory, step through the code and more. You can set breakpoints in VSCode
and execution should halt when the breakpoint is hit. Inspecting variables may
not work all the time.
