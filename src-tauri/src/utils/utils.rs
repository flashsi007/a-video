use chrono::Local;

pub fn get_current_date() -> String {
    Local::now().format("%Y/%-m/%-d").to_string()
}