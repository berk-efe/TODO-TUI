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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
            // app.write_tasks_to_csv();
            println!("write changes")
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

    app.tasks_list_state.select(Some(0));
    app.sidebar_state.select(Some(0));

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
                        if let Some((index, todo)) =
                            app.list_item_available().zip(app.current_todo.as_mut())
                        {
                            todo.tasks.remove(index);
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
                        if let Some((index, todo)) =
                            app.list_item_available().zip(app.current_todo.as_mut())
                        {
                            todo.tasks[index].done = !todo.tasks[index].done
                        }
                    }

                    _ => {}
                },

                CurrentScreen::Sidebar => match key.code {
                    KeyCode::Char('a') => {
                        app.current_screen = CurrentScreen::AddingProj;
                    }
                    KeyCode::Char('b') => {
                        app.current_screen = CurrentScreen::Main;
                    }

                    KeyCode::Down => {
                        app.sidebar_state.select_next();
                    }

                    KeyCode::Up => {
                        app.sidebar_state.select_previous();
                    }

                    KeyCode::Left => {
                        app.sidebar_state.select_first();
                    }

                    KeyCode::Right => {
                        app.sidebar_state.select_last();
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

                CurrentScreen::AddingProj if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Backspace => {
                        app.input_buffer.pop();
                    }
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Sidebar;
                    }
                    KeyCode::Char(value) => {
                        app.input_buffer.push(value);
                    }
                    _ => {}
                },

                CurrentScreen::Adding if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        app.save_task_value();
                    }
                    KeyCode::Backspace => {
                        app.input_buffer.pop();
                    }
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Char(value) => {
                        app.input_buffer.push(value);
                    }

                    _ => {}
                },

                CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    }

                    KeyCode::Backspace => {
                        if let Some((index, todo)) =
                            app.list_item_available().zip(app.current_todo.as_mut())
                        {
                            todo.tasks[index].desc.pop();
                        }
                    }

                    KeyCode::Char(char) => {
                        if let Some((index, todo)) =
                            app.list_item_available().zip(app.current_todo.as_mut())
                        {
                            todo.tasks[index].desc.push(char);
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
