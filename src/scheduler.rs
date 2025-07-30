use std::collections::HashSet;
use rand::prelude::*;
use rand_distr::{Distribution, Normal};
use crate::models::ProjectSchedule;

impl ProjectSchedule {
    pub fn generate_random_durations(&mut self, rng: &mut ThreadRng) {
        for (task_id, task) in &self.tasks {
            // Beta dağılımı simülasyonu için Normal dağılım kullanıyoruz
            // PERT expected ve standard deviation kullanarak
            let normal = Normal::new(task.pert_expected, task.pert_stddev).unwrap();
            let duration = normal.sample(rng).max(0.1); // Negatif süreleri önle
            self.task_durations.insert(task_id.clone(), duration);
        }
    }

    pub fn calculate_schedule(&mut self) -> f64 {
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

    pub fn find_critical_path(&self) -> (Vec<String>, f64) {
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