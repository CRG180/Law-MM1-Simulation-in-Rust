use std::io::stdout;
use std::{
    env,
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


pub fn read_input_file()-> Result<Vec<Record>, Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut records:Vec<Record>=vec![];
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record: csv::StringRecord = result?;
        
        let line: Record = Record{
                mean_interarrival:record[0].parse()?,
                mean_service:record[1].parse()?,
                num_delays_required:record[2].parse()?,
                q_limit:record[3].parse()?,
                design_point:record[4].parse()?,
                run_number:record[5].parse()?};
        records.push(line);
        
    }

    Ok(records)
}


fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

pub fn write_output_file(record:OutputRecord) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(stdout());
    wtr.serialize(record)?;
    wtr.flush()?;

    Ok(())
}



#[test]
fn test_read_input_file(){
    if let Err(err) = read_input_file() {
        println!("error running example: {}", err);
    }
}