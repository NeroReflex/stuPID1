#![no_main]

extern crate libc;

#[no_mangle]
fn main() {
    let program = b"/usr/bin/init\0".as_ptr() as *const libc::c_char;

    #[cfg(feature = "debug")]
    libc::printf(b"Started stuPID1...\n\0".as_ptr() as *const libc::c_char);

    unsafe {
        // Check if the current process is PID 1
        let pid = libc::getpid();
        if pid != 1 {
            #[cfg(feature = "debug")]
            libc::printf(b"Current process is not PID1: %d -- exiting.\n\0".as_ptr() as *const libc::c_char, pid);

            libc::exit(1);
        }

        // Create a signal set and fill it
        let mut set: libc::sigset_t = core::mem::zeroed();
        libc::sigfillset(&mut set);

        // Block all signals
        libc::sigprocmask(libc::SIG_BLOCK, &set, 0 as *mut libc::sigset_t);

        // Fork the process
        if libc::fork() != 0 {
            // Parent process: wait indefinitely
            loop {
                let mut status: libc::c_int = 0;
                libc::wait(&mut status);
            }
        }

        // Unblock all signals
        libc::sigprocmask(libc::SIG_UNBLOCK, &set, 0 as *mut libc::sigset_t);

        // Create a new session and set the process group ID
        // This drops kernel privileges
        libc::setsid();
        libc::setpgid(0, 0);

        // Execute a (supposedly init system) stored in /bin/init:
        // this replaces the current process with the specified one
        let args = [program, 0 as *mut libc::c_char];
        let envs = [0 as *mut libc::c_char];
        let execve_res = libc::execve(
            program,
            args.as_ptr(),
            envs.as_ptr() as *const *const libc::c_char,
        );

        if execve_res != 0 {
            #[cfg(feature = "debug")]
            libc::printf(b"execve failed with %d -- exiting.\n\0".as_ptr() as *const libc::c_char, execve_res);
        } else {
            unreachable!()
        }

        // If execve fails, exit with an error
        libc::exit(1);
    }
}
