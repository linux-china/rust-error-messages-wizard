use thiserror::Error as ThisError;
use error_stack::{ResultExt};
use rust_error_messages_wizard::error_stack_anyhow::AnyhowIntoReport;

fn main() {
    let num_text = "11T11";
    match parse_num(num_text)
        .into_report()
        .attach_printable_lazy(|| format!("Failed to parser num {}", num_text))
        .change_context(AppError::Unknown) {
        Ok(num) => println!("num: {}", num),
        Err(e) => println!("{:?}", e),
    }
}

#[derive(ThisError, Debug)]
pub enum AppError {
    #[error("unknown App error")]
    Unknown,
}

fn parse_num(num_text: &str) -> anyhow::Result<i32> {
    let result = num_text.parse()?;
    Ok(result)
}
