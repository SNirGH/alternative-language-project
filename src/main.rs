use std::cmp::Ordering;
use std::collections::HashMap;
use csv;
use regex::Regex;
use std::error::Error;
use std::fs::File;

#[derive(Debug)]
struct Cell {
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

impl Cell {
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

    fn check_empty(value: &str) -> Option<String> {
        if value.trim().is_empty() || value.trim() == "-" {
            None
        } else {
            Some(value.to_string())
        }
    }

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
}

fn main() -> Result<(), Box<dyn Error>> {
    let cells = Cell::read_csv("cells.csv")?;
    let highest_body_weight = Cell::highest_avg_body_weight_oem(&cells);
    println!("Highest Average Body Weight OEM: {}", highest_body_weight.unwrap());
    for cell in cells {
        println!("{:?}\n", cell);
    }

    Ok(())
}
