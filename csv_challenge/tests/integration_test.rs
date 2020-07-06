#[cfg(test)]
mod test {
    use csv_challenge::{
        replace_column, {load_csv, write_csv},
    };
    use std::path::PathBuf;
    #[test]
    fn test_csv_challenge() {
        let filename = PathBuf::from("./input/1.csv");
        let csv_data = load_csv(filename).unwrap();
        println!("{}", csv_data);
        let modified_data = replace_column(csv_data, "City", "Beijing").unwrap();
        println!("{}", modified_data);
        let output_file = write_csv(&modified_data, "output/test.csv");
        assert!(output_file.is_ok());
    }
}
