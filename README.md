# rust-learning
Just toying with rust, nothing to see here :)

See more:
https://www.rust-lang.org/
https://www.tutorialspoint.com/rust/rust_environment_setup.htm
https://docs.rs/
https://docs.rs/sdl2/

# Install
`curl https://sh.rustup.rs -sSf | sh`

# Setup environment
`source $HOME/.cargo/env`

# Emacs install
`M-x package-install rust-mode`

See more: https://github.com/rust-lang/rust-mode

# Compile
`rustc whatever.rs`

# Execute
`./whatever`

# Crates
When using some dependency the cargo tool seemed most easy to use:
https://www.tutorialspoint.com/rust/rust_package_manager.htm

To create a binary/executable
`cargo new <name> --bin`

To create a library
`cargo new <name> --lib`

To build just edit the Cargo.toml file and execute
`cargo build`

To run
`cargo run`

Or
~~~
cd target/debug
./binary-name
~~~
