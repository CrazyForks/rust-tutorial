# rust-tutorial

Rust simple tutorial.

一个简单的 Rust 教程。

本文默认读者具有一定编程基础。

## 初始准备

首先通过 [Rust 官网](https://www.rust-lang.org/zh-CN/learn/get-started) 获取 Rust 安装包。

并跟随官方的文档来进行安装环境。

安装完毕之后，使用 `cargo init` 可以初始化一个项目。

```shell
cargo init # 在当前目录下初始化
cargo init Project # 当前目录下新建一个 Project 目录
```

本文将带领读者使用 Rust 编写一个简单的 CLI。

## 初始化项目

使用 `cargo init cli` 初始化一个名为 cli 的项目。

目录结构如下：

```sh
# cli 目录结构
- .git
- src
  - main.rs
- .gitignore
- Cargo.toml
```

其中的 `src` 目录存放的是项目源代码。 `Cargo.toml` 文件则用于保存项目依赖。

使用编辑器打开 `src/main`。可以看见以下内容:

```rust
// main.rs

// 这是一个简单的 Rust 程序
// 仅输出 Hello world!
// rust 语句需要以分号(;)结尾
fn main() {
    println!("Hello, world!");
}
```

就像婴儿以啼哭宣告自己生命开始。

我们在学习一门新语言时，会使用输出 `hello world` 来代表自己学习的开始。

使用终端打开项目。使用 `cargo run` 运行项目。

可以看见 `Hello world!` 被输出。

## 安装依赖

我们需要编写的是一个 cli 程序。

cli 全称为 **Command Line Interface**, 中文名为**命令行接口**。

前文提到的 `cargo` 就是这样的一个程序。

在程序开发中，我们通常会将常用的一些功能给封装起来，方便后续重复使用。
更进一步的就是将封装好的功能包给发布到网络中，让其他人也可以使用。

在项目中使用到的包就是依赖。`cargo` 就是 rust 的包管理工具。我们可以通过它安装项目依赖包。

```toml
# Cargo.toml

# ...

[dependencies]
# 在 dependencies 模块下增加以下内容：
# 也可以不直接修改文件 而是使用命令行增加依赖
# 比如:
# 添加依赖 clap 并启用 derive 功能
# cargo add clap --features derive
clap = { version = "4.5.16", features = ["derive"] }          # 解析命令行参数
comfy-table = "7.1.1"                                         # 可以输出表格形式
console = "0.15.8"                                            # 美化终端输出
dialoguer = "0.11.0"                                          # 命令行交互，比如文本输入，用户确认
indicatif = "0.17.8"                                          # 进度条
rusqlite = { version = "0.32.1", features = ["bundled"] }     # 在 rust 使用 sqlite
serde = { version = "1.0.208", features = ["derive"] }        # 序列化和反序列化框架
serde_json = "1.0.125"                                        # 序列化和反序列化 JSON
```

然后使用 `cargo build` 进行构建，`cargo` 会自动将依赖拉取。_如果是使用 `cargo add` 命令增加的依赖，则可以跳过此步骤。_

### 使用依赖

```rust
// main.rs

// 导入包模块
// 可以合并
// use clap::{Parser, Subcommand};
use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]                     // 派生宏 会应用到结构体上
#[command(version, about, long_about=None)]  // 派生宏 会应用到结构体上
pub struct Program {                         // 声明结构体
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]          // 派生宏
pub enum Command {                           // 声明枚举
    TODO {
        #[arg(short, long, default_value = None)]
        find: String,
    },
}

// main 函数 是一个程序的开始
fn main() {
    let args = Program::parse();

    println!("args: -- {:?}", args);
}

```

之后使用 `cargo run -- todo --find a` 即可运行。
