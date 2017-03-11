use std;
use std::sync::mpsc;

pub mod process;
pub use self::process::{Process,ProcessTask};
pub type ProcessSender=mpsc::Sender<ProcessTask>;

pub mod error;
pub use self::error::ProcessError;
