use kurisu::*;
use std::path::PathBuf;

#[derive(Debug, Kurisu)]
#[kurisu(
    name = "grep",
    version = "3.0",
    desc = "Search for PATTERN in each FILE or standard input. PATTERN is, by default, a basic regular expression (BRE)."
)]
/// 'egrep' means 'grep -E'.  'fgrep' means 'grep -F'.
/// Direct invocation as either 'egrep' or 'fgrep' is deprecated.
/// When FILE is -, read standard input.  With no FILE, read . if a command-line
/// -r is given, - otherwise.  If fewer than two FILEs are given, assume -h.
/// Exit status is 0 if any line is selected, 1 otherwise;
/// if any error occurs and -q is not given, the exit status is 2.
///
/// Report bugs to: bug-grep@gnu.org
/// GNU grep home page: <http://www.gnu.org/software/grep/>
/// General help using GNU software: <http://www.gnu.org/gethelp/>
// TODO: Add feature to change --test <VALUE> to --test=VALUE ?
struct Grep {
    #[kurisu(pos = 1, doc = "PATTERN is, by default, a basic regular expression (BRE).")]
    pattern: String,
    #[kurisu(pos, doc = "File paths")]
    files: Vec<PathBuf>,
    // Regexp selection and interpretation:
    #[kurisu(short = "E", doc = "PATTERN is an extended regular expression (ERE)")]
    extended_regexp: bool,
    #[kurisu(short = "F", doc = "PATTERN is a set of newline-separated strings")]
    fixed_strings: bool,
    #[kurisu(short = "G", doc = "PATTERN is a basic regular expression (BRE)")]
    basic_regexp: bool,
    #[kurisu(short = "P", doc = "PATTERN is a Perl regular expression")]
    perl_regexp: bool,
    #[kurisu(short = "e", vname = "PATTERN", doc = "use PATTERN for matching")]
    regexp: String,
    #[kurisu(short, doc = "obtain PATTERN from FILE")]
    file: PathBuf,
    #[kurisu(short, doc = "ignore case distinctions")]
    ignore_case: bool,
    #[kurisu(short, doc = "force PATTERN to match only whole words")]
    word_regexp: bool,
    #[kurisu(short = "x", doc = "force PATTERN to match only whole lines")]
    line_regexp: bool,
    #[kurisu(short = "z", doc = "a data line ends in 0 byte, not newline")]
    null_data: bool,
    // Miscellaneous:
    #[kurisu(short = "s", doc = "suppress error messages")]
    no_message: bool,
    #[kurisu(short = "v", doc = "select non-matching lines")]
    invert_match: bool,
    // Output control:
    #[kurisu(short = "m", vname = "NUM", doc = "stop after NUM matches")]
    max_count: usize,
    #[kurisu(short = "b", doc = "print the byte offset with output lines")]
    byte_offset: bool,
    #[kurisu(short = "n", doc = "print line number with output lines")]
    line_number: bool,
    #[kurisu(doc = "flush output on every line")]
    line_buffered: bool,
    #[kurisu(short = "H", doc = "print the file name for each match")]
    with_filename: bool,
    #[kurisu(doc = "suppress the file name prefix on output")]
    no_filename: bool,
    #[kurisu(vname = "LABEL", doc = "use LABEL as the standard input file name prefix")]
    label: String,
    #[kurisu(short, doc = "use LABEL as the standard input file name prefix")]
    only_matching: bool,
    #[kurisu(short, doc = "suppress all normal output", aliases = "silent")]
    quiet: bool,
    #[kurisu(vname = "TYPE", doc = "assume that binary files are TYPE; TYPE is 'binary', 'text', or 'without-match'")]
    binary_files: String,
    #[kurisu(short = "a", doc = "equivalent to --binary-files=text")]
    text: bool,
    #[kurisu(short = "I", nolong, doc = "equivalent to --binary-files=without-match")]
    bin_without_match: bool,
    #[kurisu(short = "d", vname = "ACTION", doc = "how to handle directories; ACTION is 'read', 'recurse', or 'skip'")]
    directories: String,
    #[kurisu(
        short = "D",
        vname = "ACTION",
        doc = "how to handle devices, FIFOs and sockets; ACTION is 'read', 'recurse', or 'skip'"
    )]
    devices: String,
    #[kurisu(short = "r", nolong, doc = "like --directories=recurse")]
    recursive: bool,
    #[kurisu(short = "R", nolong, doc = "likewise, but follow all symlinks")]
    dereference_recursive: bool,
    #[kurisu(vname = "FILE_PATTERN", doc = "search only files that match FILE_PATTERN")]
    include: String,
    #[kurisu(vname = "FILE_PATTERN", doc = "skip files and directories matching FILE_PATTERN")]
    exclude: String,
    #[kurisu(vname = "FILE", doc = "skip files matching any file pattern from FILE")]
    exclude_from: String,
    #[kurisu(vname = "FILE", doc = "directories that match PATTERN will be skipped.")]
    exclude_dir: String,
    #[kurisu(short = "L", doc = "print only names of FILEs containing no match")]
    files_without_match: bool,
    #[kurisu(short = "l", doc = "print only names of FILEs containing matches")]
    files_with_match: bool,
    #[kurisu(short = "c", doc = "print only a count of matching lines per FILE")]
    count: bool,
    #[kurisu(short = "T", doc = "make tabs line up (if needed)")]
    initial_tab: bool,
    #[kurisu(short = "Z", doc = "print 0 byte after FILE name")]
    null: bool,
    // Context control:
    #[kurisu(short = "B", vname = "NUM", doc = "print NUM lines of leading context")]
    before_context: usize,
    #[kurisu(short = "A", vname = "NUM", doc = "print NUM lines of trailing context")]
    after_context: usize,
    #[kurisu(short = "C", vname = "NUM", doc = "print NUM lines of output context")]
    context: usize,
    // #[kurisu(doc = "same as --context=NUM")] // Cant do -NUM  aka: -12
    // num: usize,
    #[kurisu(
        vname = "WHEN",
        aliases = "colour",
        doc = "use markers to highlight the matching strings; WHEN is 'always', 'never', or 'auto'"
    )]
    color: String,
    #[kurisu(short = "U", doc = "do not strip CR characters at EOL (MSDOS/Windows)")]
    binary: bool,
    #[kurisu(short = "u", doc = "report offsets as if CRs were not there (MSDOS/Windows)")]
    unix_byte_offsets: bool,
}

fn main() {
    let env_args = std::env::args().skip(1).collect();
    let args = Grep { ..Grep::from_args(env_args) };
    kurisu::valid_exit(&args);

    // Logic
}
