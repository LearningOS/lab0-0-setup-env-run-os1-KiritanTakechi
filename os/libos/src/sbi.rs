use sbi_rt::{legacy::console_putchar, system_reset, NoReason, Shutdown, SystemFailure};

pub fn putchar(c:usize ){
    #[allow(deprecated)]
    console_putchar(c);
}

pub fn shutdown(failure: bool) -> ! {
    match failure {
        true => system_reset(Shutdown, SystemFailure),
        false => system_reset(Shutdown, NoReason),
    };
    unreachable!()
}