
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand, style::Stylize,
};
use ratatui::{
    prelude::{CrosstermBackend,Style, Terminal, Alignment, Layout, Direction},
    widgets::{Block, Borders, block, Paragraph, Wrap}, layout::Constraint,
};
use std::io::{stdout, Result};
use tui_textarea::*;

use std::io::prelude::*;
use std::io::BufWriter;
use std::io::LineWriter;
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
    let mut text2 = TextArea::default();

    //Modifying the text area with certain qualities
    text.set_placeholder_text("Please enter what you want");
    text.set_style(Style::default());
    text.set_block(
        Block::default()
        .borders(Borders::ALL)
        .title(block::Title::from("Captains Logbook").alignment(Alignment::Center))
        );
    text2.set_placeholder_text("Please enter what you want");
    text2.set_style(Style::default());
    text2.set_block(
        Block::default()
        .borders(Borders::ALL)
        .title(block::Title::from("Captains Logbook").alignment(Alignment::Center))
        );

    //main loop that the program runs
    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let textwidget = text.widget();
            let textwidget2 = text2.widget();

            let outer_border = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                             Constraint::Percentage(80),
                             Constraint::Percentage(20),
                ])
                .split(area);

            //Rendering the frames of the program
            frame.render_widget(textwidget, outer_border[0]);
            frame.render_widget(textwidget2, outer_border[1]);
        })?;

        //Apon pressing escape, close the program and write to the file
        if event::poll(std::time::Duration::from_millis(16))? {
            match crossterm::event::read()?.into() {
                Input {
                    key: Key::Esc,
                    ..
                } => {
                    {
                        if Path::new(&file_name).exists(){
                            let f = File::options().append(true).open(file_name)?;
                            let mut writer = LineWriter::new(f);
                            for line in text.lines(){
                                if line == "" {
                                    break;
                                }
                                    writer.write(line.as_bytes())?;
                                    writeln!(writer, "").unwrap();
                            }
                        }
                        else {
                            let f = File::create(file_name)?;
                            let mut writer = LineWriter::new(f);
                            for line in text.lines(){
                                if line == "" {
                                    break; 
                                }
                                    writer.write(line.as_bytes())?;
                                    writeln!(writer, "").unwrap();
                            }
                        }
                    }
                    break;
                },
                Input{
                    key: Key::Up,
                    ..
                } => {text.set_placeholder_text("Fuck you")},
                Input{
                    key: Key::Down,
                    ..
                } => {text.set_placeholder_text("Stop Fucking around")},
                Input{
                    key: Key::Enter,
                    ..
                } => {text2.set_placeholder_text(format!("{:?}", text.lines()));
                    text.insert_newline();
                },
                input => {
                    if text.input(input) {
                    }
                }
            }
        }
    }

    //exiting the program
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    println!("{:?}",text.lines());
    Ok(())
}
