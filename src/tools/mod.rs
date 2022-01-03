pub mod file_utils;
pub mod pretty_prints;

pub use self::file_utils::file_utils::get_file;
pub use self::file_utils::file_utils::read_file_as_numbers;
pub use self::file_utils::file_utils::read_file_as_string;
pub use self::pretty_prints::pretty_print_day;
pub use self::pretty_prints::pretty_print_result;
