#[cfg(test)]
mod tests {
    use walkdir::WalkDir;

    #[test]
    fn test_pages_setup(){
        let pages_path = "./pages";
        let mut counter: usize = 0; 

        let walker = WalkDir::new(pages_path).into_iter();
        
        for entry in walker{
            match entry {
                Ok(entry) => {
                    let path = entry.file_name().to_string_lossy();
                    if path.ends_with(".html"){
                        counter += 1;
                    }
                },
                _ => (),
            }
        }

        assert_eq!(78, counter, "[ERROR]Test that there are 78 html files in the setup page, was {counter}");
    }
}