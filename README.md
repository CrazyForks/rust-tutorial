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

目前我们已经实现了通过命令行添加和查找 TODO 项的功能，但这些内容仅仅是打印输出，并没有真正保存下来。

但是目前的 TODO 项分为 `title` 和 `content` 两个部分。

我们可以使用结构体来存储 TODO 项信息。

### 定义 TODO 结构体

```rust
/// TodoItem 结构体
pub struct TodoItem {
  /// 标题
  pub title: String,
  /// 内容
  pub content: String,
  /// 状态
  pub done: bool,
}
```

实现结构体的方式如下:

```rust
let learn = TodoItem {
    title: String::from("learn"),
    content: String::from("learn rust"),
    done: false,
}

let work = TodoItem {
    title: String::from("work"),
    content: String::from("requirement 1 must be completed before next week"),
    done: true,
}
```

但是这样写着比较繁琐，因此我们可以实现一个函数来简化创建 todo 项。

```rust
fn create_todo(title: String, content: String) -> TodoItem{
  TodoItem {
    title,
    content,
    done: false
  }
}

let learn = create_todo("learn".to_string(), "learn rust".to_string())
let work = create_todo("work".to_string(), "requirement 1 completed".to_string())
```

现在我们改造 `main` 函数。

```rust
fn main() {
    let mut todo_list: Vec<TodoItem> = vec![
      create_todo("learn".to_string(), "learn rust".to_string()),
      create_todo("work".to_string(), "requirement 1 completed".to_string()),
      create_todo("play".to_string(),"play games".to_string()),
      create_todo("read".to_string(),"read books".to_string()),
    ];

    // 使用 clap 自动解析命令行参数，返回 Program 实例
    let args = Program::parse();

    // 模式匹配 command 字段
    match args.command {
        // 如果是 Command::TODO 就将字段 find 解构出来
        Command::TODO { find } => match find {
            // 因为是 Option<String> 类型
            // 需要额外的模式匹配
            Some(val) => {
              println!("find {}", val);
              for item in todo_list {
                if item.title.contains(&val) {
                  let status: String;
                  if item.done{
                    status = String::from("X");
                  } else {
                    status = String::from(" ");
                  }
                  println!("[{:}] {:?} {:?}", status, item.title, item.content);
                }
              }
            },
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

                  todo_list.push(create_todo(title, todo));

                }
                _ => println!("No title or content provided"),
            }
        }
    }
}
```

现在运行 `cargo run -- todo --find` 命令就可以查到内容了。

### 访问修饰符

默认情况下，Rust 中的所有项都是私有的, 只能在定义它们的模块内访问。

通过访问修饰符 `pub` ，可以将结构体、枚举、函数变为外部可见的。

在之前的代码中，`pub` 使用的地方相当多。

```rust
// 外部可见 TodoItem 结构体
pub struct TodoItem {
  // 外部可见 TodoItem 结构体的 id 属性
  pub id: i32;
  // ...
}

// 外部可见 Command 枚举
pub enum Command {
  // 与结构体不同，只要将枚举定义为 pub 的
  // 那么它的枚举值将自动外部可见
  // 这是因为如果不外部可见枚举值，那么这个枚举相当于没用了
  TODO,
  // ...
}
```

## 模块化

迄今为止，我们已经实现了 todo 项的查找和增加了。也定义了相关的代码结构。

但是这些代码混在在一起显得程序看起来有些紊乱。

所以我们需要将这些代码进行拆分，进行模块化。

### 拆分文件

目前的项目结构如下:

```bash
|- src
  |- main.rs
```

我们将代码进行拆分，建立以下文件:

```bash
|- src            # 项目的主源代码目录
  |- main.rs      # 应用程序入口，负责解析命令行参数并调用具体逻辑
  |- todo         # todo 模块，封装所有与待办事项相关的逻辑
    |- add.rs     # 实现 "add" 命令的处理逻辑（添加 TODO 项）
    |- core.rs    # 定义核心数据结构（如 TodoItem）及通用操作
    |- find.rs    # 实现 "find" 命令的处理逻辑（查找 TODO 项）
  |- todo.rs      # todo 模块入口，声明并组织 todo 子模块
```

`src/todo.rs` 文件内容如下：

```rust
pub mod add;
pub mod find;
pub mod core;
```

`src/todo/add.rs` 文件内容如下：

```rust
use super::core::{TodoItem,create_todo};

pub fn add_todo(title: Option<String>, content: Option<String>, todo_list: &mut Vec<TodoItem>) {
  match title {
    Some(title) => {
      let todo: String;

      if let Some(content) = content {
        todo = content;
      } else{
        todo = String::from("default todo content");
      }

      println!("title: {}, content: {}", title, todo);
      todo_list.push(create_todo(title, todo));

    }
    _ => println!("No title or content provided"),
  }
}
```

`src/todo/core.rs` 文件内容如下：

```rust
use clap::Subcommand;

pub struct TodoItem {
  pub title: String,
  pub content: String,
  pub done: bool,
}

pub fn create_todo(title: String, content: String) -> TodoItem{
  TodoItem {
    title,
    content,
    done: false
  }
}

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

`src/todo/find.rs` 文件内容如下：

```rust
use super::core::TodoItem;

pub fn find_todo(find: Option<String>, todo_list: &Vec<TodoItem>){
  match find {
    Some(val) => {
      println!("find {}", val);
      for item in todo_list {
        if item.title.contains(&val) {
          let status: String;
          if item.done{
            status = String::from("X");
          } else {
            status = String::from(" ");
          }
          println!("[{:}] {:?} {:?}", status, item.title, item.content);
        }
      }
    },
    None => println!("No --find argument provided"),
  }
}
```

`src/main.rs` 文件内容如下：

```rust
use clap::Parser;
use todo::core::Command;
use crate::todo::{add::add_todo, core::{create_todo, TodoItem}, find::find_todo};

mod todo;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Program {
    #[command(subcommand)]
    pub command: Command,
}

fn main() {
  let mut todo_list: Vec<TodoItem> = vec![
    create_todo("learn".to_string(), "learn rust".to_string()),
    create_todo("work".to_string(), "requirement 1 must be completed before next week".to_string()),
    create_todo("play".to_string(),"play games".to_string()),
    create_todo("read".to_string(),"read books".to_string()),
  ];

  let args = Program::parse();

  match args.command {
    Command::TODO { find } => find_todo(find, &todo_list),
    Command::ADD { title, content } => add_todo(title, content, &mut todo_list)
  }
}

```

在拆分的文件中可以看见 `use super::core::{TodoItem,create_todo};` 这样的内容。

Rust 为模块引用增加了一些模块路径前缀，或者说别名：

- `crate`: 根模块，代表当前项目
- `self`: 当前模块自身
- `super`: 当前模块的父模块

`use super::core::{TodoItem,create_todo};` 表示从当前模块的“父模块”中访问名为 core 的兄弟模块，并使用其定义的 TodoItem 类型。

> mod 声明是必须的，用于声明模块存在
>
> use 则是使用该模块，否则不会引用模块

而 `mod xx` 则是表示声明一个模块。`pub mod xx` 则是将这个模块给暴露为外部可访问。

### 为结构体实现方法

在先前的代码中，我们使用到了 `use super::core::{TodoItem,create_todo};` 用来分别引入 `TodoItem` 和创建它的函数。

但实际上，我们完全可以将 `create_todo` 这个独立函数作为 `TodoItem` 的一个关联函数来实现。
这样在调用时语义上更加自然，也符合 Rust 的结构体设计习惯。

```rust
impl TodoItem {
  pub fn new(title: String, content: String) -> Self {
    create_todo(title, content)
  }
}
```

以上代码为结构体 `TodoItem` 实现了 `new` 构造函数。

> _Rust 中使用 new 作为构造器名称是一个约定俗成的规则，且 Rust 没有 new 关键字。_

再次修改代码，将 `create_todo` 更改为 `TodoItem::new`。

### 项目组织

随着项目逐步扩展，我们可能会加入更多的命令、数据结构与功能模块。

为了保持良好的可维护性和清晰的目录结构，有必要对项目进行合理的组织与命名。

常见策略如下：

每个功能一个目录模块, 将功能模块放入单独文件夹，并划分子模块。

```bash
src
|- main.rs
|- todo
  |- mod.rs      # todo 模块入口
  |- core.rs     # 数据结构与核心逻辑
  |- add.rs      # "add" 子命令
  |- find.rs     # "find" 子命令
```

在 `mod.rs` 中统一组织模块。

`mod.rs` 或与其模块同名的文件，如 `todo.rs` 被用于充当模块入口，负责：

- 声明子模块，如 `pub mod core;`。
- 统一导出，如 `pub use self:;core::TodoItem;`。
- 维护清晰入口。

在划分模块时需要注意，模块最好不要同时依赖子模块和父模块，避免循环依赖。
且尽量将共享数据或类型上升至公共层。
使用访问修饰符来限制内部访问，以免内部代码对外暴露过多。

## 数据持久化

引入数据持久化知识。

罗列方案，文件存储，数据库存储，并选择 sqlite。

### 文件存储

### 实现序列化特征

### 使用 sqlite 数据库

## 日志

引入日志作用知识，并实现一个派生宏。引入 `cargo init --lib`。

### 自动记录操作派生宏

## 发布

引入发布知识。
