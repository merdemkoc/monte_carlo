# PERT (Program Evaluation and Review Technique) Methodology

## Overview

PERT is a statistical project management technique used to analyze and estimate the time required to complete project tasks. This methodology forms the foundation of our Monte Carlo simulation by providing probabilistic estimates for individual task durations.

## Core Concepts

### Three-Point Estimation

PERT uses three time estimates for each task:

- **Optimistic (O)**: The minimum time required if everything goes perfectly
- **Most Likely (M)**: The most realistic time estimate under normal conditions  
- **Pessimistic (P)**: The maximum time required if everything goes wrong

### PERT Calculations

#### Expected Time (Te)
The PERT expected time uses a weighted average that emphasizes the most likely estimate:

```
Te = (O + 4M + P) / 6
```

This formula gives four times more weight to the most likely estimate than to the optimistic or pessimistic estimates.

#### Variance (σ²)
The PERT variance measures the uncertainty or risk associated with the task:

```
σ² = ((P - O) / 6)²
```

#### Standard Deviation (σ)
The standard deviation is the square root of variance:

```
σ = √(σ²) = (P - O) / 6
```

## Statistical Foundation

### Beta Distribution Approximation

PERT assumes that task durations follow a Beta distribution, which is:
- **Bounded**: Has definite minimum and maximum values
- **Flexible**: Can be symmetric or skewed
- **Realistic**: Reflects real-world project constraints

The Beta distribution is approximated using a Normal distribution with:
- Mean = Te (PERT expected time)
- Standard Deviation = σ (PERT standard deviation)

### Why This Works

1. **Central Limit Theorem**: When multiple tasks are combined, the total duration approaches a normal distribution
2. **Real-world Behavior**: Most tasks have natural lower and upper bounds
3. **Expert Judgment**: Three-point estimates capture human intuition about uncertainty

## Implementation in Monte Carlo

### Sampling Process

For each simulation iteration:

1. **Load PERT Parameters**: Read optimistic, most likely, and pessimistic estimates from CSV
2. **Calculate Statistics**: Compute PERT expected time and standard deviation
3. **Generate Sample**: Use Normal distribution to sample a duration for each task
4. **Apply Constraints**: Ensure no negative durations (minimum 0.1 days)

### Example Calculation

Given estimates for a task:
- Optimistic: 5 days
- Most Likely: 10 days  
- Pessimistic: 15 days

**PERT Calculations:**
```
Expected Time = (5 + 4×10 + 15) / 6 = 60 / 6 = 10.0 days
Variance = ((15 - 5) / 6)² = (10/6)² = 2.78
Standard Deviation = √2.78 = 1.67 days
```

**Interpretation:**
- The task will most likely take 10 days
- There's approximately 68% probability it will complete between 8.33 and 11.67 days
- There's approximately 95% probability it will complete between 6.66 and 13.34 days

## Advantages of PERT

### 1. Captures Uncertainty
- Acknowledges that project estimates are inherently uncertain
- Provides statistical measures of risk for each task

### 2. Uses Expert Knowledge
- Leverages team expertise through three-point estimates
- Balances optimistic and pessimistic scenarios

### 3. Mathematical Foundation
- Based on established statistical principles
- Enables quantitative risk analysis

### 4. Practical Implementation
- Simple to understand and apply
- Widely accepted in project management

## Limitations and Considerations

### 1. Distribution Assumptions
- Assumes Beta distribution may not always reflect reality
- Normal approximation works best for larger projects

### 2. Estimation Bias
- Estimates may be influenced by anchoring or other cognitive biases
- Requires calibrated estimators for accuracy

### 3. Independence Assumption
- Assumes tasks are independent (no correlation between durations)
- Real projects may have systematic factors affecting multiple tasks

## Best Practices

### 1. Calibrated Estimators
- Train team members on three-point estimation
- Use historical data to validate estimates
- Regular estimation retrospectives

### 2. Granular Tasks
- Break large tasks into smaller, more predictable units
- Aim for tasks between 1-10 days duration
- Avoid very long or very short tasks

### 3. Risk Identification
- Tasks with high standard deviation need special attention
- Consider external dependencies and constraints
- Document assumptions behind estimates

### 4. Continuous Refinement
- Update estimates as more information becomes available
- Track actual vs. estimated durations
- Improve estimation accuracy over time

## Integration with Project Management

### Critical Path Method (CPM)
PERT estimates feed into CPM calculations to:
- Identify the critical path (longest sequence of dependent tasks)
- Calculate early start and finish times
- Determine project duration

### Risk Management
PERT statistics help identify:
- **High-risk tasks**: Large standard deviation
- **Schedule buffers**: Based on confidence intervals
- **Resource allocation**: Focus on critical and risky tasks

### Communication
PERT provides data-driven basis for:
- Client communications (confidence intervals)
- Internal planning (buffer calculations)
- Stakeholder reporting (probabilistic outcomes)

## Mathematical Details

### Normal Distribution Sampling
In our implementation, we sample from:
```
N(μ = Te, σ² = PERT_variance)
```

Where:
- μ (mu) = PERT expected time
- σ² (sigma squared) = PERT variance

### Minimum Duration Constraint
To prevent unrealistic negative durations:
```rust
let duration = normal.sample(rng).max(0.1);
```

This ensures all sampled durations are at least 0.1 days (about 1 hour).

## Conclusion

PERT methodology provides the statistical foundation that makes Monte Carlo project simulation both mathematically sound and practically useful. By converting expert estimates into probability distributions, PERT enables quantitative risk analysis and confidence-based project planning.

The combination of PERT's three-point estimation with Monte Carlo simulation creates a powerful tool for understanding and communicating project uncertainty, leading to more informed decision-making and better project outcomes.