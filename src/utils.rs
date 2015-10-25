static mut verbose_flag: bool = false;

pub fn set_verbose(verbose: bool) {
    unsafe {
        verbose_flag = verbose;
    }
}

pub fn is_verbose() -> bool {
    unsafe {
       verbose_flag
    }
}

macro_rules! verboseln(
    ($($arg:tt)*) => ({
        if ::utils::is_verbose() {
            println!($($arg)*);
        }
    })
);
