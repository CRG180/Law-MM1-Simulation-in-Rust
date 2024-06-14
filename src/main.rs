
pub mod sim_framework;
pub mod input_out_manager;
use input_out_manager::OutputRecordContainer;

use crate::sim_framework::Sim;
use crate::input_out_manager::{read_input_file,write_output_file};

fn main() {
    
/*Read data from input file csv. */
   let file_inputs = read_input_file().unwrap();
   let mut sim_outputs = OutputRecordContainer::new();
   //let first_run = &file_inputs[0];

  for i in file_inputs{
/* initialize from csv ead input parameters. */
   let mut simulation: Sim = Sim::initialize_from_record(&i);
      //simulation.print_inputs();
      simulation.run_simulation();
    /* Invoke the report generator and end the simulation. */
    let output = simulation.report();
    
    sim_outputs.records.push(output);
    
  }

  let _ = write_output_file(sim_outputs);
  
}

