mod models;
mod data_loader;
mod scheduler;
mod simulation;
mod reporter;

use std::error::Error;
use data_loader::load_project_from_csv;
use simulation::MonteCarloSimulator;
use reporter::SimulationReporter;

fn run_monte_carlo_simulation(filename: &str, iterations: usize) -> Result<(), Box<dyn Error>> {
    // Print methodology and setup
    SimulationReporter::print_methodology(iterations, filename);
    
    // Load project data
    let schedule = load_project_from_csv(filename)?;
    
    // Print loaded tasks
    SimulationReporter::print_loaded_tasks(&schedule);
    
    // Run simulation
    SimulationReporter::print_simulation_progress();
    let start_time = std::time::Instant::now();
    
    let simulator = MonteCarloSimulator::new(iterations);
    let results = simulator.run_simulation(schedule);
    
    let elapsed = start_time.elapsed();
    println!("   ({:.2} seconds)", elapsed.as_secs_f64());
    
    // Print results
    SimulationReporter::print_results(&results);
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("🎯 MONTE CARLO PROJECT PLANNING TOOL");
    println!("════════════════════════════════════");
    println!();

    // CSV dosyasını yükle ve simülasyonu çalıştır
    let filename = "project_data.csv";
    let iterations = 10000;

    match run_monte_carlo_simulation(filename, iterations) {
        Ok(_) => {
            println!();
            println!("✨ Simulation completed successfully!");
            println!("📝 Share these results with your managers to make data-driven decisions.");
        },
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            eprintln!();
            eprintln!("🔧 Troubleshooting:");
            eprintln!("   • Make sure project_data.csv file exists in the current directory");
            eprintln!("   • Check that the CSV format is correct");
            eprintln!("   • Check file permissions");
        }
    }

    Ok(())
}