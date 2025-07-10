// ANCHOR: all
use std::{error::Error, io};

use std::path::Path;

use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen},
    ui::ui,
};

// ANCHOR: main_all
// ANCHOR: setup_boilerplate
fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    // ANCHOR_END: setup_boilerplate
    // ANCHOR: application_startup
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);
    // ANCHOR_END: application_startup

    // ANCHOR: ending_boilerplate
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    // ANCHOR_END: ending_boilerplate

    if let Ok(do_print) = res {
        if do_print {
            app.write_tasks_to_csv();
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
// ANCHOR_END: final_print
// ANCHOR_END: main_all

// ANCHOR: run_app_all
// ANCHOR: run_method_signature
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    // ANCHOR_END: run_method_signature
    // ANCHOR: ui_loop

    // CHECK FOR EXISTING CSV FILE
    if Path::new("tasks.csv").exists() {
        app.read_tasks_from_csv();
    }

    app.tasks_list_state.select(Some(0));
    app.todo_files_list_state.select(Some(0));

    loop {
        terminal.draw(|f| ui(f, app))?;
        // ANCHOR_END: ui_loop

        // ANCHOR: event_poll
        // ANCHOR: main_screen
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('b') => {
                        app.current_screen = CurrentScreen::Sidebar;
                    }

                    KeyCode::Char('a') => {
                        app.current_screen = CurrentScreen::Adding;
                        app.adding_task = true;
                    }

                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                        if let Some(selected_index) = app.list_item_available() {
                            app.editing_task_at = Some(selected_index);
                        }
                    }

                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }

                    KeyCode::Char('d') => {
                        if let Some(selected_index) = app.list_item_available() {
                            app.tasks.remove(selected_index);
                        }
                    }

                    KeyCode::Down => {
                        app.tasks_list_state.select_next();
                    }

                    KeyCode::Up => {
                        app.tasks_list_state.select_previous();
                    }

                    KeyCode::Left => {
                        app.tasks_list_state.select_first();
                    }

                    KeyCode::Right => {
                        app.tasks_list_state.select_last();
                    }

                    KeyCode::Enter => {
                        if let Some(selected_index) = app.list_item_available() {
                            app.tasks[selected_index].done = !app.tasks[selected_index].done;
                        }
                    }

                    _ => {}
                },

                CurrentScreen::Sidebar => match key.code {
                    KeyCode::Char('b') => {
                        app.current_screen = CurrentScreen::Main;
                    }

                    KeyCode::Down => {
                        app.todo_files_list_state.select_next();
                    }

                    KeyCode::Up => {
                        app.todo_files_list_state.select_previous();
                    }

                    KeyCode::Left => {
                        app.todo_files_list_state.select_first();
                    }

                    KeyCode::Right => {
                        app.todo_files_list_state.select_last();
                    }

                    _ => {}
                },

                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },

                CurrentScreen::Adding if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        app.save_task_value();
                    }
                    KeyCode::Backspace => {
                        app.task_input.pop();
                    }
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.adding_task = false;
                    }
                    KeyCode::Char(value) => {
                        app.task_input.push(value);
                    }

                    _ => {}
                },

                CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    }

                    KeyCode::Backspace => {
                        if let Some(selected_index) = app.list_item_available() {
                            app.tasks[selected_index].desc.pop();
                        }
                    }

                    KeyCode::Char(char) => {
                        if let Some(selected_index) = app.list_item_available() {
                            app.tasks[selected_index].desc.push(char);
                        }
                    }

                    _ => {}
                },

                _ => {}
            }
        }
        // ANCHOR_END: event_poll
    }
}
// ANCHOR: run_app_all

// ANCHOR_END: all
