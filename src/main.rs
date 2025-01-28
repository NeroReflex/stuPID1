extern crate libc;

use std::process::exit;
use std::ptr;

fn main() {
    unsafe {
        // Check if the current process is PID 1
        if libc::getpid() != 1 {
            exit(1);
        }

        // Create a signal set and fill it
        let mut set: libc::sigset_t = std::mem::zeroed();
        libc::sigfillset(&mut set);

        // Block all signals
        libc::sigprocmask(libc::SIG_BLOCK, &set, ptr::null_mut());

        // Fork the process
        if libc::fork() != 0 {
            // Parent process: wait indefinitely
            loop {
                let mut status: libc::c_int = 0;
                libc::wait(&mut status);
            }
        }

        // Unblock all signals
        libc::sigprocmask(libc::SIG_UNBLOCK, &set, ptr::null_mut());

        // Create a new session and set the process group ID
        // This drops kernel privileges
        libc::setsid();
        libc::setpgid(0, 0);

        // Execute a (supposedly init system) stored in /bin/init:
        // this replaces the current process with the specified one
        let args = [b"init\0".as_ptr() as *const i8, ptr::null()];
        let envs = [ptr::null()];
        libc::execve(
            b"/bin/init\0".as_ptr() as *const i8,
            args.as_ptr(),
            envs.as_ptr(),
        );

        // If execve fails, exit with an error
        exit(1);
    }
}
