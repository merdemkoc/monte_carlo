use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use csv::Reader;
use crate::models::{Task, ProjectSchedule};
use std::collections::HashMap;

pub fn load_project_from_csv(filename: &str) -> Result<ProjectSchedule, Box<dyn Error>> {
    let mut schedule = ProjectSchedule {
        tasks: HashMap::new(),
        dependencies: HashMap::new(),
        task_durations: HashMap::new(),
        early_start: HashMap::new(),
        early_finish: HashMap::new(),
    };
    
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut csv_reader = Reader::from_reader(reader);

    for result in csv_reader.deserialize() {
        let task: Task = result?;

        // Parse predecessors
        let predecessors: Vec<String> = if task.predecessors.trim().is_empty() {
            Vec::new()
        } else {
            task.predecessors.split(',')
                .map(|s| s.trim().to_string())
                .collect()
        };

        schedule.dependencies.insert(task.id.clone(), predecessors);
        schedule.tasks.insert(task.id.clone(), task);
    }

    Ok(schedule)
}