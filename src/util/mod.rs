pub mod edit_recompile_shared;
pub mod executable;
pub mod file_contents;
pub mod zip;

#[macro_export]
macro_rules! time {
    ($description:expr, $($tts:tt)*) => {
        {
            crate::print_task_start!($description);
            let start = std::time::Instant::now();
            let val = {$($tts)*};
            let time = start.elapsed();
            crate::println_task_duration!(time);
            val
        }
    };
}

#[macro_export]
macro_rules! time_no_scope {
    ($description:expr, $($tts:tt)*) => {
        crate::print_task_start!($description);
        let start = std::time::Instant::now();
        $($tts)*;
        let time = start.elapsed();
        crate::println_task_duration!(time)
    };
}

#[macro_export]
macro_rules! print_task_start {
    ($description:expr) => {
        print!("{}... ", $description);
    };
}

#[macro_export]
macro_rules! println_task_duration {
    ($end:expr) => {
        color_print::cprintln!("<cyan,bold>[{:?}]</>", $end)
    };
}
