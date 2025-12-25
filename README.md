# Rust Todo List (CLI)

A simple **command-line todo list** written in **Rust**.
Entries are stored locally in a JSON file and can be viewed, added, and removed.

This project is my **first Rust project** and is intended as a learning exercise for:

* Rust fundamentals
* Module structure
* File handling & JSON serialization
* Basic CLI interaction

---

## ✨ Features

* View all todo entries
* Add new entries
* Delete existing entries
* Automatic timestamp for each entry
* Persistent storage in `entries.json`
* Cross-platform (Windows / Linux / macOS)

---

## 📂 Project Structure

```txt
.
├── src
│   ├── main.rs        # CLI logic & menu
│   ├── json.rs        # Loading/saving the JSON file
│   └── list_entry.rs # Data structure for todo entries
├── data
│   └── entries.json  # Stored todos (created automatically)
└── Cargo.toml
```

---

## 🚀 Installation & Usage

### Requirements

* **Rust**: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Run the project

```bash
cargo run
```

On first launch, the file
`./data/entries.json` is created automatically.

---

## 🧑‍💻 Usage

After starting the program, a menu is displayed:

```text
1: View entries
2: Add entry
3: Delete entry
```

### ➕ Add an entry

* Enter a title
* Enter a description
* A timestamp is added automatically

### ❌ Delete an entry

* Enter the entry number (starting from 1)

### 🚪 Exit the program

```txt
q | quit | exit
```

---

## 🧱 Used Crates

* [`chrono`](https://crates.io/crates/chrono) – Date & timestamps
* [`serde_json`](https://crates.io/crates/serde_json) – JSON serialization & deserialization

---

## 🛠️ Possible Improvements

* [ ] Edit existing entries
* [ ] Priorities or tags
* [ ] Search / filtering
* [ ] Sorting by date
* [ ] Colors (e.g. with `crossterm`)
* [ ] Tests
* [ ] Import / export
