
use risc0_zkvm_host::{Prover, Receipt, Result};
use risc0_zkvm_serde::{from_slice, to_vec};
use risc0_rust_starter_methods_host::methods::{SOLVE_PATH, SOLVE_ID};
use std::fs;
use tempfile::tempdir;
use nalgebra::{SMatrix};

pub struct WithReceipt {
    receipt: Receipt,
}

fn run() -> anyhow::Result<()> {
    // Pick two numbers
    let a : u64 = 17;
    let b : u64 = 23;

    // Write the ID to a file, this is to work around the fact that the C++
    // prover API doesn't take ID's as buffers currently.
    let temp_dir = tempdir().unwrap();
    let id_path = temp_dir
        .path()
        .join("solve.id")
        .to_str()
        .unwrap()
        .to_string();
    fs::write(&id_path, SOLVE_ID).unwrap();

    // Multiply them inside the ZKP
    // First, we make the prover, loading the 'multiply' method
    let mut prover = Prover::new(&SOLVE_PATH, &id_path).unwrap();
    // Next we send a + b to the guest
    prover.add_input(to_vec(&a).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&b).unwrap().as_slice()).unwrap();
    // Run prover + generate receipt
    let receipt = prover.run()?;
    println!("Got receipt.");

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c : SMatrix::<u32, 3, 4> = from_slice(&receipt.get_journal_vec().expect("Journal vec").as_slice()).expect("Get matrix");
    // let c: u64 = from_slice(&receipt.get_journal_vec()?.as_slice())?;

    // Print an assertation
    println!("I know the factors of {}, and I can prove it!", c);
    
    // Here is where one would send 'receipt' over the network...
    
    // Verify receipt, panic if it's wrong
    receipt.verify(&id_path)?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Run failed: {}.", e);
    };
}

