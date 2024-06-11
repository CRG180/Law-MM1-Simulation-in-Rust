
pub mod sim_framework;
pub mod input_out_manager;
use crate::sim_framework::Sim;
use crate::input_out_manager::{read_input_file,write_output_file};

fn main() {
    
/*Read data from input file csv. */
   let file_inputs = read_input_file().unwrap();
   let first_run = &file_inputs[0];

/* initialize from csv ead input parameters. */
   let mut simulation: Sim = Sim::initialize_from_record(first_run);
      simulation.print_inputs();
      simulation.run_simulation();
    /* Invoke the report generator and end the simulation. */
    let output = simulation.report();

    let _ = write_output_file(output);

}
