use std::io::Error;

pub fn read_file_vec(file_path: &str) -> Result<Vec<String>, Error> {
    match std::fs::read_to_string(file_path) {
        Ok(result) => Ok(result
            .split("\r\n")
            .map(str::to_string)
            .collect::<Vec<String>>()),
        Err(error) => Err(error),
    }
}
