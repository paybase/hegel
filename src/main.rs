extern crate clap;

mod utils;
mod process;

use utils::{print, get_arguments, parse_arguments};
use process::{Process, check_procs, kill_procs};

fn main() {
  let args = get_arguments();
  let vals = parse_arguments(&args);
  let mut pids: Vec<Process> = Vec::new();

  for (cmd, args) in vals {
    match Process::new(cmd, &args) {
      Ok(mut ps) => {
        print(&format!("successfully spawned {}({}) with arguments {:?}", ps.command, ps.process.id(), ps.arguments));
        ps.run();
        pids.push(ps)
      },
      Err(_) => {
        print(&format!("failed to spawn {} with arguments {:?}", cmd, args));
        return kill_procs(pids, 1);
      }
    }
  }

  let status = check_procs(&mut pids);
  kill_procs(pids, status)
}