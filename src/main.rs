use csv::ReaderBuilder;
use max_finder::find_max; // Import the find_max function from the max_finder library
use std::error::Error;
use std::fs::File;
use std::process::Command;
use std::time::Instant;
use sys_info::mem_info;

fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("ps")
        .arg("-o")
        .arg("%cpu")
        .arg("-p")
        .arg(format!("{}", std::process::id()))
        .output()
        .expect("Failed to execute ps command");

    let usage = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = usage.split('\n').collect();
    if lines.len() >= 2 {
        let usage_str = lines[1].trim();
        let usage_float: Result<f32, _> = usage_str.parse();
        match usage_float {
            Ok(usage) => println!("CPU Usage: {:.2}%", usage),
            Err(_) => println!("Failed to parse CPU usage"),
        }
    } else {
        println!("Failed to get CPU usage");
    }
    // Record the start time
    let start_time = Instant::now();
    // Load the CSV file
    let csv_file = "cereal.csv"; // Read in cereal data
    let file = File::open(csv_file)?;

    // Create a CSV reader
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_reader(file);

    let mut calories_values: Vec<f64> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let calories: f64 = record[3].parse()?; // Change index to match your data
        calories_values.push(calories);
    }

    // Find and print the maximum calorie value using the find_max function
    let max_calories = find_max(&calories_values);
    println!("Max Calories: {}", max_calories);

    let end_time = Instant::now();

    // Calculate the elapsed time and resource usage
    let elapsed_time = end_time.duration_since(start_time);
    let mem_info = mem_info().unwrap();

    println!(
        "Memory Usage: {}%",
        mem_info.total.saturating_sub(mem_info.avail) as f32 / mem_info.total as f32 * 100.0
    );
    println!("Elapsed time: {:?}", elapsed_time);

    Ok(())
}
