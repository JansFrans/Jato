````markdown
<div align="center">

<img src="stuff/hero.png" width="100%" alt="Jato Banner">

# Jato

### Cross-Platform Growtopia Automation Framework

A modern Growtopia companion built with **Rust**, designed to run on multiple operating systems while providing an intuitive web-based control panel.

[![License](https://img.shields.io/badge/License-CC--BY--NC--SA--4.0-blue?style=for-the-badge)](https://creativecommons.org/licenses/by-nc-sa/4.0/)
[![Rust](https://img.shields.io/badge/Built%20With-Rust-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-success?style=for-the-badge)]

### Repository

https://github.com/JansFrans/Jato

</div>

---

# 📖 Overview

Jato is an open-source Growtopia automation framework written in **Rust**.

Unlike traditional Growtopia bots that depend on native desktop interfaces, Jato exposes a lightweight web dashboard that allows you to manage bots directly from any modern browser.

The project focuses on:

- Cross-platform compatibility
- High performance
- Lightweight architecture
- Multi-bot management
- Extensible scripting system
- Modern web interface

---

# ✨ Features

## 🌐 Web Dashboard

- Browser-based interface
- Live bot monitoring
- World visualization
- Interactive world map
- Inventory viewer
- Item database
- Real-time console logs

---

## 🤖 Bot Functions

- Multi Bot Support
- Automatic Pathfinding
- Warp Between Worlds
- Punch Blocks
- Place Blocks
- Auto Collect
- Drop Items
- Trash Items
- Auto Reconnect

---

## 📜 Lua Scripting

Automate repetitive tasks using the embedded Lua engine.

Features include:

- Custom scripts
- Configurable delays
- Automation logic
- Event-based execution

---

## 🔐 Authentication

Current support includes:

- Legacy Login
- Automatic Session Refresh
- SOCKS5 Proxy

Planned:

- Google Authentication
- Apple Authentication

---

## 📦 Data Management

Included:

- Inventory Viewer
- Item Database
- World Scanner

Planned:

- Automatic Updates
- Online Item Database Sync

---

# 🚀 Installation

## Requirements

Install:

- Rust (Edition 2024)
- Bun

---

## Clone Repository

```bash
git clone https://github.com/JansFrans/Jato.git
cd Jato
````

---

## Build Web Assets

```bash
cd web
bun install
bun run build
cd ..
```

---

## Compile

```bash
cargo build --release
```

---

## Run

```bash
./target/release/Jato
```

Open your browser:

```
http://localhost:3000
```

---

# 📂 Project Structure

```
Jato
│
├── web/
│   ├── src/
│   ├── public/
│   └── dist/
│
├── src/
│
├── stuff/
│
├── Cargo.toml
│
└── README.md
```

---

# 🛣 Roadmap

* [x] Multi Bot
* [x] Web Dashboard
* [x] Inventory Viewer
* [x] World Scanner
* [x] Lua Support
* [x] Proxy Support
* [ ] Google Login
* [ ] Apple Login
* [ ] Auto Updater
* [ ] Plugin System
* [ ] Remote Dashboard
* [ ] REST API

---

# 🤝 Contributing

Contributions are welcome.

If you have ideas, improvements, or bug fixes, feel free to:

* Open an Issue
* Submit a Pull Request
* Suggest New Features

---

# 📜 License

This project is licensed under the

**Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International License (CC BY-NC-SA 4.0).**

You are free to study and modify this project for personal and educational purposes.

Commercial redistribution without permission is prohibited.

---

<div align="center">

Made with ❤️ using Rust

**Jato**

Created & Maintained by **Jans**

https://github.com/JansFrans/Jato

</div>
```
