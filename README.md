
# Law M/M/1 Simulation in Rust

This project aims to rewrite Law's M/M/1 simulation model, originally written in C, in Rust. The original code can be found in the textbook "Simulation Modeling and Analysis 5th Edition" by Dr. Averill M. Law.

The purpose of this project is to practice Rust and review concepts from my graduate education.

To run the program after cloning the repository, you can just enter the path to the test input file, provide an optional path for the output CSV, and optional argument for the number replication runs for each of design points (rows in the csv files).  
```
cargo run data/test.csv -o mm1_out_putfile.csv -r 1
```
