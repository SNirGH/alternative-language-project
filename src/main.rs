mod test;

use std::cmp::Ordering;
use std::collections::HashMap;
use csv;
use regex::Regex;
use std::error::Error;
use std::fs::File;

#[derive(Debug)]
// Create a struct called Cell; create the variables and their respective types.
pub struct Cell {
    oem: Option<String>,
    model: Option<String>,
    launch_announced: Option<u32>,
    launch_status: Option<String>,
    body_dimensions: Option<String>,
    body_weight: Option<f32>,
    body_sim: Option<String>,
    display_type: Option<String>,
    display_size: Option<f32>,
    display_resolution: Option<String>,
    features_sensors: Option<String>,
    platform_os: Option<String>,
}

// Implements cell.
impl Cell {
    /*
     Initializes all the variable values
     Runtime: O(1)
     */
    fn new() -> Cell {
        Cell {
            oem: None,
            model: None,
            launch_announced: None,
            launch_status: None,
            body_dimensions: None,
            body_weight: None,
            body_sim: None,
            display_type: None,
            display_size: None,
            display_resolution: None,
            features_sensors: None,
            platform_os: None,
        }
    }

    /*
        Read the CSV file using the csv library.
        Return a vector of each line of the CSV file.
        Each cell corresponds to a value from the struct variables.

        Runtime: O(n)
     */
    fn read_csv(filename: &str) -> Result<Vec<Cell>, Box<dyn Error>> {
        let file = File::open(filename)?;
        let mut reader = csv::Reader::from_reader(file);
        let mut cells = Vec::new();

        let regex_year = Regex::new(r"\b(\d{4})\b").unwrap();
        let regex_numeric = Regex::new(r"\d+(\.\d+)?").unwrap();

        for result in reader.records() {
            let record = result?;
            let mut cell = Cell::new();

            cell.oem = Some(record.get(0).unwrap_or_default().to_string());
            cell.model = Some(record.get(1).unwrap_or_default().to_string());

            if let Some(capture) = regex_year.captures(record.get(2).unwrap_or_default()) {
                cell.launch_announced = Some(capture[0].parse::<u32>().unwrap());
            } else {
                cell.launch_announced = None;
            }

            let status = record.get(3).unwrap_or_default().to_string();

            if let Some(capture) = regex_year.captures(&status) {
                cell.launch_status = Some(capture[1].to_string());
            } else {
                cell.launch_status = Some(status);
            }

            if let Some(weight_str) = record.get(5) {
                if let Some(capture) = regex_numeric.captures(weight_str) {
                    if let Ok(weight) = capture[0].parse::<f32>() {
                        cell.body_weight = Some(weight);
                    }
                }
            }

            cell.body_dimensions = Self::check_empty(record.get(4).unwrap_or_default());
            cell.body_sim = Self::check_empty(record.get(6).unwrap_or_default());
            cell.display_type = Self::check_empty(record.get(7).unwrap_or_default());

            if let Some(size_str) = record.get(8) {
                if let Some(capture) = regex_numeric.captures(size_str) {
                    if let Ok(size) = capture[0].parse::<f32>() {
                        cell.display_size = Some(size);
                    }
                }
            }

            cell.display_resolution = Self::check_empty(record.get(9).unwrap_or_default());
            cell.features_sensors = Self::check_empty(record.get(10).unwrap_or_default());
            cell.platform_os = Self::check_empty(record.get(11).unwrap_or_default());

            cells.push(cell);
        }

        Ok(cells)
    }

    // Checks if the value passed in is '-' or blank. If yes, replace it with the value None
    fn check_empty(value: &str) -> Option<String> {
        if value.trim().is_empty() || value.trim() == "-" {
            None
        } else {
            Some(value.to_string())
        }
    }

    /*
        Function used to check which year after the year 1999 released the most amount of phones.

        Runtime: O(n)
     */
    fn year_most_phones_launched__after_year(cells: &[Cell]) -> Option<u32> {
        let mut year_counts: HashMap<u32, usize> = HashMap::new();

        for cell in cells {
            if let Some(year) = cell.launch_announced {
                if year > 1999 {
                    let count = year_counts.entry(year).or_insert(0);
                    *count += 1;
                }
            }
        }

        year_counts.into_iter().max_by_key(|&(_, count)| count).map(|(year, _)| year)
    }

    /*
        Function used to check which phone oem had the highest average weight.

        Runtime: O(n)
     */
    fn highest_avg_body_weight_oem(cells: &[Cell]) -> Option<String> {
        let mut oem_weights: HashMap<String, (f32, usize)> = HashMap::new();

        for cell in cells {
            if let Some(oem) = &cell.oem {
                if let Some(weight) = cell.body_weight {
                    let entry = oem_weights.entry(oem.clone()).or_insert((0.0, 0));
                    entry.0 += weight;
                    entry.1 += 1;
                }
            }
        }

        if oem_weights.is_empty() {
            return None;
        }

        let (oem, (_, _)) = oem_weights
            .into_iter()
            .max_by(|(_, (avg1, count1)), (_, (avg2, count2))| {
                (avg1 / *count1 as f32)
                    .partial_cmp(&(avg2 / *count2 as f32))
                    .unwrap_or(Ordering::Equal)
            })?;

        Some(oem)
    }

    /*
        Function used which phones had a single sensor. This is determined by splitting the cell by ','

        Runtime: O(n)
     */
    fn count_phones_with_single_sensor(cells: &[Cell]) -> usize {
        let mut count = 0;

        for cell in cells {
            if let Some(sensors_str) = &cell.features_sensors {
                let sensors = sensors_str.split(',').map(|s| s.trim()).collect::<Vec<&str>>();
                if sensors.len() == 1 {
                    count += 1;
                }
            }
        }

        count
    }

    /*
        Function used to check which phones were announced one year, but released on a different year.

        Runtime: O(n)
     */
    fn phones_announced_in_one_year_released_in_another(cells: &[Cell]) -> Vec<(String, String)> {
        let mut mismatched_years = Vec::new();

        for cell in cells {
            if let (Some(announced_year), Some(released_year)) = (cell.launch_announced, &cell.launch_status) {
                if announced_year != released_year.parse().unwrap_or_default() {
                    if let (Some(oem), Some(model)) = (&cell.oem, &cell.model) {
                        mismatched_years.push((oem.clone(), model.clone()));
                    }
                }
            }
        }

        mismatched_years
    }

    /*
        Function used to check which oem was the most common in the file.

        Runtime: O(n)
     */
    fn most_common_oem(cells: &[Cell]) -> Option<String> {
        let mut oem_counts: HashMap<String, usize> = HashMap::new();

        for cell in cells {
            if let Some(oem) = &cell.oem {
                let count = oem_counts.entry(oem.clone()).or_insert(0);
                *count += 1;
            }
        }

        oem_counts.into_iter().max_by_key(|&(_, count)| count).map(|(oem, _)| oem)
    }

    /*
        Function used to check what the most common display size is.

        Runtime: O(n)
     */
    fn most_common_display_size(cells: &[Cell]) -> Option<String> {
        let mut oem_counts: HashMap<String, usize> = HashMap::new();

        for cell in cells {
            if let Some(display_size) = &cell.display_size {
                let count = oem_counts.entry(display_size.clone().to_string()).or_insert(0);
                *count += 1;
            }
        }

        oem_counts.into_iter().max_by_key(|&(_, count)| count).map(|(oem, _)| oem)
    }

    /*
        Function used to the mean (average) body weight.

        Runtime: O(n)
     */
    fn mean_body_weight(cells: &[Cell]) -> Option<f32> {
        let mut sum = 0.0;
        let mut count = 0;

        for cell in cells {
            if let Some(weight) = cell.body_weight {
                sum += weight;
                count += 1;
            }
        }

        if count > 0 {
            Some(sum / count as f32)
        } else {
            None
        }
    }

    /*
        Function used to check the median body weight throughout the file.

        Runtime: O(n)
     */
    fn median_body_weight(cells: &[Cell]) -> Option<f32> {
        let mut weights: Vec<f32> = cells.iter().filter_map(|cell| cell.body_weight).collect();
        weights.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

        let len = weights.len();
        if len == 0 {
            None
        } else if len % 2 == 0 {
            Some((weights[len / 2 - 1] + weights[len / 2]) / 2.0)
        } else {
            Some(weights[len / 2])
        }
    }

    /*
        Create a new Cell in the vector. This does not affect the file itself.

        Runtime: O(n)
     */
    fn insert_cell(cells: &mut Vec<Cell>, index: usize, new_cell: Cell) {
        if index <= cells.len() {
            cells.insert(index, new_cell);
        } else {
            println!("Index out of bounds.");
        }
    }

    /*
        Modify an existing Cell within the vector.

        Runtime: O(1)
     */
    fn modify_cell(cells: &mut Vec<Cell>, index: usize, modified_cell: Cell) {
        if let Some(cell) = cells.get_mut(index) {
            *cell = modified_cell;
        } else {
            println!("Index out of bounds.");
        }
    }

    /*
        Delete an existing Cell within the vector.

        Runtime: O(n)
     */
    fn delete_cell(cells: &mut Vec<Cell>, index: usize) {
        if index < cells.len() {
            cells.remove(index);
        } else {
            println!("Index out of bounds.");
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cells = Cell::read_csv("cells.csv")?;
    let most_appearances = Cell::most_common_oem(&cells);
    let most_common_display_size = Cell::most_common_display_size(&cells);
    let highest_body_weight = Cell::highest_avg_body_weight_oem(&cells);

    let phones_with_mismatched_years = Cell::phones_announced_in_one_year_released_in_another(&cells);

    if phones_with_mismatched_years.is_empty() {
        println!("No phones were announced in one year and released in another.");
    } else {
        println!("Phones announced in one year and released in another:");
        for (oem, model) in phones_with_mismatched_years {
            println!("OEM: {}, Model: {}", oem, model);
        }
    }

    let phones_with_single_sensor = Cell::count_phones_with_single_sensor(&cells);
    println!("Phones with only one feature sensor: {}", phones_with_single_sensor);

    if let Some(oem) = most_appearances {
        println!("Most Common OEM: {}", oem);
    } else {
        println!("No data found.");
    }

    if let Some(display_size) = most_common_display_size {
        println!("Most Common Display Size: {}", display_size);
    } else {
        println!("None");
    }

    if let Some(mean) = Cell::mean_body_weight(&cells) {
        println!("Mean Body Weight: {:.2}", mean);
    } else {
        println!("None");
    }

    if let Some(median) = Cell::median_body_weight(&cells) {
        println!("Median Body Weight: {:.2}", median);
    } else {
        println!("None");
    }

    let new_cell = Cell {
        oem: Some("New OEM".to_string()),
        model: Some("New Model".to_string()),
        launch_announced: Some(2024),
        launch_status: Some("New".to_string()),
        body_dimensions: Some("New Dimensions".to_string()),
        body_weight: Some(150.0),
        body_sim: Some("New SIM".to_string()),
        display_type: Some("New Type".to_string()),
        display_size: Some(6.0),
        display_resolution: Some("New Resolution".to_string()),
        features_sensors: Some("New Sensors".to_string()),
        platform_os: Some("New OS".to_string()),
    };

    // let modified = Cell {
    //     oem: Some("New OEM".to_string()),
    //     model: Some("New Model".to_string()),
    //     launch_announced: Some(2024),
    //     launch_status: Some("New".to_string()),
    //     body_dimensions: Some("New Dimensions".to_string()),
    //     body_weight: Some(150.0),
    //     body_sim: Some("New SIM".to_string()),
    //     display_type: Some("New Type".to_string()),
    //     display_size: Some(6.0),
    //     display_resolution: Some("New Resolution".to_string()),
    //     features_sensors: Some("New Sensors".to_string()),
    //     platform_os: Some("New OS".to_string()),
    // };
    //
    // Cell::insert_cell(&mut cells, 1, new_cell);
    //
    // Cell::modify_cell(&mut cells, 0, modified);
    //
    // Cell::delete_cell(&mut cells, 2);

    if let Some(year) = Cell::year_most_phones_launched__after_year(&cells) {
        println!("Year with most phones launched after 1999: {}", year);
    } else {
        println!("None");
    }

    println!("Highest Average Body Weight OEM: {}", highest_body_weight.unwrap());
    for cell in cells {
        println!("{:?}\n", cell);
    }

    Ok(())
}
