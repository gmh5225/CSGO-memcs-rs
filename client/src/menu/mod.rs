use std::{sync::{Arc, Mutex}, time::Duration};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, Event, self, KeyCode}};
use memcs_core::sdk::csgo::SignOnState;
use tui::{backend::{Backend, CrosstermBackend}, Frame, layout::{Layout, Direction, Constraint}, widgets::{List, ListItem, Block, Borders}, style::{Style, Modifier}, Terminal};

use crate::structs::{SharedData, PlayerInfo};

use self::stateful_list::StatefulList;

mod stateful_list;

const BORDER_STYLE: tui::style::Color = tui::style::Color::LightCyan;
const TEXT_STYLE: tui::style::Color = tui::style::Color::DarkGray;
const LOCAL_TEXT_STYLE: tui::style::Color = tui::style::Color::White;
const ENEMY_TEXT_STYLE: tui::style::Color = tui::style::Color::LightRed;
const FRIENDLY_TEXT_STYLE: tui::style::Color = tui::style::Color::LightBlue;

pub struct Menu {
    shared_data: Arc<Mutex<SharedData>>,
    settings_list: StatefulList<(String, bool)>
}

impl Menu {
    fn new(shared_data: Arc<Mutex<SharedData>>) -> Menu {
        Menu {
            shared_data,
            settings_list: StatefulList::with_items(vec![
                (String::from("glow"), false),
                (String::from("radar"), false),
                (String::from("chams"), false),
                (String::from("fakelag"), false)
            ])
        }
    }

    pub fn kill_cheat(&self) {
        let mut lock = self.shared_data.lock().unwrap();
        lock.should_exit = true;
    }

    pub fn run<B: Backend>(&mut self, f: &mut Frame<B>) {
        let rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(55),
                Constraint::Percentage(25),
                Constraint::Percentage(20),
            ])
            .margin(1)
            .split(f.size());
        
        let lock = self.shared_data.lock().unwrap();
        let settings = get_settings_block(&self.settings_list);
        let player_info = get_player_block(&lock.player_list);
        let statistics = get_statistics_block(lock.elapsed_time, lock.game_state);
        drop(lock);

        f.render_stateful_widget(settings, rects[0], &mut self.settings_list.state);
        f.render_widget(player_info, rects[1]);
        f.render_widget(statistics, rects[2]);
    }
}

pub fn run_menu(shared_data: Arc<Mutex<SharedData>>) -> std::io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut menu = Menu::new(shared_data.clone());
    terminal.clear().unwrap();
    
    loop {
        terminal.draw(|f| {
            menu.run(f);
        }).unwrap();

        if crossterm::event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        menu.kill_cheat();
                        break
                    },
                    KeyCode::Down => menu.settings_list.next(),
                    KeyCode::Up => menu.settings_list.previous(),
                    KeyCode::Char(' ') => {
                        if let Some(idx) = menu.settings_list.state.selected() {
                            let mut lock = shared_data.lock().unwrap();

                            fn switch(items: &mut [(String, bool)], setting: &mut bool, index: usize) {
                                *setting = !*setting;
                                items[index].1 = *setting;
                            }

                            match idx {
                                0 => {
                                    switch(&mut menu.settings_list.items, &mut lock.config.glow, idx)
                                },
                                1 => {
                                    switch(&mut menu.settings_list.items, &mut lock.config.radar, idx)
                                },
                                2 => {
                                    switch(&mut menu.settings_list.items, &mut lock.config.chams, idx)
                                },
                                3 => {
                                    switch(&mut menu.settings_list.items, &mut lock.config.fakelag, idx)
                                }
                                _ => {}
                            }
                        };
                    },
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}

fn get_settings_block(list_state: &StatefulList<(String, bool)>) -> List<'static> {
    let items: Vec<ListItem> = list_state
        .items
        .iter()
        .map(|i| {
            let enabled_text = match i.1 {
                false => "disabled",
                true => "enabled"
            };
            ListItem::new(format!("{} - {}", i.0, enabled_text))
                .style(
                    Style::default()
                        .fg(TEXT_STYLE)
                )
        })
        .collect();

    List::new(items)
        .block(
            Block::default()
                .title("settings")
                .borders(Borders::ALL)
                .border_style(
                    Style::default()
                        .fg(BORDER_STYLE)
                )
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
}

fn get_player_block(player_data: &[PlayerInfo]) -> List<'static> {
    let player_data = player_data
        .iter()
        .filter(|data| data.name().is_some());

    let items: Vec<ListItem> = player_data
        .map(|data| {
            let style = match (data.is_enemy(), data.is_local()) {
                (true, false) => Style::default().fg(ENEMY_TEXT_STYLE),
                (false, true) => Style::default().fg(LOCAL_TEXT_STYLE),
                (true, true) => Style::default().fg(ENEMY_TEXT_STYLE),
                (false, false) => Style::default().fg(FRIENDLY_TEXT_STYLE),
            };

            let name = match data.is_alive() {
                true => data.name().unwrap(),
                false => format!("[D] {}", data.name().unwrap()),
            };

            ListItem::new(name)
            .style(
                style
            )
    }).collect();

    List::new(items)
        .block(
            Block::default()
                .title("player list")
                .borders(Borders::ALL)
                .border_style(
                    Style::default()
                        .fg(BORDER_STYLE)
                )
        )
}

fn get_statistics_block(elapsed_time: Option<Duration>, game_state: Option<SignOnState>) -> List<'static> {
    let game_state_str = match game_state {
        Some(state) => state.to_string(),
        None => String::from("unknown"),
    };

    let items = vec![
        ListItem::new(format!("runtime: {} µs", elapsed_time.unwrap_or_default().as_micros()))
            .style(
                Style::default()
                .fg(TEXT_STYLE)
                .add_modifier(Modifier::ITALIC)
            ),

        ListItem::new(format!("state: {}",game_state_str))
            .style(
                Style::default()
                .fg(TEXT_STYLE)
                .add_modifier(Modifier::ITALIC)
            ),

        ListItem::new(" "),

        ListItem::new("Press q to quit.")
            .style(
                Style::default()
                .fg(TEXT_STYLE)
            )
    ];

    List::new(items)
        .block(
            Block::default()
                .title("info")
                .borders(Borders::ALL)
                .border_style(
                    Style::default()
                        .fg(BORDER_STYLE)
                )
        )
}