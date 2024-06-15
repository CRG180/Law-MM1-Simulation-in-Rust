
pub mod sim_framework;
pub mod input_out_manager;
use input_out_manager::OutputRecordContainer;
use crate::sim_framework::Sim;
use crate::input_out_manager::{read_input_file,write_output_file};
use clap::Parser;
use std::ffi::OsString;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    input_csv_path: OsString,
    #[arg(short, long, default_value = "mm1_output_data.csv")]
    out_csv_path:OsString,
    #[arg(short, long, default_value = "1")]
    runs_per_design_point: usize
}

fn main() {
  let args = Args::parse();
  
    
/*Read data from input file csv. */
   let file_inputs = read_input_file(args.input_csv_path,
  args.runs_per_design_point).unwrap();
   let mut sim_outputs = OutputRecordContainer::new(args.out_csv_path);

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

