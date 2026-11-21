use std::io::{self, IsTerminal, Stdout};

use clap::{Parser, Subcommand};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use clw_core::Blockchain;
use clw_security::PlatformTarget;

#[derive(Parser)]
#[command(name = "clw", version, about = "CommandLineWallet — Phase 1 preview")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Show current security capabilities and planned platform integration.
    Security,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().command {
        Some(Command::Security) => {
            println!("{}", security_report());
            Ok(())
        }
        None => run_home().map_err(Into::into),
    }
}

fn security_report() -> String {
    format!(
        "CLW security status (Phase 1)\nHardware target: {}\nHardware detection: not implemented\nHardware-assisted vault protection: not implemented\nEncrypted vault and recovery: not implemented\nTransaction parsing, approval, and signing: not implemented\nNo wallet functionality is available.",
        PlatformTarget::current().planned_hardware()
    )
}

fn home_text() -> String {
    let targets = Blockchain::ALL.map(Blockchain::name).join(" · ");
    format!(
        "CommandLineWallet (CLW)\n\nTargets: {targets}\n\nWallet functionality is not implemented yet.\n\nPress q or Esc to exit."
    )
}

fn run_home() -> io::Result<()> {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        println!("{}", home_text());
        return Ok(());
    }

    let mut terminal = TerminalSession::start()?;
    loop {
        terminal.terminal.draw(|frame| {
            let area = frame.area();
            let [_, center, _] = Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(10),
                Constraint::Fill(1),
            ])
            .areas(area);
            let card = Paragraph::new(home_text())
                .alignment(Alignment::Center)
                .style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .block(Block::default().title(" CLW ").borders(Borders::ALL));
            frame.render_widget(card, center);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press
                && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
            {
                break;
            }
        }
    }
    Ok(())
}

struct TerminalSession {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalSession {
    fn start() -> io::Result<Self> {
        enable_raw_mode()?;
        if let Err(error) = execute!(io::stdout(), EnterAlternateScreen) {
            let _ = disable_raw_mode();
            return Err(error);
        }
        match Terminal::new(CrosstermBackend::new(io::stdout())) {
            Ok(terminal) => Ok(Self { terminal }),
            Err(error) => {
                let _ = disable_raw_mode();
                let _ = execute!(io::stdout(), LeaveAlternateScreen);
                Err(error)
            }
        }
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn security_report_does_not_claim_operational_protection() {
        let report = security_report();
        assert!(report.contains("Hardware detection: not implemented"));
        assert!(report.contains("Transaction parsing, approval, and signing: not implemented"));
    }

    #[test]
    fn home_mentions_every_target_and_unavailable_wallet() {
        let home = home_text();
        for chain in Blockchain::ALL {
            assert!(home.contains(chain.name()));
        }
        assert!(home.contains("Wallet functionality is not implemented yet."));
    }
}
