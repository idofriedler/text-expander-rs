# 🧠 Text Expander

A lightweight desktop tool that lets you type shortcuts and automatically expands them into longer phrases — ideal for boosting productivity and saving time on repetitive typing.

> ⚠️ **Disclaimer**: This is a personal project to help me learn the Rust programming language. I provide this software as-is, with no guarantees or warranties. **Use at your own risk**. I am not responsible for any damage or data loss caused by this tool.

---

## ✨ Features

- 🔡 Define custom text shortcuts (e.g., `omw` → `On my way!`)
- ��️ Simple graphical interface to add/remove shortcuts
- 🧠 Runs quietly in the background
- 🟢 Toggle on/off from the tray window
- 💾 All shortcuts are saved locally
- ✅ Cross-platform: **Linux** and **Windows** supported

---

## 📥 Installation

### 🐧 Linux

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-username/text_expander.git
   cd text_expander
   ```
2. **Make the installer executable (if needed)**
   ```bash
   chmod +x install.sh
   ```

3. **Run the install script**
```bash
./install.sh
```
4. **Open your app menu and launch Text Expander**

## 🪟 Windows

1. **Download** `text_expander_windows.zip` from the [Releases](https://github.com/idofriedler/text-expander-rs/tree/main/releases)

2. **Extract** the archive anywhere (e.g., Desktop or Program Files)

3. **Double-click** `install.bat`

4. A desktop shortcut will be created. Run **Text Expander** from there.

---

## 📁 How It Works

- Define shortcuts using the GUI (`Shortcut → Expansion`)

- Type your shortcut and press **Tab**

- The shortcut is deleted and replaced with your defined expansion

- All shortcuts are stored in a file (`shortcuts.txt`) under your local data directory

---

## 🔧 Built With

- [Rust](https://www.rust-lang.org/)

- `eframe` + `egui` for GUI

- `rdev` for keyboard input simulation

- `simplelog` for logging

---

## 📄 License

No license provided yet. This is an experimental project. You're welcome to try it out, but again — **no guarantees!**Enjoy using Text Expander! 😊




