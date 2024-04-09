
use crossterm::{
    event::{self},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use file_list::file_list;
use greeting::greeting;
use ratatui::{
    layout::Constraint, prelude::{Alignment, CrosstermBackend, Direction, Layout, Style, Terminal}, style::{Color, Styled, Stylize}, widgets::{block, Block, Borders, Paragraph, Wrap}
};
use std::{fs::OpenOptions, io::{stdout, Result}};
use tui_textarea::*;

use std::fs;
use std::io::prelude::*;
use std::io::LineWriter;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use chrono::prelude::*;
use directories::UserDirs;

mod file_list;
mod greeting;


//This function sets the active textarea configuration 
fn active(textarea: &mut TextArea<'_>){
    textarea.set_block(Block::default()
                       .set_style(Style::default())
                       .borders(Borders::ALL))
}

//This function sets the inactive textarea configuration 
fn inactive(textarea: &mut TextArea<'_>){
    textarea.set_block(Block::default().borders(Borders::ALL));
}

fn main() -> Result<()> {

    let message = greeting();
    let files = file_list();
    let list: String = files.join("\n");

    let path = UserDirs::new().unwrap();
    let home_path = format!("{}/Documents/logbook", path.home_dir().to_string_lossy());
    let settings_path = format!("{}/Documents/logbook/settings", path.home_dir().to_string_lossy());

    if !Path::new(&home_path).exists() {
        fs::create_dir(&home_path).expect("fuck");
        File::create(&settings_path)?;
    }

    //grabbing the date for the file name
    let date = Utc::now();
    let file_name = format!("{}-{}-{}.txt", date.month(), date.day(), date.year());
    let file_string = format!("{}/{}",&home_path, &file_name);

    //Create both TextArea's here
    //editors[0] is the main and editors[1] is the "search bar"
    let mut editors = [TextArea::default(), TextArea::default()];

    //This is all a big stinky hack. This feels wrong in so many ways 
    //Please find a way to write this better
    let read_settings = OpenOptions::new()
        .read(true)
        .open(&settings_path)
        .unwrap();
    let mut reader = BufReader::new(&read_settings);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let mut number: i32 = buf.trim().parse().unwrap();

    if !Path::new(&file_string).exists(){
        number += 1;
        let mut write_settings = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&settings_path)
            .unwrap();
        write!(write_settings, "{}", number)?;
    }

    if Path::new(&file_string).exists() {
        let read_file = OpenOptions::new()
            .read(true)
            .open(&file_string)
            .unwrap();
        let mut read_file = BufReader::new(&read_file);
        let mut file_buf = String::new();
        read_file.read_to_string(&mut file_buf)?;
        editors[0].insert_str(file_buf);
    }

    //Entering the alternate screen 
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    //set the active and inactive textareas and a counter to keep track which is active
    let mut which = 0;
    active(&mut editors[0]);
    inactive(&mut editors[1]);

    //some basic configuration
    editors[0].set_selection_style(Style::default().bg(Color::LightBlue));
    editors[0].set_placeholder_text("Talk to me skipps");
    editors[0].set_cursor_line_style(Style::default().not_underlined().not_hidden());
    editors[0].set_block(
        Block::default()
        .borders(Borders::ALL)
        .title(block::Title::from(format!("Logbook entry {}", &number)).alignment(Alignment::Center))
        );
    editors[1].set_block(Block::default().borders(Borders::ALL));

    //main loop that the program runs
    loop {
        terminal.draw(|frame| {

            //setting up things for the basic layout of it all
            let area = frame.size();

            let outer_border = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                             Constraint::Percentage(10),
                             Constraint::Percentage(90),
                ])
                .split(area);

            let inner_border = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                             Constraint::Percentage(20),
                             Constraint::Percentage(80),
                ]).split(outer_border[1]);

            let weird_border = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                             Constraint::Percentage(90),
                             Constraint::Percentage(10),
                ])
                .split(inner_border[0]);

            //Rendering the frames of the program
            frame.render_widget(editors[1].widget(), weird_border[1]);
            frame.render_widget(editors[0].widget(), inner_border[1]);
            frame.render_widget(Paragraph::new(format!("{}", message))
                                .wrap(Wrap { trim: (true) })
                                .alignment(Alignment::Center)
                                .block(Block::default()
                                       .title("Captain's Log")
                                       .title_alignment(Alignment::Center)
                                       .borders(Borders::ALL)), outer_border[0]);

            frame.render_widget(Paragraph::new(format!("{}", &list))
                                .wrap(Wrap { trim: (true) })
                                .alignment(Alignment::Center)
                                .block(Block::default()
                                       .title("Previous files")
                                       .title_alignment(Alignment::Center)
                                       .borders(Borders::ALL)), inner_border[0]);
        })?;

        //Apon pressing escape, close the program and write to the file
        if event::poll(std::time::Duration::from_millis(16))? {
            match crossterm::event::read()?.into() {
                Input {
                    key: Key::Char('s'),
                    ctrl: true,
                    ..
                } => {
                    {
                        if Path::new(&file_string).exists(){
                            let mut file_writer= OpenOptions::new()
                                .write(true)
                                .truncate(true)
                                .open(&file_string)
                                .unwrap();
                            for line in editors[0].lines() {
                                write!(file_writer, "{}", line)?;
                                writeln!(file_writer, "")?;
                            }
                        }
                        else {
                            let f = File::create(&file_string)?;
                            let mut writer = LineWriter::new(f);
                            for line in editors[0].lines(){
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
                } => {editors[0].move_cursor(CursorMove::Up)},
                Input{
                    key: Key::Down,
                    ..
                } => {editors[0].move_cursor(CursorMove::Down)},
                Input{
                    key: Key::Right,
                    ..
                } => {editors[0].move_cursor(CursorMove::Forward)},
                Input{
                    key: Key::Left,
                    ..
                } => {editors[0].move_cursor(CursorMove::Back)},
                Input{
                    key: Key::Enter,
                    ..
                } => {
                    if (which+1)%2 == 1 {
                        editors[0].insert_newline();
                    }
                    else {
                        editors[1].delete_line_by_head();
                        which = (which+1)%2;
                        active(&mut editors[which]);
                        editors[0].set_block(
                            Block::default()
                            .borders(Borders::ALL)
                            .title(block::Title::from(format!("Logbook entry {}", &number)).alignment(Alignment::Center))
                            );
                    }
                },
                Input{
                    key: Key::Char('u'),
                    ctrl: true,
                    ..
                } => {
                    editors[0].move_cursor(CursorMove::Up);
                    editors[0].move_cursor(CursorMove::End)
                },
                Input{
                    key: Key::Char('e'),
                    ctrl: true,
                    ..
                } => {editors[0].move_cursor(CursorMove::End)},
                Input{
                    key: Key::Char('d'),
                    ctrl: true,
                    ..
                } => {
                    editors[0].move_cursor(CursorMove::Down);
                    editors[0].move_cursor(CursorMove::End)
                },
                Input{
                    key: Key::Char('V'),
                    alt: true,
                    ..
                } => {editors[0].start_selection();},
                Input{
                    key: Key::Char('t'),
                    ctrl: true,
                    ..
                } => {
                    inactive(&mut editors[which]);
                    which = (which+1)%2;
                    active(&mut editors[which]);
                    editors[0].set_block(
                        Block::default()
                        .borders(Borders::ALL)
                        .title(block::Title::from(format!("Logbook entry {}", &number)).alignment(Alignment::Center))
                        );
                },

                input => {
                    if editors[which].input(input) {
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
