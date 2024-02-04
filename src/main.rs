use anyhow::Result;

mod tui;

pub mod converter;

fn main() -> Result<()> {
    startup()?;

    let status = tui::run();
    shutdown()?;
    status?;

    Ok(())
}

fn startup() -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
