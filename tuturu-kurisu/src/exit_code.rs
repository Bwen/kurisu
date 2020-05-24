// TODO: Implement Termination trait once stable
pub enum ExitCode {
    OK,
    /// General Failure
    FAILURE,
    /// The command was used incorrectly, e.g., with the wrong number of arguments, a bad flag, a bad syntax in a parameter, or whatever.
    USAGE,
    /// The input data was incorrect in some way. This should only be used for user's data & not system files.
    DATAERR,
    /// An input file (not a system file) did not exist or was not readable.
    /// This could also include errors like "No message" to a mailer (if it cared to catch it).
    NOINPUT,
    /// The user specified did not exist.  This might be used for mail addresses or remote logins.
    NOUSER,
    /// The host specified did not exist.  This is used in mail addresses or network requests.
    NOHOST,
    /// A service is unavailable.  This can occur if a support program or file does not exist.
    /// This can also be used as a catchall message when something you wanted to do doesn't work,
    /// but you don't know why.
    UNAVAILABLE,
    /// An internal software error has been detected. This should be limited to non-operating system related errors as possible.
    SOFTWARE,
    /// An operating system error has been detected.
    /// This is intended to be used for such things as "cannot fork", "cannot create pipe", or the like.
    /// It includes things like getuid returning a user that does not exist in the passwd file.
    OSERR,
    /// Some system file (e.g., /etc/passwd, /etc/utmp, etc.) does not exist, cannot be opened, or has some sort of error (e.g., syntax error).
    OSFILE,
    /// A (user specified) output file cannot be created.
    CANTCREAT,
    /// An error occurred while doing I/O on some file.
    IOERR,
    /// temporary failure, indicating something that is not really an error.
    /// In sendmail, this means that a mailer (e.g.) could not create a connection,
    /// and the request should be reattempted later.
    TEMPFAIL,
    /// the remote system returned something that was "not possible" during a protocol exchange.
    PROTOCOL,
    /// You did not have sufficient permission to perform the operation.
    /// This is not intended for file system problems, which should use NOINPUT or
    /// CANTCREAT, but rather for higher level permissions.
    NOPERM,
    /// Configuration error
    CONFIG,
}

impl Into<i32> for ExitCode {
    fn into(self) -> i32 {
        match self {
            ExitCode::OK => 0,
            ExitCode::FAILURE => 1,
            ExitCode::USAGE => 64,
            ExitCode::DATAERR => 65,
            ExitCode::NOINPUT => 66,
            ExitCode::NOUSER => 67,
            ExitCode::NOHOST => 68,
            ExitCode::UNAVAILABLE => 69,
            ExitCode::SOFTWARE => 70,
            ExitCode::OSERR => 71,
            ExitCode::OSFILE => 72,
            ExitCode::CANTCREAT => 73,
            ExitCode::IOERR => 74,
            ExitCode::TEMPFAIL => 75,
            ExitCode::PROTOCOL => 76,
            ExitCode::NOPERM => 77,
            ExitCode::CONFIG => 78,
        }
    }
}
