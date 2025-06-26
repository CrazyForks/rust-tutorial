# Rust Tutorial

Rust 是一门近年来快速发展的系统级编程语言。

它兼具高性能与内存安全，广泛应用于嵌入式系统、操作系统、WebAssembly、后端服务以及命令行工具的开发。

相较于 C 语言 “相信你知道自己在做什么”，因此几乎不加限制地允许你操作内存和指针。
Rust 则恰恰相反，它从语言设计层面 “不信任开发者”，认为 “你总有一天会犯错”。

因此，Rust 引入了所有权系统、借用检查和生命周期机制。以求在编译阶段就将那些 “未来可能出问题的代码” 拒之门外。
这也意味着，开发者在编写代码时，往往需要花时间理解这些机制，努力 “说服” 编译器接受自己的写法。
尽管这过程曲折，但最终收获的是更加健壮和安全的程序。

本文将通过带领读者实现一个简单的 CLI 程序来学习 Rust。

## 初始准备

首先通过 [Rust 官网](https://www.rust-lang.org/zh-CN/learn/get-started) 获取 Rust 安装包。

并跟随官方的文档来进行安装环境。

安装完毕之后，使用 `cargo init` 可以初始化一个项目。

```shell
cargo init # 在当前目录下初始化
cargo init Project # 当前目录下新建一个 Project 目录
```

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

使用编辑器打开 `src/main.rs`。可以看见以下内容:

```rust
fn main() {
    println!("Hello, world!");
}
```

使用终端打开项目。使用 `cargo run` 运行项目。

可以看见 `Hello world!` 被输出。
