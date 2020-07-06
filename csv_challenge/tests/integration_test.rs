#[cfg(test)]
mod test {
    use csv_challenge::{
        replace_column, Opt, {load_csv, write_csv},
    };
    use std::path::PathBuf;
    #[test]
    fn test_csv_challenge() {
        let filename = PathBuf::from("./input/1.csv");
        let csv_data = load_csv(filename).unwrap();
        let modified_data = replace_column(csv_data, "City", "Beijing").unwrap();
        let output_file = write_csv(&modified_data, "ouput/test.csv");
        assert!(output_file.is_ok());
    }
}
