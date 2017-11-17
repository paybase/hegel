use std::io::{self, BufReader, Error};
use std::sync::mpsc;
use std::thread;
use std::io::prelude::*;
use std::process::{exit, Command, Child, Stdio};
use utils::{print};

pub enum Msg {
  StdErr(Option<String>),
  StdOut(Option<String>),
}

pub struct Process {
  pub command: String,
  pub arguments: Vec<String>,
  pub process: Child,
  tx: mpsc::Sender<Msg>,
  pub rx: mpsc::Receiver<Msg>,
}

impl Process {
  pub fn new(cmd: &str, args: &Vec<&str>) -> Result<Process, Error> {
    let command = cmd.to_owned();
    let arguments = args.iter()
      .map(|s|s.to_string())
      .collect();
    
    let (tx, rx) = mpsc::channel::<Msg>();
    let mut act = Command::new(cmd);
    
    match act.args(args)
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .spawn() {
      Ok(child) => Ok(Process {
        command,
        arguments,
        process: child,
        tx, rx,
      }),
      Err(e) => Err(e),
    }
  }

  pub fn run(&mut self) {
    let out_tx = self.tx.clone();
    let err_tx = self.tx.clone();
    
    let stdout = self.process.stdout.take().unwrap();
    let stderr = self.process.stderr.take().unwrap();

    thread::spawn(move || {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            out_tx.send(Msg::StdOut(Some(line.unwrap())))
              .expect("should be able to send to channel");
        }
    });

    thread::spawn(move || {
        let reader = BufReader::new(stderr);

        for line in reader.lines() {
            err_tx.send(Msg::StdErr(Some(line.unwrap())))
              .expect("should be able to send to channel");
        }
    });
  }
}

pub fn check_procs(pids: &mut Vec<Process>) -> i32 {
  loop {
    for ps in pids.iter_mut() {
      match ps.rx.try_recv() {
        Ok(Msg::StdOut(Some(string))) => {
          let out = format!("[ {}({}) ] {}\n", ps.command, ps.process.id(), string);
          io::stdout().write(out.as_bytes())
            .expect("should be able to send to stdout");;
        },
        Ok(Msg::StdErr(Some(string))) => {
          let out = format!("[ {}({}) ] {}\n", ps.command, ps.process.id(), string);
          io::stderr().write(out.as_bytes())
            .expect("should be able to send to stderr");
        },
        _ => ()
      }
      if let Ok(Some(status)) = ps.process.try_wait() {
        print(&format!("{}({}) exited with status {}", ps.command, ps.process.id(), status));
        return status.code().unwrap();
      }
    }
  }
}

pub fn kill_procs(mut pids: Vec<Process>, status: i32) {
  for ps in pids.iter_mut() {
    if let Ok(None) = ps.process.try_wait() {
      print(&format!("attempting to kill {}({})", ps.command, ps.process.id()));
        ps.process.kill().expect("should kill process")
      }
  }
  match status {
    0 => exit(1),
    s => exit(s),
  }
}