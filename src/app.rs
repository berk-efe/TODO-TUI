use csv::Writer;
use std::{error::Error};
use std::fs::OpenOptions;

use std::io::{self, Read};

use serde::{Serialize, Deserialize};
use std::fs::File;
use csv::{Reader, ReaderBuilder};


// ANCHOR: all

pub enum CurrentScreen{
    Main,
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
            .delimiter(b';')
            .from_reader(data.as_bytes());

        

        Ok(())
    }

   
}
// ANCHOR_END: all