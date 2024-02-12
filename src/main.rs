
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand, style::Stylize,
};
use ratatui::{
    prelude::{CrosstermBackend,Style, Terminal, Alignment},
    widgets::{Block, Borders, block},
};
use std::io::{stdout, Result};
use tui_textarea::TextArea;

use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;
use chrono::prelude::*;

fn main() -> Result<()> {

    //grabbing the date for the file name
    let date = Utc::now();
    let file_name = format!("{}-{}-{}.txt", date.month(), date.day(), date.year());

    //Entering the alternate screen 
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    //creating the Text box for writing in
    let mut text = TextArea::default();

    //Modifying the text area with certain qualities
    text.set_placeholder_style(Style::default());
    text.set_placeholder_text("Please enter what you want");
    text.set_alignment(Alignment::Center);
    text.set_block(
        Block::default()
        .borders(Borders::ALL)
        .title(block::Title::from("Captains Logbook").alignment(Alignment::Center))
        );

    //main loop that the program runs
    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let textwidget = text.widget();
            frame.render_widget(textwidget, area)
        })?;

        //Apon pressing escape, close the program and write to the file
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && key.code == KeyCode::Esc
                    {
                        {
                            if Path::new(&file_name).exists(){
                                let f = File::options().append(true).open(file_name)?;
                                let mut writer = BufWriter::new(f);
                                for line in text.lines(){
                                    writer.write(line.as_bytes())?;
                                }
                            }
                            else {
                                let f = File::create(file_name)?;
                                let mut writer = BufWriter::new(f);
                                for line in text.lines(){
                                    writer.write(line.as_bytes())?;
                                }
                            }
                        }
                        break;
                    }
                text.input(key);
            }
        }
    }

    //exiting the program
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
