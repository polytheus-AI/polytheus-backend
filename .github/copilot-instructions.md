# Project Presentation

This repository contains the **backend of Polytheus**, a **universal multi-agent system** designed to integrate the best AI models.  
Its purpose is to enable communication between different models and provide the most accurate and efficient responses to the user.

---

## Development Style

Development must follow a **strict Test-Driven Development (TDD)** workflow.

### Rules

#### 1. Definitions First

- Always start by creating **definitions** for each component (function, class, etc.).

#### 2. Documentation and Comments

- Every function, class, or module must include proper **documentation**.  
- Documentation must be written in **Rustdoc format**.  
- Avoid documenting trivial elements such as local variables.  
- Add comments only when needed — to clarify complex logic or design choices.  
- Do not add unnecessary comments.

#### 3. Testing

- Write **tests** immediately after defining and documenting each component.  
- Each test must be placed in a **dedicated test module** and use assertions (`assert!`, `assert_eq!`, etc.).  
- Every test must measure its **execution time**:
  1. Record the start time.  
  2. Record the end time.  
  3. Calculate and log the execution duration.

#### 4. Performance

- All code must be **optimized for performance**.  
- Use test execution times as performance benchmarks.

#### 5. Dependencies

- **Avoid external libraries** unless absolutely necessary.  
- Prefer using the **standard library** whenever possible.
