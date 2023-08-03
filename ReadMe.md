# RustMark

A simple and lightweight CPU benchmarking tool based on Rust.
In each loop every thread calculates a factorial(20). 

<!-- toc -->

* [General usage](#general-usage)
* [Meta](#meta)

<!-- tocstop -->

## General usage

When Rust is installed and the repo cloned, open the terminal and run the following command from the repo's directory:

```sh
cargo run --release
```

The command line utility also accepts an argument to change the number of iterations:

```sh
cargo run --release 400000000 # 400000000 is the default value
```

We are working on providing binaries to be executable from Linux, Windows and MacOS.

## Meta

Creator: Thomas Meißner – [LinkedIn](https://www.linkedin.com/in/thomas-mei%C3%9Fner-m-a-3808b346)
