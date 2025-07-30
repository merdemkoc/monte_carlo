use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use csv::Reader;
use serde::{Deserialize};
use rand::prelude::*;
use rand::rng;
use rand_distr::{Distribution, Normal};

#[derive(Debug, Deserialize, Clone)]
struct Task {
    #[serde(rename = "task_id")]
    id: String,
    #[serde(rename = "task_name")]
    name: String,
    #[serde(rename = "predecessor")]
    predecessors: String,
    #[serde(rename = "optimistic")]
    optimistic: f64,
    #[serde(rename = "most_likely")]
    most_likely: f64,
    #[serde(rename = "pessimistic")]
    pessimistic: f64,
    #[serde(rename = "PERT_Expected")]
    pert_expected: f64,
    #[serde(rename = "PERT_Variance")]
    pert_variance: f64,
    #[serde(rename = "PERT_StdDev")]
    pert_stddev: f64,
}

#[derive(Debug, Clone)]
struct ProjectSchedule {
    tasks: HashMap<String, Task>,
    dependencies: HashMap<String, Vec<String>>,
    task_durations: HashMap<String, f64>,
    early_start: HashMap<String, f64>,
    early_finish: HashMap<String, f64>,
}

impl ProjectSchedule {
    fn new() -> Self {
        ProjectSchedule {
            tasks: HashMap::new(),
            dependencies: HashMap::new(),
            task_durations: HashMap::new(),
            early_start: HashMap::new(),
            early_finish: HashMap::new(),
        }
    }

    fn load_from_csv(filename: &str) -> Result<Self, Box<dyn Error>> {
        let mut schedule = ProjectSchedule::new();
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

    fn generate_random_durations(&mut self, rng: &mut ThreadRng) {
        for (task_id, task) in &self.tasks {
            // Beta dağılımı simülasyonu için Normal dağılım kullanıyoruz
            // PERT expected ve standard deviation kullanarak
            let normal = Normal::new(task.pert_expected, task.pert_stddev).unwrap();
            let duration = normal.sample(rng).max(0.1); // Negatif süreleri önle
            self.task_durations.insert(task_id.clone(), duration);
        }
    }

    fn calculate_schedule(&mut self) -> f64 {
        self.early_start.clear();
        self.early_finish.clear();

        // Topological sort için task listesi
        let mut processed = HashSet::new();
        let mut processing_queue = Vec::new();

        // Başlangıç görevlerini bul (predecessorı olmayan)
        for (task_id, predecessors) in &self.dependencies {
            if predecessors.is_empty() {
                processing_queue.push(task_id.clone());
            }
        }

        while !processing_queue.is_empty() {
            let current_task = processing_queue.remove(0);

            if processed.contains(&current_task) {
                continue;
            }

            // Tüm predecessor'lar işlendi mi kontrol et
            let predecessors = self.dependencies.get(&current_task).unwrap();
            let all_predecessors_done = predecessors.iter()
                .all(|pred| processed.contains(pred));

            if !all_predecessors_done {
                // Predecessor'lar henüz hazır değil, sıranın sonuna ekle
                processing_queue.push(current_task);
                continue;
            }

            // Early start hesapla
            let early_start = if predecessors.is_empty() {
                0.0
            } else {
                predecessors.iter()
                    .map(|pred_id| self.early_finish.get(pred_id).unwrap_or(&0.0))
                    .fold(0.0f64, |acc, &x| acc.max(x))
            };

            let duration = self.task_durations.get(&current_task).unwrap_or(&0.0);
            let early_finish = early_start + duration;

            self.early_start.insert(current_task.clone(), early_start);
            self.early_finish.insert(current_task.clone(), early_finish);
            processed.insert(current_task.clone());

            // Bu görevin successor'larını kuyruğa ekle
            for (task_id, deps) in &self.dependencies {
                if deps.contains(&current_task) && !processed.contains(task_id) {
                    processing_queue.push(task_id.clone());
                }
            }
        }

        // Proje bitiş süresi = en geç biten görevin early finish'i
        self.early_finish.values().fold(0.0, |acc, &x| acc.max(x))
    }

    fn find_critical_path(&self) -> (Vec<String>, f64) {
        let project_duration = self.early_finish.values().fold(0.0f64, |acc, &x| acc.max(x));

        // Kritik yoldaki görevleri bul (early finish = project duration olanlar)
        let mut critical_tasks: Vec<String> = self.early_finish.iter()
            .filter(|(_, finish)| (**finish - project_duration).abs() < 0.001)
            .map(|(task_id, _)| task_id.clone())
            .collect();

        critical_tasks.sort();
        (critical_tasks, project_duration)
    }
}

fn run_monte_carlo_simulation(filename: &str, iterations: usize) -> Result<(), Box<dyn Error>> {
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

    let mut schedule = ProjectSchedule::load_from_csv(filename)?;
    let mut rng = rng();
    let mut durations = Vec::new();

    println!("📋 Loaded Tasks:");
    for (task_id, task) in &schedule.tasks {
        println!("   • {} - {} (PERT: {:.1} ± {:.1} days)",
                 task_id, task.name, task.pert_expected, task.pert_stddev);
    }
    println!();

    println!("⚡ Running simulation...");
    let start_time = std::time::Instant::now();

    let mut total_base_duration = 0.0;
    let mut total_invisible_tasks = 0.0;
    let mut total_system_risk_factor = 0.0;

    for iteration in 0..iterations {
        if iteration % 1000 == 0 {
            print!("   Progress: {:.1}%\r", (iteration as f64 / iterations as f64) * 100.0);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }

        schedule.generate_random_durations(&mut rng);
        let base_project_duration = schedule.calculate_schedule();

        // McKinsey bulgularını uygula

        // 1. Görünmeyen görevler için ek süre (proje toplam süresinin %10-15'i)
        let invisible_tasks_factor = rng.random_range(0.10..=0.15);
        let invisible_tasks_duration = base_project_duration * invisible_tasks_factor;

        // 2. Sistem düzeyinde risk faktörü (1.0 - 1.35 arası)
        let system_risk_factor = rng.random_range(1.0..=1.35);

        // Final proje süresi hesaplama
        let final_project_duration = (base_project_duration + invisible_tasks_duration) * system_risk_factor;

        // İstatistik topla
        total_base_duration += base_project_duration;
        total_invisible_tasks += invisible_tasks_duration;
        total_system_risk_factor += system_risk_factor;

        durations.push(final_project_duration);
    }

    let elapsed = start_time.elapsed();
    println!("   ✅ {} iterations completed ({:.2} seconds)", iterations, elapsed.as_secs_f64());
    println!();

    // McKinsey faktörlerinin ortalamalarını hesapla
    let avg_base_duration = total_base_duration / iterations as f64;
    let avg_invisible_tasks = total_invisible_tasks / iterations as f64;
    let avg_system_risk_factor = total_system_risk_factor / iterations as f64;


    // Sonuçları sırala
    durations.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // İstatistikleri hesapla
    let mean = durations.iter().sum::<f64>() / durations.len() as f64;
    let median = durations[durations.len() / 2];
    let p80 = durations[(durations.len() as f64 * 0.80) as usize];
    let p95 = durations[(durations.len() as f64 * 0.95) as usize];
    let min = durations[0];
    let max = durations[durations.len() - 1];

    // Kritik yol analizi (ortalama değerlerle)
    schedule.task_durations.clear();
    for (task_id, task) in &schedule.tasks {
        schedule.task_durations.insert(task_id.clone(), task.pert_expected);
    }
    schedule.calculate_schedule();
    let (critical_path, _) = schedule.find_critical_path();

    // Sonuçları yazdır
    println!("📈 MONTE CARLO SIMULATION RESULTS");
    println!("═══════════════════════════════════════");
    println!("   📝 Note: All week calculations are in WORK WEEKS (5 business days)");
    println!();
    println!("🎯 Basic Statistics:");
    println!("   • Average Duration:  {:.1} days ({:.1} work weeks)", mean, mean / 5.0);
    println!("   • Median Duration:   {:.1} days ({:.1} work weeks)", median, median / 5.0);
    println!("   • Minimum Duration:  {:.1} days ({:.1} work weeks)", min, min / 5.0);
    println!("   • Maximum Duration:  {:.1} days ({:.1} work weeks)", max, max / 5.0);
    println!();

    println!("🎲 Probability Distribution:");
    println!("   • 50% Probability:   Completes within {:.1} days ({:.1} work weeks)", median, median / 5.0);
    println!("   • 80% Probability:   Completes within {:.1} days ({:.1} work weeks)", p80, p80 / 5.0);
    println!("   • 95% Probability:   Completes within {:.1} days ({:.1} work weeks)", p95, p95 / 5.0);
    println!();

    println!("📋 Buffer Analysis (Including McKinsey 35% Variance):");
    let buffer_80 = p80 - mean;
    let buffer_95 = p95 - mean;
    let mckinsey_buffer = mean * 0.35; // Referans için McKinsey'nin %35'i
    println!("   • For 80% Confidence: +{:.1} days buffer ({:.1}% addition)", buffer_80, (buffer_80 / mean) * 100.0);
    println!("   • For 95% Confidence: +{:.1} days buffer ({:.1}% addition)", buffer_95, (buffer_95 / mean) * 100.0);
    println!("   • McKinsey Reference: +{:.1} days buffer (35% addition)", mckinsey_buffer);
    println!("   • Average Hidden Tasks:            +{:.1} days ({:.1}% addition)", avg_invisible_tasks, (avg_invisible_tasks / avg_base_duration) * 100.0);
    println!("   • Average System Risk Multiplier:  x{:.2} ({:.1}% increase)", avg_system_risk_factor, (avg_system_risk_factor - 1.0) * 100.0);
    println!();

    println!("🛤️  Critical Path Analysis:");
    println!("   • Critical Tasks: {}", critical_path.join(" → "));
    println!("   • Critical Path Duration: {:.1} days", schedule.early_finish.values().fold(0.0f64, |acc, &x| acc.max(x)));
    println!();
    println!("💡 RECOMMENDATIONS:");
    // Calculate comprehensive recommendation: (80% Probability + Hidden Tasks) * System Risk
    let recommended_duration = (p80 + avg_invisible_tasks) * avg_system_risk_factor;
    
    println!("   • Recommended client estimate: {} work weeks ({:.0} days)", (recommended_duration / 5.0).ceil(), recommended_duration.ceil());
    println!("     - Formula: (80% confidence {:.0} days + Hidden tasks {:.0} days) × System risk {:.2} = {:.0} days", p80, avg_invisible_tasks, avg_system_risk_factor, recommended_duration);  
    println!("   • Add {} work weeks ({:.0} days) buffer for internal planning", ((p95 - recommended_duration) / 5.0).ceil().max(1.0), (p95 - recommended_duration).ceil().max(5.0));
    println!("   • Pay special attention to critical path tasks");
    println!("   • McKinsey factors already integrated in recommendation");

    // Risk analizi
    println!();
    println!("⚠️  Risk Analysis:");
    let mut risk_tasks: Vec<_> = schedule.tasks.iter().collect();
    risk_tasks.sort_by(|a, b| b.1.pert_stddev.partial_cmp(&a.1.pert_stddev).unwrap());

    println!("   Highest Risk Tasks (High Uncertainty):");
    for (i, (task_id, task)) in risk_tasks.iter().take(3).enumerate() {
        let risk_level = if task.pert_stddev > 2.0 { "🔴 High" }
        else if task.pert_stddev > 1.0 { "🟡 Medium" }
        else { "🟢 Low" };
        println!("   {}. {} - {} ({} Risk, ±{:.1} days)",
                 i + 1, task_id, task.name, risk_level, task.pert_stddev);
    }

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