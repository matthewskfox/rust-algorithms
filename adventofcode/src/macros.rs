#[macro_export]
macro_rules! local_path {
    ($filename:expr) => {
        ::std::path::Path::new(file!())
            .parent()
            .expect("Parent directory not found")
            .join($filename)
            .to_string_lossy()
            .into_owned()
    };
    () => {
        local_path!("input.txt")
    };
}