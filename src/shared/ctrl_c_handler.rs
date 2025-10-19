use once_cell::sync::OnceCell;
use std::sync::Mutex;

// Global, not thread-local, with Send requirement on the callback
static CTRLC_CALLBACK: OnceCell<Mutex<Option<Box<dyn FnMut() + Send>>>> = OnceCell::new();

pub fn set_ctrl_c_handler(callback: Box<dyn FnMut() + Send>) {
    let cell = CTRLC_CALLBACK.get_or_init(|| Mutex::new(None));
    let mut cb = cell.lock().unwrap();
    *cb = Some(callback);
}

pub fn clear_ctrl_c_handler() {
    if let Some(cell) = CTRLC_CALLBACK.get() {
        let mut cb = cell.lock().unwrap();
        cb.take();
    }
}

pub fn init_ctrl_c_handler() {
    ctrlc::set_handler(|| {
        if let Some(cell) = CTRLC_CALLBACK.get() {
            let mut cb = cell.lock().unwrap();
            if let Some(mut callback) = cb.take() {
                callback();
            } else {
                unsafe {
                    // Reset handler to default
                    libc::signal(libc::SIGINT, libc::SIG_DFL);
                    libc::raise(libc::SIGINT);
                }
            }
        }
    })
    .unwrap();
}
