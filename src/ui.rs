// ANCHOR: all
use ratatui::{
    Frame,
    crossterm::style,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, StatefulWidget, Wrap},
};

use crate::app::{App, CurrentScreen, Task};

// ANCHOR: method_sig
pub fn ui(frame: &mut Frame, app: &mut App) {
    // ANCHOR_END: method_sig
    // Create the layout sections.
    // ANCHOR: ui_layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());
    // ANCHOR_END: ui_layout

    // ANCHOR: title_paragraph
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title =
        Paragraph::new(Text::styled("Todo", Style::default().fg(Color::Green))).block(title_block);

    frame.render_widget(title, chunks[0]);
    // ANCHOR_END: title_paragraph

    let mut todo_files_list_items: Vec<ListItem> = Vec::new();

    // ADD DUMMY ELEMENTS FOR NOW
    todo_files_list_items.push(ListItem::new(Line::from("Todo App 1")));

    let todo_files_list = List::new(todo_files_list_items)
        .highlight_symbol("> ")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default()),
        );

    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Fill(1)])
        .split(chunks[1]);

    if app.current_screen == CurrentScreen::Sidebar {
        frame.render_stateful_widget(todo_files_list, main[0], &mut app.todo_files_list_state);
    }
    let mut tasks_list_items: Vec<ListItem> = Vec::new();

    // ADD DATA MANUALLY FOR TESTING
    // tasks_list_items.push(ListItem::new(Line::from("Task 1")));
    // tasks_list_items.push(ListItem::new(Line::from("Task 2")));

    for (index, Task { done, desc }) in app.tasks.iter().enumerate() {
        let mut done_char = ' ';
        if *done == true {
            done_char = '✓'
        }

        let is_editing =
            app.current_screen == CurrentScreen::Editing && app.editing_task_at == Some(index);

        let cur_style = if is_editing {
            Style::default().fg(Color::Blue)
        } else if *done {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        };

        tasks_list_items.push(ListItem::new(Line::from(Span::styled(
            format!("[{}] - {: <32}", done_char, desc),
            cur_style,
        ))));
    }

    let tasks_list = List::new(tasks_list_items).highlight_symbol("> ").block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default()),
    );

    if app.current_screen == CurrentScreen::Sidebar {
        frame.render_stateful_widget(tasks_list, main[1], &mut app.tasks_list_state);
    } else {
        frame.render_stateful_widget(tasks_list, chunks[1], &mut app.tasks_list_state);
    }
    // EDITING MODE

    if app.adding_task == true {
        let popup_block = Block::default()
            .title("Enter task.")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(popup_block, area);
        // ANCHOR_END: editing_popup

        // ANCHOR: key_value_blocks
        let task_block = Block::default()
            .title("Task description:")
            .borders(Borders::ALL);

        let desc_text = Paragraph::new(app.task_input.clone()).block(task_block);

        frame.render_widget(desc_text, area);
    }

    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Gray)),
            CurrentScreen::Adding => {
                Span::styled("Adding Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Sidebar => {
                Span::styled("Sidebar Open", Style::default().fg(Color::Blue))
            }
            CurrentScreen::Editing => Span::styled("Editing", Style::default().fg(Color::Green)),
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        {
            if app.adding_task == true {
                Span::styled("Adding Task Key", Style::default().fg(Color::Green))
            } else {
                Span::styled("Doing NOTHING", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => {
                Span::styled("(q)uit / (a)dd / (d)elete", Style::default().fg(Color::Red))
            }
            CurrentScreen::Adding => {
                Span::styled("(ESC) to cancel", Style::default().fg(Color::Red))
            }
            CurrentScreen::Sidebar => Span::styled(
                "(b) sidebar toggle / arrow keys to select",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => {
                Span::styled("(q)uit / (a)dd / (d)elete", Style::default().fg(Color::Red))
            }
            CurrentScreen::Editing => {
                Span::styled("(ESC) to quit", Style::default().fg(Color::Red))
            }
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));
    // ANCHOR_END: lower_navigation_key_hint

    // ANCHOR: lower_navigation_layout
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);
    // ANCHOR_END: lower_navigation_layout

    // ANCHOR: lower_navigation_rendering
    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    if let CurrentScreen::Exiting = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled("Save? (y/n)", Style::default().fg(Color::Red));
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }
}

// ANCHOR: centered_rect
/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
// ANCHOR_END: centered_rect

// ANCHOR_END: all
