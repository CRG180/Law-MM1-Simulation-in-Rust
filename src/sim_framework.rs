use rand_distr::{Exp, Distribution};
use rand::{self};
use std::process;
use crate::input_out_manager::{Record, OutputRecord};


#[derive(Debug)]
pub struct Sim {
   pub design_point:Option<usize>,
   pub run_number:Option<usize>,
   pub mean_interarrival:f64,
   pub mean_service:f64,
   pub num_delays_required: i32,
   pub q_limit: usize,
   pub num_cust_delayed: i32,
      sim_time:f64,
      server_status:Status,  
      num_events: usize,
      num_in_q: usize,
      time_last_event: f64,
      total_of_delays: f64,
      area_num_in_q: f64,
      area_server_status: f64,
      time_next_event: [f64;3],
      time_arrival: Vec<f64>
    
}

impl Sim {

   pub fn initialize(mean_interarrival:f64,mean_service:f64, 
                     q_limit:usize, num_delays_required:i32)->Sim {

       let mut init = Sim {
            design_point:None,
            run_number:None,
            mean_interarrival,
            mean_service,
            num_delays_required,
            q_limit,
            sim_time:0.0,
            server_status: Status::IDLE,
            num_events: 2,
            num_in_q:0,
            time_last_event:0.0,
            num_cust_delayed:0,
            total_of_delays:0.0,
            area_num_in_q:0.0,
            area_server_status:0.0,
            time_next_event:[0.0;3],
            time_arrival:vec![0.0;q_limit + 1]

        };
        /* Initialize event list. Since no customers are present,
         the departure (service completion) event is eliminated 
         from consideration. */
        let first_event:f64 = init.sim_time + draw_exp(init.mean_interarrival);
        init.time_next_event[1]= first_event;
        init.time_next_event[2] = 1.0e+30;
    
       init
     
   }

   pub fn initialize_from_record(record:&Record)->Sim{  

        let mut init = Sim::initialize(record.mean_interarrival, record.mean_service,
             record.q_limit,record.num_delays_required);
            init.design_point = Some(record.design_point);
            init.run_number = Some(record.design_point);

            init

   }

   pub fn timing(&mut self) -> NextEventType{

      let mut min_time_next_event:f64 = 1.0e+29;
      let mut next_event_type:NextEventType = NextEventType::ENDPROGRAM;
      
      //note rust for loop is not inclusive "add 1" to iterate between 1,2
      for i in 1..self.num_events+1{
         if self.time_next_event[i] < min_time_next_event{
            min_time_next_event =  self.time_next_event[i];
            if i  == 1 {next_event_type = NextEventType::ARRIVE;}
            if i  == 2 {next_event_type = NextEventType::DEPART;}
         }
      }
      
      if next_event_type == NextEventType::ENDPROGRAM {
         /* The event list is empty, so stop the simulation. */
         println!("Simulation is complete, \
         the event list is empty at {}" , self.sim_time);
         self.report();
         process::exit(1);
      }
      /* The event list is not empty, so advance the simulation clock. */
      self.sim_time = min_time_next_event;
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
      let delay:f64;
   /* Schedule next arrival. */
      self.time_next_event[1] = self.sim_time + draw_exp(self.mean_interarrival);
   /* Check to see whether server is busy. */
   if self.server_status == Status::BUSY {
  
      /* Server is busy, so increment number of customers in queue. */
      self.num_in_q +=1;

      /* Check to see whether an overflow condition exists. */
      if self.num_in_q > self.q_limit {

      /* The queue has overflowed, so stop the simulation. */
      println!("WARNING -- Overflow of the of the array time_arrival at {}",self.sim_time);
      self.report();
      process::exit(2);  
      }
      /* There is still room in the queue, so store the time of arrival of the 
      arriving customer at the (new) end of time_arrival. */

      self.time_arrival[self.num_in_q] = self.sim_time;
   }
   else {
   /* Server is idle, so arriving customer has a delay of zero.
   (The following two statements are for program clarity and do not 
      affect the results of the simulation.) */
      delay = 0.0;
      self.total_of_delays+= delay;
      /* Increment the number of customers delayed, and make server busy. */
      self.num_cust_delayed +=1;
      self.server_status=Status::BUSY;
      /* Schedule a departure (service completion). */
      self.time_next_event[2] = self.sim_time + draw_exp(self.mean_service)

   }
  }

 pub fn depart(&mut self){
   let delay:f64;
   /* Check to see whether the queue is empty. */
   if self.num_in_q == 0 {

   /* The queue is empty so make the server idle and 
   eliminate the departure (service completion) event 
   from consideration. */
   
      self.server_status = Status::IDLE;
      self.time_next_event[2] = 1.0e+30;
   }
   else {
      /* The queue is nonempty, so decrement 
      the number of customers in queue. */
      self.num_in_q -= 1;
      
      /* Compute the delay of the customer who is beginning 
      service and update the total delay accumulator. */
      delay = self.sim_time - self.time_arrival[1];
      self.total_of_delays += delay;
      
      /* Increment the number of customers delayed,
      and schedule departure. */
      self.num_cust_delayed +=1;
      self.time_next_event[2] = self.sim_time + draw_exp(self.mean_service);

      /* Move each customer in queue (if any) up one place. */
      for i in 1..self.num_in_q+1{
         self.time_arrival[i] = self.time_arrival[i+1];
      }
   }

  }

  pub fn report(&self)->OutputRecord{
      
      println!("Average delay in queue {} minutes.", self.total_of_delays / self.num_cust_delayed as f64);
      println!("Average number in queue {}.", self.area_num_in_q/self.sim_time);
      println!("Server utilization {}.",self.area_server_status / self.sim_time );
      println!("Time Simulation ended {} minutes.\n", self.sim_time);

   OutputRecord{
      design_point:self.design_point.unwrap(),
      run: self.run_number.unwrap(),
      mean_interarrival: self.mean_interarrival,
      num_delays_required:self.num_delays_required,
      q_limit:self.q_limit,
      mean_service:self.mean_service,
      average_delay:self.total_of_delays / self.num_cust_delayed as f64,
      average_number_in_queue:self.area_num_in_q/self.sim_time,
      server_utilization:self.area_server_status / self.sim_time,
      time_simulation_ended:self.sim_time}
  } 

  pub fn print_inputs(&self){
   /* Write report heading and input parameters. */
   println!("Single-server queueing system");
   println!("Mean interarrival time: {0} minutes", self.mean_interarrival);
   println!("Mean service time: {0}", self.mean_service);
   println!("Number of customers required: {0}", self.num_delays_required);
   println!("The Que Limit is set to {0}\n", self.q_limit);
  }
    
}

pub fn draw_exp(lamda_time:f64) ->f64{
let lambda = 1.0/lamda_time;
let exp = Exp::new(lambda).unwrap();
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
   fn test_sim_timing() {
      let mean_interarrival: f64= 2.33;
      let mean_service: f64 = 1.33;
      let q_limit:usize = 200;
      let num_delays_required:i32=1000;
      let mut test_sim = Sim::initialize(mean_interarrival,mean_service,q_limit, num_delays_required);

      assert_eq!(test_sim.timing(), NextEventType::ARRIVE);
      
      test_sim.time_next_event[2] = 0.0033;
      assert!(test_sim.time_next_event[1] > test_sim.time_next_event[2]);
      assert_eq!(test_sim.timing(), NextEventType::DEPART);
   }

#[test]
fn test_arrive() {
   let mean_interarrival: f64= 2.33;
   let mean_service: f64 = 1.33;
   let q_limit:usize = 200;
   let num_delays_required:i32=1000;
   let mut test_sim = Sim::initialize(mean_interarrival,mean_service,q_limit,num_delays_required);
   test_sim.timing();
   test_sim.arrive();

   assert_eq!(test_sim.server_status, Status::BUSY);    
   }

#[test]
fn test_depart(){
   //todo
   assert_eq!(1,1);
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
   let truth_mean:f64 = 100.0 * 2.33;
   assert_eq!(draw_mean.round(), truth_mean.round())
   }
}