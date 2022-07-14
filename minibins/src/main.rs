use std::process::{ExitCode, Termination};

#[repr(u8)]
pub enum GitBisectResult {
    Good = 0,
    Bad = 1,
    Skip = 125,
    Abort = 255,
}

impl Termination for GitBisectResult {
    fn report(self) -> ExitCode {
        ExitCode::from(self as u8)
    }
}

fn main() -> GitBisectResult {
    std::panic::catch_unwind(|| {
        todo!("test the commit")
    }).unwrap_or(GitBisectResult::Abort)
}
