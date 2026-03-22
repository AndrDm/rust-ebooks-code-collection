pub mod log_api {
    pub fn info(str: String) {
        crate::log::write_log(str);
    }

    // ...
}