# ⏱️ ot_tracker

**A lightweight CLI app to track user overtime hours.**  
Built with Rust + SQLite using async workflows.

> 🚧 **Work In Progress**: This project is under active development. Expect bugs and missing features as it evolves.

---

## ✨ Features

- Create and list users
- Start and stop tracking overtime sessions
- Stores data in a local SQLite database
- Interactive CLI with `inquire` prompts
- Modular code with separation of concerns (handlers, models, prompts, etc.)
- Async-powered using `tokio` and `sqlx`

---

## 🚀 Getting Started

### 📦 Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [SQLite3](https://www.sqlite.org/index.html)

### 🛠️ Setup

```bash
git clone https://github.com/svizcaino26/ot_tracker.git
cd ot_tracker
cargo build
cargo run
```
---

### 🧪 Running in Dev Mode

To watch for changes and auto-rebuild:

---

### 🧰 Tech Stack

- Rust 🦀
- tokio – async runtime
- sqlx – SQLite support with compile-time checking
- inquire – terminal UI prompts
- anyhow – ergonomic error handling

---

### 🛣️ Roadmap / Todo

- User management: edit & delete users
- Export overtime sessions (e.g., to CSV)
- Better input validation
- Test coverage
- CLI flags & commands (e.g., --export, --add-user)
- Add logging
- Persistent config settings

---

### 🤝 Contributing

Pull requests and feedback are welcome!
Feel free to fork this repo and submit a PR.

---

### 📝 License

MIT — feel free to use it, remix it, and build on it.
See LICENSE for more details.

---

### 🙋‍♂️ Author

Stephen Vizcaíno
- 📍 Panama City, Panama
- 📧 svizcaino26@gmail.com
- 🌐 github.com/svizcaino26

> “Small tools make a big difference.” — Me, probably
