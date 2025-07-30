use crate::models::{ProjectSchedule, SimulationResults};

pub struct SimulationReporter;

impl SimulationReporter {
    pub fn print_methodology(iterations: usize, filename: &str) {
        println!("🚀 Starting Monte Carlo Project Planning Simulation...");
        println!();
        println!("📘 CALCULATION METHODOLOGY:");
        println!("This simulation follows these steps in each iteration: (1) PERT distribution is calculated from your");
        println!("Optimistic-Most Likely-Pessimistic estimates for each task, (2) A random duration is generated from");
        println!("this distribution for each task, (3) Critical path is calculated based on dependencies to find base");
        println!("project duration, (4) Hidden tasks are added (10-15% of project time) based on McKinsey findings,");
        println!("(5) Finally, result is multiplied by a random factor (1.0x-1.35x) to model systemic risks. This");
        println!("process is repeated {} times to obtain a realistic probability distribution and provide estimates", iterations);
        println!("at 50%, 80%, and 95% confidence levels.");
        println!();
        println!("📊 Simulation Parameters:");
        println!("   • File: {}", filename);
        println!("   • Number of Iterations: {}", iterations);
        println!("   • Target Confidence Levels: 50%, 80%, 95%");
        println!("   • McKinsey Settings: Hidden tasks 10-15%, System risk 1.0-1.35x");
        println!();
    }

    pub fn print_loaded_tasks(schedule: &ProjectSchedule) {
        println!("📋 Loaded Tasks:");
        for (task_id, task) in &schedule.tasks {
            println!("   • {} - {} (PERT: {:.1} ± {:.1} days)",
                     task_id, task.name, task.pert_expected, task.pert_stddev);
        }
        println!();
    }

    pub fn print_simulation_progress() {
        println!("⚡ Running simulation...");
    }

    pub fn print_results(results: &SimulationResults) {
        println!();
        
        // Sonuçları yazdır
        println!("📈 MONTE CARLO SIMULATION RESULTS");
        println!("═══════════════════════════════════════");
        println!("   📝 Note: All week calculations are in WORK WEEKS (5 business days)");
        println!();
        
        Self::print_basic_statistics(results);
        Self::print_probability_distribution(results);
        Self::print_buffer_analysis(results);
        Self::print_critical_path_analysis(results);
        Self::print_recommendations(results);
        Self::print_risk_analysis(results);
    }

    fn print_basic_statistics(results: &SimulationResults) {
        println!("🎯 Basic Statistics:");
        println!("   • Average Duration:  {:.1} days ({:.1} work weeks)", results.mean, results.mean / 5.0);
        println!("   • Median Duration:   {:.1} days ({:.1} work weeks)", results.median, results.median / 5.0);
        println!("   • Minimum Duration:  {:.1} days ({:.1} work weeks)", results.min, results.min / 5.0);
        println!("   • Maximum Duration:  {:.1} days ({:.1} work weeks)", results.max, results.max / 5.0);
        println!();
    }

    fn print_probability_distribution(results: &SimulationResults) {
        println!("🎲 Probability Distribution:");
        println!("   • 50% Probability:   Completes within {:.1} days ({:.1} work weeks)", results.median, results.median / 5.0);
        println!("   • 80% Probability:   Completes within {:.1} days ({:.1} work weeks)", results.p80, results.p80 / 5.0);
        println!("   • 95% Probability:   Completes within {:.1} days ({:.1} work weeks)", results.p95, results.p95 / 5.0);
        println!();
    }

    fn print_buffer_analysis(results: &SimulationResults) {
        println!("📋 Buffer Analysis (Including McKinsey 35% Variance):");
        let buffer_80 = results.p80 - results.mean;
        let buffer_95 = results.p95 - results.mean;
        let mckinsey_buffer = results.mean * 0.35; // Referans için McKinsey'nin %35'i
        println!("   • For 80% Confidence: +{:.1} days buffer ({:.1}% addition)", buffer_80, (buffer_80 / results.mean) * 100.0);
        println!("   • For 95% Confidence: +{:.1} days buffer ({:.1}% addition)", buffer_95, (buffer_95 / results.mean) * 100.0);
        println!("   • McKinsey Reference: +{:.1} days buffer (35% addition)", mckinsey_buffer);
        println!("   • Average Hidden Tasks:            +{:.1} days ({:.1}% addition)", results.avg_invisible_tasks, (results.avg_invisible_tasks / results.avg_base_duration) * 100.0);
        println!("   • Average System Risk Multiplier:  x{:.2} ({:.1}% increase)", results.avg_system_risk_factor, (results.avg_system_risk_factor - 1.0) * 100.0);
        println!();
    }

    fn print_critical_path_analysis(results: &SimulationResults) {
        println!("🛤️  Critical Path Analysis:");
        println!("   • Critical Tasks: {}", results.critical_path.join(" → "));
        println!("   • Critical Path Duration: {:.1} days", results.critical_path_duration);
        println!();
    }

    fn print_recommendations(results: &SimulationResults) {
        println!("💡 RECOMMENDATIONS:");
        println!("   • Recommended client estimate: {} work weeks ({:.0} days)", (results.p80 / 5.0).ceil(), results.p80.ceil());
        let hidden_task_percentage = (results.avg_invisible_tasks / results.avg_base_duration) * 100.0;
        let system_risk_increase = (results.avg_system_risk_factor - 1.0) * results.avg_base_duration;
        println!("     Note: This 80% estimate includes PERT task variations + hidden tasks ({:.0}% - {:.0} days) + system risk ({:.2}x - {:.0} days)", 
                 hidden_task_percentage, results.avg_invisible_tasks, results.avg_system_risk_factor, system_risk_increase);
        println!("   • Add {} work weeks ({:.0} days) buffer for internal planning", ((results.p95 - results.p80) / 5.0).ceil().max(1.0), (results.p95 - results.p80).ceil().max(5.0));
        println!("   • Pay special attention to critical path tasks");
        println!("   • 80% confidence already includes hidden tasks and risk assessment");
        println!();
    }

    fn print_risk_analysis(results: &SimulationResults) {
        // This would need access to the original schedule to show risk tasks
        // For now, we'll keep it simple
        println!("⚠️  Risk Analysis:");
        println!("   • Monitor tasks with high uncertainty (high standard deviation)");
        println!("   • Focus on critical path tasks for schedule control");
        println!("   • Consider additional risk mitigation for high-risk tasks");
    }
}