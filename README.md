# Rust 基础知识学习 (Rust Fundamentals)

[![shipthatcode — Rust Fundamentals](https://api.shipthatcode.com/cert/cc5dbd1077c14a6728bcae86d8a73093.svg)](https://shipthatcode.com/courses/rust-fundamentals)

本项目是我学习 **Rust 基础语法与核心概念** 的个人代码练习与笔记仓库，配套 [shipthatcode.com](https://shipthatcode.com) 上的 [Rust Fundamentals](https://shipthatcode.com/courses/rust-fundamentals) 课程。

---

## 📚 学习内容与章节进度

- [ ] `01` - Hello World (基础输出)
- [ ] `02` - Variables & Mutability (变量与可变性)
- [ ] `03` - Numbers & Math (数值类型与数学运算)
- [ ] `04` - String & &str (字符串与字符串切片)
- [ ] `05` - Formatting (格式化输出)
- [ ] `06` - If & Match (条件判断与模式匹配)
- [ ] `07` - Loops (循环控制)
- [ ] `08` - Functions (函数与返回值)
- [ ] `09` - Ownership & Borrowing (所有权与借用机制)
- [ ] `10` - Vec (动态数组)
- [ ] `11` - HashMap (哈希表)
- [ ] `12` - Iterators (迭代器)
- [ ] `13` - Structs (结构体)
- [ ] `14` - Enums & Match (枚举与高级匹配)
- [ ] `15` - Result & Option (错误处理与空值安全)

---

## 🛠️ 环境要求

- **Rust 工具链**：已安装 `rustc` 和 `cargo`（建议通过 [rustup.rs](https://rustup.rs/) 安装）。
- **Git**：用于版本管理及运行测试。
- **代码编辑器**：VS Code（推荐安装 `rust-analyzer` 插件）、RustRover 或任何喜欢的编辑器。

### Windows 运行说明

本项目测试脚本为 `run_tests.sh`（Shell 脚本），在 Windows 环境下推荐以下方式运行：

1. **方式 1（推荐）：使用 Git Bash 终端**
   - 在项目目录空白处右键选择 **"Open Git Bash here"**，直接运行 `./run_tests.sh 01`。
2. **方式 2：在 CMD / PowerShell 中调用 Git 的 Bash**
   ```cmd
   bash run_tests.sh 01
   # 或
   sh run_tests.sh 01
   ```
3. **方式 3：使用 WSL (Windows Subsystem for Linux)**
   ```cmd
   wsl ./run_tests.sh 01
   ```

> **提示**：Windows 下请注意保持文件换行符为 `LF`，避免因 `CRLF` 导致测试输出比对失败。

---

## 🚀 学习与测试流程

1. **编写代码**：
   在 `main.rs` 中编写对应章节的练习代码。

2. **本地运行测试**：
   针对当前章节进行针对性测试（如第 1 课）：
   ```bash
   bash run_tests.sh 01
   ```
   *(如果直接运行 `./run_tests.sh` 则会一次性运行所有章节的测试)*

3. **保存与归档（可选）**：
   每道题都是独立的练习，进入下一课时可以直接覆盖 `main.rs` 内容。
   - 历史练习代码都已保存在 Git 提交记录中。
   - 如果想在本地保留每个课时的代码副本，可以在根目录下新建 `solutions/` 文件夹（例如复制为 `solutions/01-hello-world.rs`）。

4. **提交与推送**：
   ```bash
   git add -A
   git commit -m "feat: 完成第 01 课练习"
   git push
   ```

5. **在线评测**：
   在课程页面点击 **Check my solution** 进行全量在线判题。

---

## 📁 目录结构

```text
rust/
├── main.rs              # 当前练习的代码文件
├── run_tests.sh         # 本地测试运行脚本
├── tests/               # 各课程章节的测试用例输入/输出 (01-15)
│   ├── 01-hello-world/
│   ├── 02-variables-and-mutability/
│   └── ...
├── .shipthatcode.json   # 课程在线评测配置文件（请勿删除）
└── README.md            # 项目说明文档
```

