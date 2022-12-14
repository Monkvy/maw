# maw

## Build
Run `cargo +nightly build` to build the os.
The build files can be found at `./target/`.
If you dont have the rust nightly toolchain,
run `rustup toolchain install nightly`.

Alternatively, you can set the nightly toolchain 
for this project by executing this inside your terminal:
`rustup override set nightly`.
Now, you just have to type `cargo build` to build.
To switch back to the stable toolchain, execute
`rustup override set stable`.

## Run
To run this operating system on qemu, just run
`cargo run`.

### Requirements
* [rustup](https://www.rust-lang.org/tools/install) to compile the os
* [qemu](https://www.qemu.org/download/) to emulate the os
