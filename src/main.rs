
pub mod sim_framework;
pub mod input_out_manager;
use crate::sim_framework::{Sim, NextEventType};
use crate::input_out_manager::{read_input_file,write_output_file};
use std::process;


fn main() {
    
/*Read data from input file csv. */
   let file_inputs = read_input_file().unwrap();
   let first_run = &file_inputs[0];

/* initialize from csv ead input parameters. */
   let mut simulation: Sim = Sim::initialize_from_record(first_run);
   
    simulation.print_inputs();

   /* Run the simulation while more delays are still needed. */

    while simulation.num_cust_delayed < simulation.num_delays_required { 

     // println!("number in the que {} at {} min -- {} Num Cust Delayed", simulation.num_in_q, simulation.sim_time, simulation.num_cust_delayed);
      
      /* Determine the next event. */
     let next_event_type: NextEventType = simulation.timing();
      
      simulation.update_time_avg_stats();

      match next_event_type {

         NextEventType::ARRIVE => {
            simulation.arrive();
         }
         NextEventType::DEPART => {
            simulation.depart();
         }
         NextEventType::ENDPROGRAM =>{
            process::exit(1);
         }
          
      }

    }
    /* Invoke the report generator and end the simulation. */
    let output = simulation.report();

    let _ = write_output_file(output);

}
