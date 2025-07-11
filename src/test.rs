#[cfg(test)]
mod tests {
    use crate::Cell;
    use super::*;

    // Test if the file being read is empty.
    #[test]
    fn check_for_empty_file() {
        let file = "cells_test.csv";

        match Cell::read_csv(file) {
            Ok(cells) => {
                assert!(!cells.is_empty(), "{} is empty", file);
            }
            Err(err) => {
                panic!("Error reading file: {:?}", err);
            }
        }
    }

    // Test to check if each transformation is in its final form.
    #[test]
    fn test_display_size_type() {
        let filename = "cells_test.csv";
        match Cell::read_csv(filename) {
            Ok(cells) => {
                for cell in cells {
                    if let Some(launch_announced) = cell.launch_announced {
                        assert_eq!(launch_announced as u32, launch_announced, "Launch announced is not a u32: {:?}", launch_announced);
                    } else {
                        assert!(cell.body_weight.is_none(), "Display size is not None: {:?}", cell.body_weight);
                    }
                    if let Some(body_weight) = cell.body_weight {
                        assert!(body_weight.is_finite(), "Display size is not a float: {:?}", body_weight);
                    } else {
                        assert!(cell.body_weight.is_none(), "Display size is not None: {:?}", cell.body_weight);
                    }
                    if let Some(display_size) = cell.display_size {
                        assert!(display_size.is_finite(), "Display size is not a float: {:?}", display_size);
                    } else {
                        assert!(cell.display_size.is_none(), "Display size is not None: {:?}", cell.display_size);
                    }
                }
            }
            Err(err) => {
                panic!("Error reading file: {:?}", err);
            }
        }
    }

    // Test to ensure all the values or non-empty.
    #[test]
    fn check_for_none() {
        let cells = Cell::read_csv("cells_test.csv").unwrap();

        for cell in cells.iter() {
            assert!(cell.oem.is_some() || cell.oem.is_none());
            assert!(cell.model.is_some() || cell.model.is_none());
            assert!(cell.launch_announced.is_some() || cell.launch_announced.is_none());
            assert!(cell.launch_status.is_some() || cell.launch_status.is_none());
            assert!(cell.body_dimensions.is_some() || cell.body_dimensions.is_none());
            assert!(cell.body_weight.is_some() || cell.body_weight.is_none());
            assert!(cell.body_sim.is_some() || cell.body_sim.is_none());
            assert!(cell.display_size.is_some() || cell.display_size.is_none());
            assert!(cell.display_resolution.is_some() || cell.display_resolution.is_none());
            assert!(cell.features_sensors.is_some() || cell.features_sensors.is_none());
            assert!(cell.platform_os.is_some() || cell.platform_os.is_none());

            println!("{:?}", cell);
        }
    }
}