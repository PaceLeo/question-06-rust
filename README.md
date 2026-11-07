# 第 6 题 · C 方向：Rust 环境

OpenLab 2026 纳新第 6 题的 Rust 环境实验。由本人在 WSL2 中操作，保留原始终端日志和截图；AI 提供步骤说明、排错建议及仓库整理。

## 已完成内容

- [x] 用 rustup 安装 Rust 工具链及 Clippy。
- [x] 执行 `cargo new rust_lab --vcs none`，用 `cargo run` 运行程序。
- [x] 编写两个单元测试并运行 `cargo test`：2 passed，0 failed。
- [x] 运行 `cargo clippy --all-targets -- -D warnings`。
- [x] 说明 rustc、cargo、rustup 的分工。
- [x] 通过 `cargo add rand@0.9` 引入第三方 crate，生成五次 1～6 的骰子点数。
- [ ] 高阶加分：在 QEMU 中运行 rCore 第一章并修改欢迎语。
- [ ] 额外加分：运行 rCore 第二章。

## 环境与运行

安装阶段原始记录显示：WSL2 内核 `6.6.87.2-microsoft-standard-WSL2`，Rust 和 Cargo 为 `1.99.1`，rustup 为 `1.29.3`。实际版本以 `evidence/` 中的截图和日志为准。安装阶段路径包含 `Openlab纳新`，后续操作路径为 `Openlab`，原始日志保持原样。

已有 Rust 工具链时，在仓库根目录执行：

```bash
cd rust_lab
cargo run --locked
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
```

程序先输出 `2 + 3 = 5`，再输出五次掷骰子结果。点数范围为 1～6，允许重复。测试覆盖普通加法与加零的情形，不用于证明随机分布。`Cargo.toml` 声明 `rand = "0.9"`，`Cargo.lock` 保存解析出的具体依赖版本。

## 三个工具的分工

| 工具 | 分工 |
| --- | --- |
| rustup | 安装、更新和切换 Rust 工具链，管理组件与编译目标。 |
| cargo | 管理项目和依赖，组织构建、运行、测试，并在构建时调用 rustc。 |
| rustc | Rust 编译器，将 Rust 源代码编译为目标程序或库。 |

## 留痕索引

| 文件 | 内容 |
| --- | --- |
| `evidence/01-setup.typescript` | 基础工具安装与实际排错原始记录。 |
| `evidence/01-build-tools.png` | 基础构建工具与环境信息。 |
| `evidence/02-rust-install.typescript` | Rust 安装、组件及版本信息原始记录。 |
| `evidence/02-rust-versions.png` | Rust 工具版本截图。 |
| `evidence/03-cargo.typescript` | 创建项目、运行、测试及 Clippy 原始记录。 |
| `evidence/03-cargo-run.png` | 创建项目与 Hello World 运行截图。 |
| `evidence/04-cargo-clippy.png` | 运行结果、两个测试通过及 Clippy 退出码 0。 |
| `evidence/04-rand.typescript` | 引入 rand、运行程序、测试与 Clippy 原始记录。 |
| `evidence/05-rand-run.png` | rand 程序相关操作和结果截图。 |

终端日志可能包含控制字符、输入修正和历史错误，作为原始记录保留，不对其清洗或截断。`04-rand.typescript` 在首次整理时尚无 `Script done` 结束标记，提交保留当时已有的原始内容；如记录仍在进行，应退出记录子 shell 后再同步更新。

## AI 协作记录

- 提问：Rust 安装完成后，如何继续完成第 6 题 C 方向，并在关键步骤留痕？
- 建议：用 `script` 记录终端；创建 Cargo 项目；添加两个单元测试；运行程序、测试和 Clippy；保存带有本人路径的截图。
- 核验：`03-cargo.typescript` 和 `04-cargo-clippy.png` 记录了实际执行结果，两个测试通过，Clippy 退出码为 0，建议有效。
- 后续建议：通过 `cargo add rand@0.9` 引入依赖，使用 `rand::random_range(1..=6)` 实现掷骰子。
- 核验：`04-rand.typescript` 记录了依赖引入、骰子输出、两个测试通过及 Clippy 完成，用户确认完成。
- 仓库整理：用户授权 AI 建立独立 Git 仓库并上传；Git 提交时间对应整理和提交时间，实验执行时间见原始日志。

## 踩坑记录（300 字以内，提交前本人复核）

我选择 WSL2 配置 Rust 环境。查询 GCC 版本时误写成 `gcc -version`，终端提示参数不识别；检查命令后改用 `gcc --version`，成功得到版本信息。安装 Rust 后，我向 AI 询问后续步骤及留痕方式，按建议用 `script` 记录操作，完成 Cargo 项目创建、运行、两个单元测试和 Clippy 检查，并引入 rand 实现掷骰子。建议经实际操作有效，相关截图保留了用户名和项目路径。
