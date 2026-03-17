## Project Plan: `inventory_manager`

### Project Idea

I want to build a small Rust console application — an **RPG character inventory manager**.

The goal of this project is to practice everything I learned from **Rust Book chapters 1–7**:

* basic syntax
* variables and types
* functions
* `if`, `match`, loops
* `struct`
* `enum`
* methods
* `String`, `Vec`
* error handling with `Result`
* modules
* `pub`, `use`, `super`
* splitting code into files

---

# Main Idea of the Program

The program manages a **character inventory**.

It should allow me to:

* add items
* remove items
* show all items
* equip an item
* unequip an item
* show equipped items
* calculate total inventory weight

---

# Main Goal

The goal is to practice writing a **real program with modules**, using only the Rust features I already learned.

I will not use topics that I have not studied yet.

---

# Main Entities

## Item

I will create a structure `Item` that represents a single item.

Each item will have:

* `id`
* name
* item type
* weight
* state

---

## Item Type

I will create an `enum ItemType`.

Example types:

* Weapon
* Armor
* Potion
* Misc

---

## Item State

I will create an `enum ItemState`.

It will show if an item is equipped or not.

Example states:

* InInventory
* Equipped

---

## Inventory

I will create a structure `Inventory`.

It will store all items.

It will likely contain:

* `Vec<Item>`

---

# Program Features

## Item Management

The program should allow me to:

* add a new item
* remove an item by `id`
* find an item
* show all items

---

## Item State Management

The program should also allow me to:

* equip an item
* unequip an item
* show only equipped items

---

## Summary Information

The program should be able to:

* calculate total inventory weight
* count items
* show basic inventory information

---

# Project Structure

At first I may write everything in one file.

Later I will split the project into modules and files.

Target structure:

```
src/
├ main.rs
├ item.rs
├ inventory.rs
└ ui.rs
```

---

## `main.rs`

This file will contain:

* the program entry point
* creation of the inventory
* starting the menu

---

## `item.rs`

This module will contain:

* `struct Item`
* `enum ItemType`
* `enum ItemState`
* item-related methods

---

## `inventory.rs`

This module will contain inventory logic:

* storing items
* adding items
* removing items
* equipping items
* unequipping items
* calculating total weight
* searching items

---

## `ui.rs`

This module will contain the console interface:

* menu
* command handling
* reading user input
* calling inventory methods

---

# What I Want to Practice

## Structures and Enums

I want to practice modeling data with:

* `struct`
* `enum`

---

## Methods

I want to use methods with `impl` instead of writing everything as free functions.

---

## Match

I want to use `match` for:

* menu commands
* item types
* state handling
* error handling

---

## Result

Some operations should return `Result`, for example:

* item not found
* item already equipped
* item already unequipped

---

## Modules

One main goal of this project is to practice **Rust modules**.

I want to organize the code into clear modules.

---

# Development Steps

## Step 1 — Basic Version

Create a minimal working version:

* one inventory
* several items
* add items
* list items
* remove items

---

## Step 2 — Item States

Add:

* equipping items
* unequipping items
* filtering equipped items

---

## Step 3 — Improve Code Structure

Refactor the code:

* convert functions into methods
* review `pub` and private items
* hide internal logic inside modules

---

## Step 4 — Split Into Modules

Split the project into multiple files.

This step will help me practice **Rust module system**.

---

## Step 5 — Cleanup

Improve the project:

* better names
* simpler `main.rs`
* cleaner module structure
* more readable code

---

# Project Limitations

To keep the project simple, I will **not** include topics I have not learned yet:

* traits as a main topic
* generics
* lifetimes
* testing
* file storage
* external crates unless necessary
* async programming
* multithreading

The project should only use Rust features from **chapters 1–7**.

---

# Final Result

At the end I want to have:

* a working console application
* a project organized with modules
* code using only the concepts I learned
* a clear and readable structure

---

# Success Criteria

The project will be successful if:

* I understand the structure of the code
* I can explain what each module does
* I use `struct`, `enum`, methods and `Result` correctly
* modules make the code clearer and easier to understand
