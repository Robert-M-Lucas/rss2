pub mod edit_recompile_shared;
pub mod executable;
pub mod file_contents;
pub mod zip;

#[macro_export]
macro_rules! time {
    ($description:expr, $important:expr, $($tts:tt)*) => {
        {
            if ($important) || (*$crate::VERBOSE.get().unwrap()) {
                $crate::print_task_start!($description);
            }
            let start = std::time::Instant::now();
            let val = {$($tts)*};
            let time = start.elapsed();
            if (*$crate::VERBOSE.get().unwrap()) {
                $crate::println_task_duration!(time);
            }
            else if $important {
                println!();
            }
            val
        }
    };
}

#[macro_export]
macro_rules! time_leak_scope {
    ($description:expr, $important:expr, $($tts:tt)*) => {
        if ($important) || (*$crate::VERBOSE.get().unwrap()) {
            $crate::print_task_start!($description);
        }
        let start = std::time::Instant::now();
        $($tts)*;
        let time = start.elapsed();
        if (*$crate::VERBOSE.get().unwrap()) {
                $crate::println_task_duration!(time);
            }
            else if $important {
                println!();
            }
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
