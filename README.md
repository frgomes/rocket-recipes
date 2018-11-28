rocket-recipes
====

A collection of several small recipes containing routes on most common use cases, employing [Rocket](http://rocket.rs), a Web Framework developed in [Rust](http://rust-lang.org) programming language.

Environment setup
----

Setting up a development environment for Rust is simple: it's a matter of installing and running [rustup](https://rustup.rs/) and that's it!. If you are on a Unix-like machine, this is all you need:

```bash
#!/bin/bash
curl https://sh.rustup.rs -sSf | sh
```

Getting Started
----

> Make sure you ran [rustup](http://rustup.rs) as per instructions above.

Downloading and running these recipes is easy. Just follow the instructions below:

```bash
#!/bin/bash
mkdir -p $HOME/workspace
cd $HOME/workspace
git clone http://github.com/frgomes/rust-rocket-recipes
cd rust-rocket-recipes
cargo test
```
