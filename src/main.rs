use nix::sys::signal;

pub extern "C" fn signal_handler( sig: i32) {
    println!("signal handle here, sig={}", sig);
    std::process::exit(1);
}


fn main() {
    
    let sig_action = signal::SigAction::new(
        signal::SigHandler::Handler(signal_handler),
        signal::SaFlags::empty(),
        signal::SigSet::empty());
    unsafe {
        //kill -15, catch sigterm ok
        let _= signal::sigaction(signal::SIGTERM, &sig_action);
        println!("SIGTERM handler installed");
        //kill -9, cannot catch sigkill yet
        //let _= signal::sigaction(signal::SIGKILL, &sig_action);
    }

    loop {};

}
