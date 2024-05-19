/*
- Generating data 100x faster with Rust — Part I
    - https://itnext.io/generating-data-100x-faster-with-rust-part-i-713182122d53
        - https://github.com/aprxi/faster-data-generation
*/

use std::time::Instant;
use fake::Dummy;
use fake::{Fake, Faker};

use fake::faker::name::en::*;
use fake::faker::internet::en::*;
use fake::faker::company::en::*;
use fake::faker::phone_number::en::*;

use chrono::prelude::*;

const NO_ROWS: usize = 10000;

// const DATE_FORMAT_STR: &'static str = "%Y-%m-%d][%H:%M:%S";


#[derive(Debug, Dummy)]
struct TableColumns {
    #[dummy(faker = "(1000..9999999999999, NO_ROWS)")]
    pub ids: Vec<i64>,

    #[dummy(faker = "(FirstName(), NO_ROWS)")]
    pub first_names: Vec<String>,

    #[dummy(faker = "(LastName(), NO_ROWS)")]
    pub last_names: Vec<String>,

    #[dummy(faker = "(FreeEmail(), NO_ROWS)")]
    pub emails: Vec<String>,

    #[dummy(faker = "(CompanyName(), NO_ROWS)")]
    pub companies: Vec<String>,

    #[dummy(faker = "(PhoneNumber(), NO_ROWS)")]
    pub phone_numbers: Vec<String>,
}


fn generate_table() {
    let start_time = Instant::now();
    let table: TableColumns = Faker.fake();
    let elapsed = start_time.elapsed().as_secs_f64();

    let num_records = 3;

    println!("First {} records:", num_records);
    for i in 0..num_records {
        println!(
        "Record {}: {{ id: {}, first_name: \"{}\", last_name: \"{}\",\
         email: \"{}\", company: \"{}\", phone_number: \"{}\" }}", 
            i + 1,
            table.ids[i], 
            table.first_names[i], 
            table.last_names[i], 
            table.emails[i], 
            table.companies[i], 
            table.phone_numbers[i]
        );
    }

    println!("Time taken to generate {NO_ROWS} people:");
    println!("--- {:.3} seconds ---", elapsed);
}

fn main() {
    // let start_time  =OffsetDateTime::now_utc();
    let start_time = Instant::now();
    let dt_start = Utc::now();
    println!("Inicia! {}", dt_start);
    generate_table();
    println!("Fin!");
    println!("<-------------------------->");
    let elapsed = start_time.elapsed().as_secs_f64();
    println!("--- {:.3} seconds ---", elapsed);
    
    let dt_end = Utc::now();
    println!("Inició: {} terminó: {}", dt_start, dt_end);
}
