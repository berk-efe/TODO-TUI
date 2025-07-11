use csv::Writer;
use ratatui::widgets::{List, ListState};
use std::error::Error;
use std::fs::OpenOptions;

use std::io::{self, Read};

use std::path::Path;

use csv::{Reader, ReaderBuilder};
use serde::{Deserialize, Serialize};
use std::fs::File;

// ANCHOR: all

#[derive(PartialEq)]
pub enum CurrentScreen {
    Main,
    Adding,
    Sidebar,
    Editing,
    Exiting,
    AddingProj,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl Todo {
    fn new(title: String) -> Self {
        Self {
            title: String::new(),
            tasks: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub done: bool,
    pub desc: String,
}

impl Task {
    fn new(desc: String) -> Self {
        Self { done: false, desc }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

// ANCHOR: app_fields
pub struct App {
    pub running: bool,
    pub current_screen: CurrentScreen,
    pub input_buffer: String,
    pub todo: Vec<Todo>,
    pub current_todo: Option<Todo>,
    pub tasks_list_state: ListState,
    pub editing_task_at: Option<usize>,

    pub sidebar_state: ListState,
    // pub key_input: String,
}
// ANCHOR_END: app_fields

// ANCHOR: impl_new
impl App {
    pub fn new() -> App {
        App {
            running: true,
            current_screen: CurrentScreen::Main,
            input_buffer: String::new(),
            todo: Vec::new(),
            current_todo: None,
            tasks_list_state: ListState::default(),
            editing_task_at: None,

            sidebar_state: ListState::default(),
            // key_input: String::new(),
        }
    }
    // ANCHOR_END: impl_new

    pub fn save_task_value(&mut self) {
        let new_task = Task::new(self.input_buffer.clone());
        if let Some(todo) = self.current_todo.as_mut() {
            todo.tasks.insert(todo.tasks.len(), new_task);

            self.input_buffer = String::new();
            self.current_screen = CurrentScreen::Main;
        }
    }

    pub fn todo_tasks_available(&self) -> Option<&Vec<Task>> {
        self.current_todo.as_ref().map(|todo| &todo.tasks)
    }

    pub fn list_item_available(&mut self) -> Option<usize> {
        let index = self.tasks_list_state.selected()?;
        let tasks = self.todo_tasks_available()?;
        (index < tasks.len()).then_some(index)
    }
}
// ANCHOR_END: all
