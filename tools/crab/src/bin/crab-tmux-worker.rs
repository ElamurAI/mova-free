//! `crab-tmux-worker SESSION CWD -- ABS_COMMAND [ARGS...]`: hold a program in a detached tmux
//! session as an ordinary Crab worker. See `mova_crab::tmux_worker`.
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(mova_crab::tmux_worker::main_with_args(&args));
}
