use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn import_file(path: &str) -> std::io::Result<Vec<[char; 5]>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        let value: [char; 5] = line
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| std::io::Error::other("word is not 5 characters"))?;
        data.push(value);
    }

    Ok(data)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use tempdir::TempDir;

    use super::import_file;

    #[test]
    fn test_succes() {
        let tmp_dir = TempDir::new("/tmp/testing").unwrap();
        let file_path = tmp_dir.path().join("words.txt");
        let mut tmp_file = File::create(file_path.clone()).unwrap();
        writeln!(tmp_file, "abcde\nbcdea\ncdeab\n").unwrap();
        let data = import_file(&file_path.into_os_string().into_string().unwrap()).unwrap();
        assert_eq!(data.len(), 3);
    }
}
