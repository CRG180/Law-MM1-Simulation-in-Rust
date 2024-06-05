
pub mod sim_framework;
use crate::sim_framework::{Sim, NextEventType};
use std::process;



fn main() {
    
/* Specify the number of events for the timing function. */


/* Read input parameters. */
   let mean_interarrival: f64= 1.00;
   let mean_service:f64 = 0.5000;
   let num_delays_required:i32 = 1000;
   let q_limit:usize = 200;

   let mut simulation: Sim = Sim::initialize(mean_interarrival,mean_service,q_limit);
   
    simulation.print_inputs();

   /* Run the simulation while more delays are still needed. */

    while simulation.num_cust_delayed < num_delays_required { 

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
    simulation.report();

}
