/// Appends " [y/N]" to prompt, prints it to stdout and returns the answer as a
/// boolean
pub fn confirm_prompt(prompt: &str) -> Result<bool, String> {
    let full_prompt = format!("{} [y/N]", prompt);
    println!("{}", full_prompt);

    let answer = console::Term::stdout().read_char().map_err(|err| err.to_string())?;
    Ok(answer == 'y' || answer == 'Y')
}
