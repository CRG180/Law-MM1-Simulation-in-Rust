
pub mod sim_framework;
use crate::sim_framework::{Sim, NextEventType};
use std::process;



fn main() {
    
/* Specify the number of events for the timing function. */


/* Read input parameters. */
   let mean_interarrival: f64= 2.33;
   let mean_service:f64 = 1.22;
   let num_delays_required:i32 = 3;
   let q_limit:usize = 200;


/* Write report heading and input parameters. */
   println!("Single-server queueing system");
   println!("Mean interarrival time: {mean_interarrival} minutes");
   println!("Mean service time: {mean_service}");
   println!("Number of customers {num_delays_required}");
   println!("The Que Limit is set to {q_limit}");

   let mut simulation: Sim = Sim::initialize(mean_interarrival,mean_service,q_limit);

   /* Run the simulation while more delays are still needed. */

    while simulation.num_cust_delayed < num_delays_required { 

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
            //panic!("Program eneded in the the match!")
            process::exit(1);
         }
          
      }

    }

}
