# Monte Carlo Project Planning Tool

A Rust-based Monte Carlo simulation tool for project planning and scheduling that incorporates McKinsey methodology for realistic project duration estimation.

## Overview

This tool performs Monte Carlo simulations on project schedules to provide probabilistic estimates of project completion times. It goes beyond basic PERT calculations by incorporating real-world factors identified in McKinsey research:

- **Hidden Tasks**: Adds 10-15% additional time for unforeseen tasks
- **System Risk**: Applies a 1.0-1.35x multiplier for systemic risks
- **Statistical Analysis**: Provides confidence intervals at 50%, 80%, and 95% levels

## Features

- 📊 **Monte Carlo Simulation**: Runs thousands of iterations for statistical accuracy
- 🎯 **PERT-based Task Modeling**: Uses optimistic, most likely, and pessimistic estimates
- 🛤️ **Critical Path Analysis**: Identifies project bottlenecks
- ⚠️ **Risk Assessment**: Highlights tasks with highest uncertainty
- 📈 **McKinsey Methodology**: Incorporates research-backed planning factors
- 🔍 **Detailed Reporting**: Comprehensive analysis with actionable recommendations

## Installation

### Prerequisites

- Rust (2024 edition or later)
- Cargo package manager

### Build from Source

```bash
git clone <repository-url>
cd monte_carlo
cargo build --release
```

## Usage

### 1. Prepare Your Project Data

Create a CSV file named `project_data.csv` with the following columns:

```csv
task_id,task_name,predecessor,optimistic,most_likely,pessimistic,PERT_Expected,PERT_Variance,PERT_StdDev
T1,Task 1,,5,10,15,10.0,2.8,1.7
T2,Task 2,T1,3,5,8,5.2,0.7,0.8
```

**Column Descriptions:**
- `task_id`: Unique identifier for the task
- `task_name`: Descriptive name of the task
- `predecessor`: Comma-separated list of prerequisite tasks (empty for start tasks)
- `optimistic`: Best-case scenario duration (days)
- `most_likely`: Most realistic duration estimate (days)
- `pessimistic`: Worst-case scenario duration (days)
- `PERT_Expected`: PERT expected value (calculated as (O + 4M + P) / 6)
- `PERT_Variance`: PERT variance (calculated as ((P - O) / 6)²)
- `PERT_StdDev`: PERT standard deviation (square root of variance)

### 2. Run the Simulation

```bash
cargo run
```

The tool will automatically:
- Load `project_data.csv`
- Run 10,000 Monte Carlo iterations
- Generate comprehensive analysis and recommendations

## Output Analysis

The tool provides several key metrics:

### McKinsey Factor Analysis
- **Base Project Duration**: Core project timeline from task estimates
- **Hidden Tasks Impact**: Additional time for unforeseen work (10-15%)
- **System Risk Multiplier**: Systemic risk factor (1.0-1.35x)
- **Total McKinsey Effect**: Combined impact of all factors

### Statistical Results
- **50% Confidence**: Median completion time
- **80% Confidence**: Conservative estimate for client communication
- **95% Confidence**: High-confidence buffer for internal planning

### Risk Assessment
- **Critical Path**: Sequence of tasks that determine project duration
- **High-Risk Tasks**: Tasks with highest uncertainty (large standard deviation)
- **Buffer Recommendations**: Suggested time buffers for different confidence levels

## Example Output

```
🎯 MONTE CARLO PROJECT PLANNING TOOL
════════════════════════════════════

📊 Simulation Results:
   • 50% Probability: 45.2 days (6.5 weeks)
   • 80% Probability: 52.1 days (7.4 weeks)  
   • 95% Probability: 58.7 days (8.4 weeks)

🛤️ Critical Path: T1 → T2 → T3 → T6 → T7 → T8

💡 RECOMMENDATIONS:
   • Tell client: 8 weeks (80% confidence)
   • Internal buffer: +1 week (95% confidence)
   • Focus on critical path tasks
```

## Technical Details

### Dependencies

- `csv`: CSV file parsing
- `rand` & `rand_distr`: Random number generation and statistical distributions  
- `serde`: Data serialization/deserialization

### Methodology

1. **Task Duration Sampling**: Each iteration samples task durations from normal distributions based on PERT parameters
2. **Schedule Calculation**: Uses topological sorting to calculate early start/finish times respecting dependencies
3. **McKinsey Adjustments**: Applies research-based factors for hidden tasks and systemic risks
4. **Statistical Analysis**: Aggregates results across iterations to provide confidence intervals

### Algorithms

- **Critical Path Method (CPM)**: Forward pass calculation for project scheduling
- **Topological Sort**: Ensures tasks are processed in dependency order
- **Monte Carlo Sampling**: Statistical simulation for uncertainty modeling

## Configuration

You can modify simulation parameters in `src/main.rs`:

```rust
let filename = "project_data.csv";  // CSV file path
let iterations = 10000;             // Number of simulation runs
```

For different McKinsey factors, adjust the ranges in the simulation loop:
```rust
let invisible_tasks_factor = rng.gen_range(0.10..=0.15);  // 10-15% hidden tasks
let system_risk_factor = rng.gen_range(1.0..=1.35);       // 1.0-1.35x system risk
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is available under standard open source terms. See LICENSE file for details.

## Support

For issues, questions, or contributions, please create an issue in the project repository.