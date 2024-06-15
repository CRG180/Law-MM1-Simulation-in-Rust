
use std::{
    error::Error,
    ffi::OsString,
    fs::File,
};

#[derive(Debug)]
pub  struct Record {
   pub  mean_interarrival: f64,
   pub  mean_service:f64,
   pub  num_delays_required:i32,
   pub  q_limit:usize,
   pub  design_point:usize,
   pub  run_number:usize
}

#[derive(serde::Serialize)]
pub struct OutputRecord{
    pub design_point: usize,
    pub run: usize,
    pub mean_interarrival: f64,
    pub mean_service:f64,
    pub num_delays_required:i32,
    pub q_limit:usize,
    pub average_delay :f64,
    pub average_number_in_queue:f64,
    pub server_utilization:f64,
    pub time_simulation_ended:f64
}

pub struct OutputRecordContainer{
    pub records:Vec<OutputRecord>,
    pub output_path: OsString
}
 impl OutputRecordContainer {
    pub fn new(output_path:OsString)->Self {
        Self{
            records:vec![],
            output_path
        }
    }
     
 }

pub fn read_input_file(input_path:OsString, runs_per_design_point:usize)-> Result<Vec<Record>, Box<dyn Error>> {
    let file_path = input_path;
    let file = File::open(file_path)?;
    let mut records:Vec<Record>=vec![];
    let mut rdr = csv::Reader::from_reader(file);
    let run_number = runs_per_design_point;
    

    for result in rdr.records() {
        let record: csv::StringRecord = result?;
        for counter  in 1..run_number+1 {
        let line: Record = Record{
                mean_interarrival:record[0].parse()?,
                mean_service:record[1].parse()?,
                num_delays_required:record[2].parse()?,
                q_limit:record[3].parse()?,
                design_point:record[4].parse()?,
                run_number:counter};
        records.push(line);

        }   
    }

    Ok(records)
}


pub fn write_output_file(sim_out_puts:OutputRecordContainer) -> Result<(), Box<dyn Error>> {
    //let mut wtr = csv::Writer::from_writer(stdout());
    let mut wtr = csv::Writer::from_path(sim_out_puts.output_path)?;
    for record in sim_out_puts.records{
    wtr.serialize(record)?;}
    wtr.flush()?;

    Ok(())
}

#[test]
fn test_read_input_file(){
    let os_path = OsString::from("data/test.csv");
    if let Err(err) = read_input_file(os_path,5) {
        println!("error running example: {}", err);
    }
}