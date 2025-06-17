use rasn_stack_overflow::*;

fn main() {
    unsafe { backtrace_on_stack_overflow::enable() };

    rasn::uper::encode(&Fizz {
        buzz: FizzBuzz::foo(()),
    })
    .unwrap();
}
