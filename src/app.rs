use csv::Writer;
use ratatui::widgets::ListState;
use std::{error::Error};
use std::fs::OpenOptions;

use std::io::{self, Read};

use serde::{Serialize, Deserialize};
use std::fs::File;
use csv::{Reader, ReaderBuilder};


// ANCHOR: all

#[derive(PartialEq)]
pub enum CurrentScreen{
    Main,
    Adding,
    Editing,
    Exiting,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub done: bool,
    pub desc: String,
}

impl Task {
    fn new(desc:String) -> Self {
        Self {
            done: false,
            desc,
        }
    }
}



// ANCHOR: app_fields
pub struct App {
    pub running: bool,
    pub adding_task: bool,
    pub current_screen: CurrentScreen,
    pub task_input: String,
    pub tasks: Vec<Task>,
    pub tasks_list_state: ListState,
    pub editing_task_at: Option<usize>,

    // pub key_input: String,
}
// ANCHOR_END: app_fields

// ANCHOR: impl_new
impl App {
    pub fn new() -> App {
        App {
            running: true,
            adding_task: false,
            current_screen: CurrentScreen::Main,
            task_input: String::new(),
            tasks: Vec::new(),
            tasks_list_state: ListState::default(),
            editing_task_at: None,
            
            // key_input: String::new(),
        }
    }
    // ANCHOR_END: impl_new

    pub fn save_task_value(&mut self) {
        let new_task = Task::new(self.task_input.clone());
        self.tasks.insert(self.tasks.len(), new_task);


        self.task_input = String::new();
        self.adding_task = false;
        self.current_screen = CurrentScreen::Main;
    }

    pub fn write_tasks_to_csv(&mut self) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("tasks.csv")?;

        let mut writer = Writer::from_writer(file);

        for task in &self.tasks {
            writer.serialize(task)?;
        }

        writer.flush()?;
        Ok(())
    }

    pub fn read_tasks_from_csv(&mut self) -> Result<(), Box<dyn Error>> {
        let mut file = File::open("tasks.csv")?;
        let mut data = String::new();
        file.read_to_string(&mut data);
        
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_reader(data.as_bytes());

        for result in rdr.deserialize() {
            let task: Task = result?;
            self.tasks.push(task);
        }

        Ok(())
    }

    pub fn list_item_available(&mut self) -> Option<usize> {
        if let Some(selected_index) = self.tasks_list_state.selected() {
            if selected_index < self.tasks.len() {
                Some(selected_index)
            }else {
                None
            }
        }else {
            None
        }
    }

   
}
// ANCHOR_END: all