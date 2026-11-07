# 第 6 题 · C 方向：Rust 环境

OpenLab 2026 纳新第 6 题的 Rust 环境实验。由本人在 WSL2 中操作，保留原始终端日志和截图；AI 提供步骤说明、排错建议及仓库整理。

## 已完成内容

- [x] 用 rustup 安装 Rust 工具链及 Clippy。
- [x] 执行 `cargo new rust_lab --vcs none`，用 `cargo run` 运行程序。
- [x] 编写两个单元测试并运行 `cargo test`：2 passed，0 failed。
- [x] 运行 `cargo clippy --all-targets -- -D warnings`。
- [x] 说明 rustc、cargo、rustup 的分工。
- [x] 通过 `cargo add rand@0.9` 引入第三方 crate，生成五次 1～6 的骰子点数。
- [x] 高阶加分：在 QEMU 中运行 rCore 第一章并将欢迎语改为 `Hello, OpenLab!`。
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

## rCore 第一章

选择 WSL2 路线，在 `/home/pace/rcore-ch1/os` 中执行 `make run`。本次使用 QEMU `10.2.1`、Rust `1.99.1`、目标 `riscv64gc-unknown-none-elf`，并安装 `rust-src`、`llvm-tools` 和 `cargo-binutils`。

上游仓库为 `https://github.com/rcore-os/rCore-Tutorial-v3.git`，分支 `ch1`，本次基准提交为 `85fe952a04c4b671fef1beb2f470591ae7ea5123`。先运行原版，再将 `os/src/main.rs` 中的欢迎语改为 `[kernel] Hello, OpenLab!`，重新 `make run`，截图中可见命令、本机路径和修改后的输出。

当前上游版本在打印内存区间后调用 `sbi::shutdown(false)` 正常关机；`[ERROR] [kernel] .bss ...` 是源码中的日志级别示例，不是运行失败，也没有人为添加 panic 来模仿题单示例。

`rcore/ch1-welcome.patch` 保存实际的一行源码修改，`rcore/Cargo.lock` 保存本次内核构建的依赖版本。在上述上游提交的仓库根目录应用补丁，并将该锁文件放入上游 `os/Cargo.lock`，即可在相同工具链配置下重建；第三方教程完整源码和构建产物不复制进本答题仓库。

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
| `evidence/06-rcore-setup.typescript` | rCore 环境安装、编译、原版运行、修改欢迎语和重新运行的原始记录。 |
| `evidence/07-rcore-openlab.png` | `make run` 命令、本人路径、QEMU 版本和 `Hello, OpenLab!` 运行结果。 |
| `rcore/ch1-welcome.patch` | 相对于上游基准提交的欢迎语源码补丁。 |
| `rcore/Cargo.lock` | 本次 rCore 内核构建的依赖锁文件。 |

终端日志可能包含控制字符、输入修正和历史错误，作为原始记录保留，不对其清洗或截断。`04-rand.typescript` 首次提交时尚未结束，本次已补全；rand 与 rCore 日志均包含 `Script done` 结束标记。

## AI 协作记录

- 提问：Rust 安装完成后，如何继续完成第 6 题 C 方向，并在关键步骤留痕？
- 建议：用 `script` 记录终端；创建 Cargo 项目；添加两个单元测试；运行程序、测试和 Clippy；保存带有本人路径的截图。
- 核验：`03-cargo.typescript` 和 `04-cargo-clippy.png` 记录了实际执行结果，两个测试通过，Clippy 退出码为 0，建议有效。
- 后续建议：通过 `cargo add rand@0.9` 引入依赖，使用 `rand::random_range(1..=6)` 实现掷骰子。
- 核验：`04-rand.typescript` 记录了依赖引入、骰子输出、两个测试通过及 Clippy 完成，用户确认完成。
- rCore 配置：AI 指导使用 WSL2、安装 QEMU，依据克隆仓库的 `rust-toolchain.toml` 和 Makefile 准备编译组件；本人执行 `make run`，完成原版与修改欢迎语后的运行。
- 输出排查：AI 最初按题单示例预期会有 panic 和关机文字；实际输出不同。核对当前 ch1 的 `main.rs`、日志代码及 `sbi.rs` 后，确认内存区间日志后直接正常关机，并纠正最初预期。`06-rcore-setup.typescript` 和 `07-rcore-openlab.png` 保留实际结果。
- 仓库整理：用户授权 AI 建立独立 Git 仓库并上传；Git 提交时间对应整理和提交时间，实验执行时间见原始日志。

## 踩坑记录（300 字以内，提交前本人复核）

我选择 WSL2 配置 Rust 环境。查询 GCC 版本时误写 `gcc -version`，根据报错改为 `gcc --version`。向 AI 询问后续步骤，按建议完成运行、两个单元测试、Clippy 和 rand 掷骰子，并用 script 留痕。运行 rCore ch1 后出现 ERROR 级别的 .bss 日志，却没有题单示例中的 panic；向 AI 询问并核对源码，确认该版本打印日志后直接正常关机，AI 最初预期需修正。随后修改欢迎语，重新 make run，截图显示 Hello, OpenLab!，保留了命令和本人路径。
