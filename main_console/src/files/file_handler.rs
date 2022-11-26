pub fn read_file_vec(file_path: &str) -> Vec<String> {
    match std::fs::read_to_string(file_path) {
        Ok(result) => result
            .split("\r\n")
            .map(str::to_string)
            .collect::<Vec<String>>(),
        Err(_) => panic!("Failed to open file."),
    }
}
