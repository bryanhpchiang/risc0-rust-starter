# risc0-rust-starter

This is a small example designed as a minimum starting project for the RiscZero
zero knowledge virtual machine.  In this example, we use zero knowledge proof
techniques to prove that we know the factors of some composite number N,
without revealing whtat the factors are.

# Building + running

Make sure you are using the nightly version of rust

```
rustup toolchain install nightly
rustup override set nightly
```

Run 

```
cargo run --release
```

# KNOWN ISSUES

Changes to guest code (i.e. methods/guest/src/bin/multiply.rs) will not properly
trigger cargo to rebuild.  One can 'poke' cargo by doing:

```
touch methods/build.rs
```



