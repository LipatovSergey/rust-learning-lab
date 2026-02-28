# Rust Bank Account CLI

## Technical Specification (Chapters 1–5 Only)

---

# 1. Project Overview

**Project Name:** rust-bank-account
**Type:** CLI application
**Language:** Rust
**Scope Restriction:** Only features from Rust Book Chapters 1–5 may be used.

This project implements a console-based banking simulation for managing a single bank account.

---

# 2. Functional Requirements

## 2.1 Account Model

The system must define a `BankAccount` structure with:

- `owner: String`
- `balance: i32`

---

## 2.2 Associated Function

The structure must provide:

```
fn new(owner: String, balance: i32) -> Self
```

Purpose:

- Creates and returns a new `BankAccount` instance.

---

## 2.3 Methods

### 2.3.1 Deposit

```
fn deposit(&mut self, amount: i32)
```

Rules:

- `amount` must be greater than 0
- If valid → increase balance
- If invalid → print error message
- Must not panic

---

### 2.3.2 Withdraw

```
fn withdraw(&mut self, amount: i32)
```

Rules:

- `amount` must be greater than 0
- `amount` must not exceed current balance
- If valid → decrease balance
- If invalid → print error message
- Balance must never become negative

---

### 2.3.3 Display Balance

```
fn print_balance(&self)
```

Behavior:

- Prints owner and current balance
- Must not modify state

---

# 3. CLI Interface Requirements

The application must:

1. Create one `BankAccount` instance at startup.
2. Enter a continuous loop.
3. Display the following menu:

```
1) Deposit
2) Withdraw
3) Show balance
0) Exit
```

4. Read user input from stdin.
5. Parse input using `parse()` and `match`.
6. Execute the corresponding action.

---

# 4. Input Handling Rules

- Invalid command → print error and continue loop
- Invalid numeric input → print error and retry
- Input parsing must use `match`
- No advanced error handling patterns

---

# 5. Technical Constraints

The following features are strictly forbidden:

- enum
- Option
- Returning Result from business logic
- Vec
- HashMap
- Generics
- Traits
- Lifetimes
- Modules
- External crates
- Unit tests
- Advanced pattern matching

Only features from Chapters 1–5 are allowed.

---

# 6. Architecture Rules

- Business logic must be implemented inside `BankAccount`
- CLI logic must remain in `main`
- No global mutable state
- Methods must operate only on `self`
- Ownership rules must be respected

---

# 7. Implementation Order

1. Define `struct BankAccount`
2. Implement `new`
3. Implement `deposit`
4. Implement `withdraw`
5. Implement `print_balance`
6. Test methods directly in `main`
7. Implement CLI loop
8. Implement numeric input function
9. Final polish

---

# 8. Expected Behavior

The program must:

- Allow depositing funds
- Allow withdrawing funds with validation
- Display current balance
- Exit cleanly on command

The application must compile without warnings (unless explicitly justified).

---

# 9. Completion Criteria

The project is complete when:

- All requirements are implemented
- Program compiles successfully
- CLI behaves according to specification
- Code uses only features from Rust Book Chapters 1–5
