use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use max_finder::find_max; // Import the find_max function from the max_finder library

#[test]
fn test_calorie_max() -> Result<(), Box<dyn Error>> {
    let csv_file = "cereal.csv"; // Update with your CSV file path
    let file = File::open(csv_file)?;

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

    // Find the maximum calorie value using the find_max function
    let max_calories = find_max(&calories_values);

    // Assert that the maximum calorie value is greater than or equal to 0 (or adjust this condition as needed)
    assert_eq!(max_calories, 160.0);

    Ok(())
}
