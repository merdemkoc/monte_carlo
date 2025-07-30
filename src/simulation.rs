use rand::prelude::*;
use rand::rng;
use crate::models::{ProjectSchedule, SimulationResults};

pub struct MonteCarloSimulator {
    pub iterations: usize,
}

impl MonteCarloSimulator {
    pub fn new(iterations: usize) -> Self {
        MonteCarloSimulator { iterations }
    }

    pub fn run_simulation(&self, mut schedule: ProjectSchedule) -> SimulationResults {
        let mut rng = rng();
        let mut durations = Vec::new();

        let mut total_base_duration = 0.0;
        let mut total_invisible_tasks = 0.0;
        let mut total_system_risk_factor = 0.0;

        for iteration in 0..self.iterations {
            if iteration % 1000 == 0 {
                print!("   Progress: {:.1}%\r", (iteration as f64 / self.iterations as f64) * 100.0);
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

        println!("   ✅ {} iterations completed", self.iterations);

        // Ortalama değerleri hesapla
        let avg_base_duration = total_base_duration / self.iterations as f64;
        let avg_invisible_tasks = total_invisible_tasks / self.iterations as f64;
        let avg_system_risk_factor = total_system_risk_factor / self.iterations as f64;

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
        let (critical_path, critical_path_duration) = schedule.find_critical_path();

        SimulationResults {
            durations,
            mean,
            median,
            p80,
            p95,
            min,
            max,
            avg_base_duration,
            avg_invisible_tasks,
            avg_system_risk_factor,
            critical_path,
            critical_path_duration,
        }
    }
}