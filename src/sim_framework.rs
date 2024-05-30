use rand_distr::{Exp, Distribution};
use rand;

#[derive(Debug)]
pub struct Sim {
   pub mean_interarrival:f64,
   pub sim_time:f64,
   pub server_status:Status,
   pub q_limit: i64,
   pub  num_events: usize,
   pub  num_in_q: i64,
   pub time_last_event: f64,
   pub num_cust_delayed: i32,
   pub total_of_delays: f64,
   pub area_num_in_q: f64,
   pub area_server_status: f64,
   pub time_next_event: Vec<f64>,
    
}

impl Sim {

   pub fn initialize(mean_interarrival:f64, q_limit:i64)->Sim{


       let mut init = Sim {
            mean_interarrival: mean_interarrival,
            sim_time:0.0,
            server_status: Status::IDLE,
            q_limit: q_limit,
            num_events: 2,
            num_in_q:0,
            time_last_event:0.0,
            num_cust_delayed:0,
            total_of_delays:0.0,
            area_num_in_q:0.0,
            area_server_status:0.0,
            time_next_event:Vec::with_capacity(3)

        };
        /* Initialize event list. Since no customers are present,
         the departure (service completion) event is eliminated 
         from consideration. */

        let first_event:f64 = init.sim_time + draw_exp(init.mean_interarrival);
        init.time_next_event.push(first_event);
        init.time_next_event.push(1.0e+30);
    
       init
     
   }

   pub fn timing(&mut self) -> NextEventType{
     // let num_events: i32 = 2;
      let mut min_time_next_event:f64 = 1.0e+29;
      let mut next_event_type:NextEventType = NextEventType::ENDPROGRAM;

      for i in 0..self.num_events{
         if self.time_next_event[i] < min_time_next_event{
            min_time_next_event =  self.time_next_event[i];
            if i  == 0 {next_event_type = NextEventType::ARRIVE;}
            else if i  == 1 {next_event_type = NextEventType::DEPART;}
            else {println!("Unexpeted Behavior in sim.timing() {:?} index {}", next_event_type, i );}
         }
      }
      
      if next_event_type == NextEventType::ENDPROGRAM {
         /* The event list is empty, so stop the simulation. */
         println!("Simulation is complete, \
         the event list is empty at {}" , self.sim_time);
         panic!("Simulation Complete")
      }
      next_event_type
      }
      
  

  pub fn update_time_avg_stats(&mut self){
   /* Update area accumulators for time-average statistics. */

    let time_since_last_event:f64;

    /* Compute time since last event, and update last-event-time marker. */
   time_since_last_event = self.sim_time - self.time_last_event;
   self.time_last_event = self.sim_time;

   /* Update area under number-in-queue function. */
   self.area_num_in_q += self.num_in_q as f64 * time_since_last_event;

   /* Update area under server-busy indicator function. */
   self.area_server_status += self.server_status.float64() * time_since_last_event;

  }

  pub fn arrive(&mut self){
      let mut delay:f64;
   /* Schedule next arrival. */
      self.time_next_event[0] = self.sim_time + draw_exp(self.mean_interarrival);
   /* Check to see whether server is busy. */
   if self.server_status == Status::BUSY {
  
      /* Server is busy, so increment number of customers in queue. */
      self.num_in_q +=1;

      /* Check to see whether an overflow condition exists. */
      if self.num_in_q > self.q_limit {

      /* The queue has overflowed, so stop the simulation. */
      println!("Overflow of the of the array time_arrival at {}",self.sim_time);
     // process::exit(2);
    }
   }
  }

 pub fn depart(&mut self){

  } 
    
}

pub fn draw_exp(mean_interarrival:f64) ->f64{
let exp = Exp::new(mean_interarrival).unwrap();
let random_exp:f64 = exp.sample(&mut rand::thread_rng());
random_exp
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Status {
    IDLE,
    BUSY
}
impl Status {
   fn float64(&mut self) ->f64{
      match self {
         Self::IDLE => 0.0,
         Self::BUSY => 1.0
      }
   }

}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum NextEventType {
   ENDPROGRAM,
   ARRIVE,
   DEPART 
}


#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   #[should_panic]
   fn test_sim_timing() {
      let mean_interarrival: f64= 2.33;
      let q_limit:i64 = 200;
      let mut test_sim = Sim::initialize(mean_interarrival, q_limit);

      assert_eq!(test_sim.timing(), NextEventType::ARRIVE);
      
      test_sim.time_next_event[1] = 0.0033;
      assert_eq!(test_sim.timing(), NextEventType::DEPART);

      test_sim.time_next_event[0] = 1.0e+30;
      test_sim.time_next_event[1] = 1.0e+30;
      assert_eq!(test_sim.timing(), NextEventType::ENDPROGRAM);
 
}

#[test]
fn test_update_time_avg_stats(){
   //TODO
   assert_eq!(1,1);
}

#[test]
fn test_exp_draw(){
   let mut draw:Vec<f64> = Vec::new();
   let large_number = 990_000;
   for _ in 0..large_number{draw.push(draw_exp(2.33));}
   let draw_sum: f64 = draw.iter().sum();
   let draw_mean: f64 = 100.0 * draw_sum / large_number as f64;
   let truth_mean:f64 = 100.0 * 1.0 / 2.33;
   assert_eq!(draw_mean.round(), truth_mean.round())
}



}