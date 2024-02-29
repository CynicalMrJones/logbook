
use crossterm::{
    event::{self},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    style::Color,
    layout::Constraint, prelude::{Alignment, CrosstermBackend, Direction, Layout, Style, Terminal},
    widgets::{block::{self}, Block, Borders, Paragraph, Wrap}
};
use std::{fs::OpenOptions, io::{stdout, Result}};
use tui_textarea::*;

use std::io::prelude::*;
use std::io::LineWriter;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use chrono::prelude::*;

fn main() -> Result<()> {

    //grabbing the date for the file name
    let date = Utc::now();
    let file_name = format!("{}-{}-{}.txt", date.month(), date.day(), date.year());
    let true_date = format!("{}-{}-{}", date.month(), date.day(), date.year());

    //This is all a big stinky hack. This feels wrong in so many ways 
    //Please find a way to write this better
    let read_settings = OpenOptions::new()
        .read(true)
        .open("settings")
        .unwrap();
    let mut reader = BufReader::new(&read_settings);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let mut number: i32 = buf.trim().parse().unwrap();

    if !Path::new(&file_name).exists(){
        number += 1;
        let mut write_settings = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("settings")
            .unwrap();
        write!(write_settings, "{}", number)?;
    }

    //Entering the alternate screen 
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    //creating the Text box for writing in
    let mut text = TextArea::default();

    //Modifying the text area with certain qualities
    text.set_selection_style(Style::default().bg(Color::LightBlue));
    text.set_placeholder_text("Please enter what you want");
    text.set_style(Style::default());
    text.set_block(
        Block::default()
        .borders(Borders::ALL)
        .title(block::Title::from(format!("Logbook entry {}", &number)).alignment(Alignment::Center))
        );

    //main loop that the program runs
    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let textwidget = text.widget();

            let outer_border = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                             Constraint::Percentage(10),
                             Constraint::Percentage(90),
                ])
                .split(area);

            //Rendering the frames of the program
            frame.render_widget(textwidget, outer_border[1]);
            frame.render_widget(Paragraph::new(format!("Good afternoon Captain, todays date is {}", true_date))
                                .wrap(Wrap { trim: (true) })
                                .alignment(Alignment::Center)
                                .block(Block::default()
                                       .title("Captain's Log")
                                       .title_alignment(Alignment::Center)
                                       .borders(Borders::ALL)), outer_border[0]);
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
                } => {text.move_cursor(CursorMove::Up)},
                Input{
                    key: Key::Down,
                    ..
                } => {text.move_cursor(CursorMove::Down)},
                Input{
                    key: Key::Right,
                    ..
                } => {text.move_cursor(CursorMove::Forward)},
                Input{
                    key: Key::Left,
                    ..
                } => {text.move_cursor(CursorMove::Back)},
                Input{
                    key: Key::Enter,
                    ..
                } => {text.insert_newline();},
                Input{
                    key: Key::Char('c'),
                    ctrl: true,
                    ..
                } => {text.copy();},
                Input{
                    key: Key::Char('v'),
                    ctrl: true,
                    ..
                } => {text.paste();},
                Input{
                    key: Key::Char('V'),
                    shift: true,
                    ..
                } => {text.start_selection();},

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
    Ok(())
}
