# livesplit-refunct

An auto splitter for Refunct, created with Auto Splitting Runtime (ASR).

Supports auto-start, auto-split and auto-reset.

#### Auto-start
- The autosplitter will automatically start upon **pressing New Game**.
#### Auto-split
- The autosplitter will automatically split upon **pressing a button** or **collecting a cube**. The specific settings for this can be found within the autosplitter's settings.
#### Auto-reset
- The autosplitter will automatically reset upon **pressing New Game**.

## Installation
To install this autosplitter, open your LiveSplit's layout settings (Right click -> `Edit Layout`) and click the + (plus) button.
![Edit Layout](https://i.imgur.com/zvjAfk9.png)
![Add component](https://i.imgur.com/Ho6OuTr.png)

From there, hover over `Control` and click `Auto Splitting Runtime`.

![Add Auto Splitting Runtime Component](https://i.imgur.com/OTwgQyg.png)

You can then modify the settings to your liking by clicking `Layout` and finding the tab of the Auto Splitting Runtime component you just added. From there, you must locate the `.wasm` file downloaded from the repository. 

Click `Browse...`, and then find the `.wasm` file and double-click on it. You should now see something like this:

![livesplit-refunct settings](https://i.imgur.com/91AAj1H.png)

Now all that's left to do is to load up Refunct and start a run, and it should work flawlessly.

## Compilation

This autosplitter is written in Rust. In order to compile it, you need to
install the Rust compiler: [Install Rust](https://www.rust-lang.org/tools/install).

Afterwards install the WebAssembly target:
```sh
rustup target add wasm32-unknown-unknown --toolchain nightly
```

The autosplitter can now be compiled:
```sh
cargo b --release
```

The autosplitter is then available at:
```
target/wasm32-unknown-unknown/release/livesplit-refunct.wasm
```

Make sure to look into the [API documentation](https://livesplit.org/asr/asr/) for the `asr` crate.

## Development

You can use the [debugger](https://github.com/LiveSplit/asr-debugger) while
developing the auto splitter to more easily see the log messages, statistics,
dump memory, step through the code and more.

The repository comes with preconfigured Visual Studio Code tasks. During
development it is recommended to use the `Debug Auto Splitter` launch action to
run the `asr-debugger`. You need to install the `CodeLLDB` extension to run it.

You can then use the `Build Auto Splitter (Debug)` task to manually build the
auto splitter. This will automatically hot reload the auto splitter in the
`asr-debugger`.

Alternatively you can install the [`cargo
watch`](https://github.com/watchexec/cargo-watch?tab=readme-ov-file#install)
subcommand and run the `Watch Auto Splitter` task for it to automatically build
when you save your changes.

The debugger is able to step through the code. You can set breakpoints in VSCode
and it should stop there when the breakpoint is hit. Inspecting variables may
not work all the time.

