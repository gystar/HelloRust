use super::*;
pub fn replace_column(
    data: String, /*读取到的csv中的全文*/
    column: &str,
    replacement: &str,
) -> Result<String, Error> {
    let mut lines = data.lines();
    let headers = lines.next().unwrap();
    let columns: Vec<&str> = headers.split(',').collect();
    let column_nunber = columns.iter().position(|&e| e == column);
    //注意：匹配关键字的时候，关键字不能有多余的字符，包括空格
    let column_nunber = match column_nunber {
        Some(column) => column,
        _node => {
            println!("headers:{}", headers);
            Err("column name doesn't exist in the input file")?
        }
    };
    let mut result = String::with_capacity(data.capacity());
    result.push_str(&columns.join(","));
    result.push('\n');
    for line in lines {
        let mut records: Vec<&str> = line.split(',').collect();
        records[column_nunber] = replacement;
        result.push_str(&records.join(","));
        result.push('\n');
    }
    Ok(result)
}
