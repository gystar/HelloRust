use super::{Error, File, PathBuf, Read, Write}; //引入父模块中引入的
pub fn load_csv(csv_file: PathBuf) -> Result<String, Error> {
    let file = read(csv_file)?;
    Ok(file)
}
/// # Usage:
/// ```ignore
/// use std::path::PathBuf;
/// let filename = PathBuf::from("./files/challenge.csv");
/// let csv_data = load_csv(filename);
/// assert!(csv_data.is_ok());
/// ```
pub fn write_csv(csv_data: &str, filename: &str) -> Result<(), Error> {
    write(csv_data, filename)?;
    Ok(())
}
fn read(path: PathBuf) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut file = open(path)?;
    file.read_to_string(&mut buffer)?;
    if buffer.is_empty() {
        return Err("input file missing")?;
    }
    Ok(buffer)
}
fn open(path: PathBuf) -> Result<File, Error> {
    let file = File::open(path)?;
    Ok(file)
}
fn write(data: &str, filename: &str) -> Result<(), Error> {
    let mut buffer = File::create(filename)?;
    buffer.write_all(data.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::load_csv;
    use std::path::PathBuf;
    #[test]
    fn test_valid_load_csv() {
        let filename = PathBuf::from("./input/1.csv");
        let csv_data = load_csv(filename);
        assert!(csv_data.is_ok());
    }
}
