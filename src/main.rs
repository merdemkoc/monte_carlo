use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use csv::Reader;
use serde::{Deserialize};
use rand::prelude::*;
use rand::thread_rng;
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
    println!("🚀 Monte Carlo Proje Planlama Simülasyonu Başlatılıyor...");
    println!("📊 Simülasyon Parametreleri:");
    println!("   • Dosya: {}", filename);
    println!("   • İterasyon Sayısı: {}", iterations);
    println!("   • Hedef Güven Seviyeleri: %50, %80, %95");
    println!();

    let mut schedule = ProjectSchedule::load_from_csv(filename)?;
    let mut rng = thread_rng();
    let mut durations = Vec::new();

    println!("📋 Yüklenen Görevler:");
    for (task_id, task) in &schedule.tasks {
        println!("   • {} - {} (PERT: {:.1} ± {:.1} gün)",
                 task_id, task.name, task.pert_expected, task.pert_stddev);
    }
    println!();

    println!("⚡ Simülasyon çalışıyor...");
    let start_time = std::time::Instant::now();

    for iteration in 0..iterations {
        if iteration % 1000 == 0 {
            print!("   İlerleme: {:.1}%\r", (iteration as f64 / iterations as f64) * 100.0);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }

        schedule.generate_random_durations(&mut rng);
        let project_duration = schedule.calculate_schedule();
        durations.push(project_duration);
    }

    let elapsed = start_time.elapsed();
    println!("   ✅ {} iterasyon tamamlandı ({:.2} saniye)", iterations, elapsed.as_secs_f64());
    println!();

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
    println!("📈 MONTE CARLO SİMÜLASYON SONUÇLARI");
    println!("═══════════════════════════════════════");
    println!();
    println!("🎯 Temel İstatistikler:");
    println!("   • Ortalama Süre:     {:.1} gün ({:.1} hafta)", mean, mean / 7.0);
    println!("   • Medyan Süre:       {:.1} gün ({:.1} hafta)", median, median / 7.0);
    println!("   • Minimum Süre:      {:.1} gün ({:.1} hafta)", min, min / 7.0);
    println!("   • Maksimum Süre:     {:.1} gün ({:.1} hafta)", max, max / 7.0);
    println!();

    println!("🎲 Olasılık Dağılımı:");
    println!("   • %50 İhtimalle:     {:.1} gün ({:.1} hafta) içinde biter", median, median / 7.0);
    println!("   • %80 İhtimalle:     {:.1} gün ({:.1} hafta) içinde biter", p80, p80 / 7.0);
    println!("   • %95 İhtimalle:     {:.1} gün ({:.1} hafta) içinde biter", p95, p95 / 7.0);
    println!();

    println!("📋 Buffer Analizi:");
    let buffer_80 = p80 - mean;
    let buffer_95 = p95 - mean;
    println!("   • %80 Güven için:    +{:.1} gün buffer ({:.1}% ekleme)", buffer_80, (buffer_80 / mean) * 100.0);
    println!("   • %95 Güven için:    +{:.1} gün buffer ({:.1}% ekleme)", buffer_95, (buffer_95 / mean) * 100.0);
    println!();

    println!("🛤️  Kritik Yol Analizi:");
    println!("   • Kritik Görevler: {}", critical_path.join(" → "));
    println!("   • Kritik Yol Süresi: {:.1} gün", schedule.early_finish.values().fold(0.0f64, |acc, &x| acc.max(x)));
    println!();

    println!("💡 ÖNERİLER:");
    println!("   • Müşteriye %80 güvenle {} hafta ({:.0} gün) söyleyebilirsiniz", (p80 / 7.0).ceil(), p80.ceil());
    println!("   • İç planlama için {} hafta ({:.0} gün) buffer ekleyin", ((p95 - p80) / 7.0).ceil(), (p95 - p80).ceil());
    println!("   • Kritik yoldaki görevlere özel dikkat gösterin");

    // Risk analizi
    println!();
    println!("⚠️  Risk Analizi:");
    let mut risk_tasks: Vec<_> = schedule.tasks.iter().collect();
    risk_tasks.sort_by(|a, b| b.1.pert_stddev.partial_cmp(&a.1.pert_stddev).unwrap());

    println!("   En Riskli Görevler (Yüksek Belirsizlik):");
    for (i, (task_id, task)) in risk_tasks.iter().take(3).enumerate() {
        let risk_level = if task.pert_stddev > 2.0 { "🔴 Yüksek" }
        else if task.pert_stddev > 1.0 { "🟡 Orta" }
        else { "🟢 Düşük" };
        println!("   {}. {} - {} ({} Risk, ±{:.1} gün)",
                 i + 1, task_id, task.name, risk_level, task.pert_stddev);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("🎯 MONTE CARLO PROJE PLANLAMA ARACI");
    println!("════════════════════════════════════");
    println!();

    // CSV dosyasını yükle ve simülasyonu çalıştır
    let filename = "project_data.csv";
    let iterations = 10000;

    match run_monte_carlo_simulation(filename, iterations) {
        Ok(_) => {
            println!();
            println!("✨ Simülasyon başarıyla tamamlandı!");
            println!("📝 Bu sonuçları yöneticilerinizle paylaşarak data-driven karar verebilirsiniz.");
        },
        Err(e) => {
            eprintln!("❌ Hata: {}", e);
            eprintln!();
            eprintln!("🔧 Sorun Giderme:");
            eprintln!("   • project_data.csv dosyasının mevcut dizinde olduğundan emin olun");
            eprintln!("   • CSV formatının doğru olduğunu kontrol edin");
            eprintln!("   • Dosya izinlerini kontrol edin");
        }
    }

    Ok(())
}
