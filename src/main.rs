extern crate libc;

fn main() {
    let program = b"/usr/bin/init\0".as_ptr() as *const libc::c_char;

    unsafe {
        // Check if the current process is PID 1
        if libc::getpid() != 1 {
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
        libc::execve(
            program,
            args.as_ptr(),
            envs.as_ptr() as *const *const libc::c_char,
        );

        // If execve fails, exit with an error
        libc::exit(1);
    }
}
