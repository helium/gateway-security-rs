pub mod add;
pub mod info;
pub mod verify;

pub fn print_json<T: ?Sized + serde::Serialize>(value: &T) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
