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
   pub  q_limit:usize
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
                q_limit:record[3].parse()?,};
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

#[test]
fn test_read_input_file(){
    if let Err(err) = read_input_file() {
        println!("error running example: {}", err);
    }
}