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

更进一步的就是将这些功能封装为一个包给发布到网络中，让其他人也可以使用。

如果项目中有用到某个包，那么说明项目依赖于这个包。这个包就是项目的依赖。

`cargo` 就是 rust 的包管理工具。我们可以通过它安装项目依赖包。

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

然后使用 `cargo build` 进行构建，`cargo` 会自动将依赖拉取。

_如果是使用 `cargo add` 命令增加的依赖，则可以跳过此步骤。_

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

```sh
# 输出内容
args: -- Program { command: TODO { find: "a" } }
```

`cargo run` 是运行程序。

后面跟随的 `--` 是为了分隔，否则就会造成冲突。比如:

- `cargo run --help` 是查看 `cargo run` 这条命令的帮助内容。

- `cargo run -- --help` 则是查看我们编写的程序的帮助内容。

后面的 `todo` 和 `--find a` 则是在 `Command` 枚举中定义的成员。

### 可选参数

如果有试着不加 `--find` 参数，那么就会发现程序报错了。

因为 `--find` 参数是必须的。

如果需要使用可选参数，则需要这样做：

```rust
// ...

// 声明枚举
#[derive(Debug, Clone, Subcommand)] // 派生宏
pub enum Command {
    TODO {
        #[arg(short, long, default_value = None)]
        // Option<T> 是一个枚举
        // 具有两种状态 Some(T) 和 None
        // 分别代表有值和无值
        // 通过它就可以让值可选
        find: Option<String>,
    },
}

// ...
```

执行 `cargo run -- todo`, 现在不会报错了。

```sh
cargo run -- todo
# args: -- Program { command: TODO { find: None } }

cargo run -- todo --find a
# args: -- Program { command: TODO { find: Some("a") } }
```

## 模式匹配

在 Rust 中，模式匹配 是一种功能非常强大的控制流结构，可以用于解构结构体、枚举、元组，甚至匹配某些值的条件。

可以将它类比于其他语言中的 switch。

但 Rust 的 match 不仅支持值的匹配，还能解构变量、绑定新变量、甚至结合 if 条件使用，功能远强于传统 switch。

将 `main` 函数改造一下。

```rust
fn main() {
    // 使用 clap 自动解析命令行参数，返回 Program 实例
    let args = Program::parse();

    // 模式匹配 command 字段
    match args.command {
        // 如果是 Command::TODO 就将字段 find 解构出来
        Command::TODO { find } => match find {
            // 因为是 Option<String> 类型
            // 需要额外的模式匹配
            Some(val) => println!("find {}", val),
            None => println!("No --find argument provided"),
        },
    }
}
```

现在分别运行 `cargo run -- todo` 和 `cargo run -- todo --find a` 可以查看效果。

## 枚举

枚举，在各种编程语言中或多或少都有着它的身影。

它的作用是用于表示一组有限的、互斥的可能取值，例如周一到周日，性别等。

与其他语言的枚举相比，Rust 的枚举更加灵活和强大：

- 支持每个变体携带不同的数据
- 可与模式匹配强结合，做复杂的控制流
- 可以和 trait、方法一起使用，实现丰富的抽象设计

在前面示例中就定义了一个带字段的枚举 `Command`, 用于表示不同的子命令。

```rust
#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    TODO {
        #[arg(short, long, default_value = None)]
        find: String,
    },
}
```

### 增加 TODO 项

目前我们的 CLI 只能查找 TODO 项，但无法添加新项。我们接下来为 `Command` 枚举新增一个 add 子命令：

```rust
#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    TODO {
        #[arg(short, long, default_value = None)]
        find: Option<String>,
    },
    ADD {
        #[arg(short, long, default_value = None)]
        title: Option<String>,
        #[arg(short, long, default_value = None)]
        content: Option<String>,
    },
}
```

这样我们可以通过如下命令添加一条新 TODO：

```bash
cargo run -- add --title "new todo" --content "new todo content"
```

### 编译错误

但如果我们直接运行程序，会出现报错:

```text
non-exhaustive patterns: `Command::ADD { .. }` not covered the matched value is of type `Command`
```

这是 Rust 的穷尽性检查在发挥作用：在 match 表达式中没有匹配所有可能的枚举变体。

解决方法很简单：在 match 中补充对 `Command::ADD` 的处理即可。

```rust
    // 模式匹配 command 字段
    match args.command {
        // 如果是 Command::TODO 就将字段 find 解构出来
        Command::TODO { find } => match find {
            // 因为是 Option<String> 类型
            // 需要额外的模式匹配
            Some(val) => println!("find {}", val),
            None => println!("No --find argument provided"),
        },
        Command::ADD { title, content } => {
            match title {
                Some(title) => {
                  // 这里声明了一个变量 todo, 类型为 String, 尚未赋值
                  let todo: String;

                  // 这里使用 if let 表达式
                  // 它的作用是将 content 绑定到一个新的变量 content
                  // 并在 content 不是 None 时执行代码块
                  // 这样就不需要再额外的模式匹配了
                  if let Some(content) = content {
                    todo = content;
                  } else{
                    // 如果 content 是 None, 则使用默认值
                    todo = String::from("default todo content");
                  }

                  println!("title: {}, content: {}", title, todo);
                }
                _ => println!("No title or content provided"),
            }
        }
    }
```

## 结构体

将 TODO 项定义为结构体，引入结构体，访问修饰符，特征等知识。

### 定义 TODO 结构体

### 访问修饰符

### 实现特征

## 模块化

引入 Rust 模块化开发知识。

### 拆分文件

### 孤儿原则

### 项目组织

## 数据持久化

引入数据持久化知识。

罗列方案，文件存储，数据库存储，并选择 sqlite。

### 文件存储

### 使用 sqlite 数据库

## 日志

引入日志作用知识，并实现一个派生宏。引入 `cargo init --lib`。

### 自动记录操作派生宏

## 发布

引入发布知识。
