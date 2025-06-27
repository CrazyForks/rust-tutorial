# Rust Tutorial

Rust 是一门近年来快速发展的系统级编程语言。

它兼具高性能与内存安全，广泛应用于嵌入式系统、操作系统、WebAssembly、后端服务以及命令行工具的开发。

相较于 C 语言 “相信你知道自己在做什么”，因此几乎不加限制地允许你操作内存和指针。
Rust 则恰恰相反，它从语言设计层面 “不信任开发者”，认为 “你总有一天会犯错”。

因此，Rust 引入了所有权系统、借用检查和生命周期机制。以求在编译阶段就将那些 “未来可能出问题的代码” 拒之门外。
这也意味着，开发者在编写代码时，往往需要花时间理解这些机制，努力 “说服” 编译器接受自己的写法。
尽管这过程曲折，但最终收获的是更加健壮和安全的程序。

本文将通过带领读者实现一个简单用于记录 Todo 事项的 CLI (Command Line Interface, 命令行接口) 程序来学习 Rust。

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

## 变量

在 Rust 中，变量声明使用的是 `let` 关键字。

Rust 是类型后置的。即先声明变量，再定义类型。

```rust
fn main() {
  let msg: &str = "Hello, world!";
  println!("{}", msg);
}
```

类型后置一般都有一个特点，就是类型推断。
我们不需要每个变量都专门标注类型，编译器会自动推断出变量的类型。
只有当编译器无法推断出变量类型时，才需要手动标注类型。

因此以上代码可以更为:

```rust
fn main() {
  let msg = "Hello, world!";
  println!("{}", msg);
}
```

我们的 CLI 用于记录 Todo 事项，因此我们需要可以输入内容。

更改 `main.rs` 为如下内容：

```rust
fn main() {
  let args: Vec<String> = std::env::args().collect();

  println!("{:#?}", args);
}
```

命令行运行 `cargo run -- a b`，结果如下：

```bash
[
    "target\\debug\\cli.exe",
    "a",
    "b",
]
```

可以看见，我们获得的输入是一个数组格式，它的第一项是我们的可执行文件路径。

我们需要的是输入的内容，即 `a,b`。

调整代码：

```rust
fn main() {
  let args: Vec<String> = std::env::args().collect();

  let title = args[1];
  let content = args[2];

  println!("todo title: {}, content: {}", title, content);
}
```

运行 `cargo run -- a b`。会发现有报错：

```bash
error[E0507]: cannot move out of index of `Vec<String>`
 --> src\main.rs:6:15
  |
6 |   let title = args[1];
  |               ^^^^^^^ move occurs because value has type `String`, which does not implement the `Copy` trait
  |
help: consider borrowing here
  |
6 |   let title = &args[1];
  |               +
help: consider cloning the value if the performance cost is acceptable
  |
6 |   let title = args[1].clone();
  |                      ++++++++

error[E0507]: cannot move out of index of `Vec<String>`
 --> src\main.rs:7:17
  |
7 |   let content = args[2];
  |                 ^^^^^^^ move occurs because value has type `String`, which does not implement the `Copy` trait
  |
help: consider borrowing here
  |
7 |   let content = &args[2];
  |                 +
help: consider cloning the value if the performance cost is acceptable
  |
7 |   let content = args[2].clone();
  |                        ++++++++
```

### 所有权

以上报错的关键如下：

```bash
cannot move out of index of `Vec<String>`

move occurs because value has type `String`, which does not implement the `Copy` trait
```

它的意思是: 无法从 `Vec<String>` 中取出值，因为 `String` 类型没有实现 `Copy` 特征，无法被隐式复制。

在前面有提及到：

> Rust 它从语言设计层面 “不信任开发者”，认为 “你总有一天会犯错”。
>
> 因此，Rust 引入了所有权系统、借用检查和生命周期机制。以求在编译阶段就将那些 “未来可能出问题的代码” 拒之门外。

这里就是因为 Rust 引入的所有权系统导致的问题。

根据 Rust 所有权规则：

- 每个值都有一个所有者。
- 每个值同时只能有一个所有者。
- 当所有者离开作用域时，这个值将被丢弃。

以上报错就很好理解了。

我们试图从 `Vec<String>` 这个类型中取出值，但是根据所有权原则，每个值都只能有一个所有者。
因此 `Vec<String>` 拥有它内部所有 `String` 元素的所有权。

当我们使用 `args[1]` 这样的方式访问时，实际上是尝试将该元素的所有权 “移动” 到另一个变量。这就违反了所有权规则，
因为 `args` 还可能在后续被使用，如果移动了元素所有权，那么会导致它内部状态不一致，甚至出现悬垂指针、重复释放等问题。

### 引用和借用

Rust 在其语言设计层面上“不信任开发者”，因此它采用了所有权系统来强制保障内存安全。
编译器非常“智能”，它不仅会告诉你哪里出错了，还会提供修复建议。

比如，下面的编译错误信息中就给出了两种可能的解决方式：

```bash
help: consider borrowing here
  |
6 |   let title = &args[1];
  |               +
help: consider cloning the value if the performance cost is acceptable
  |
6 |   let title = args[1].clone();
  |                      ++++++++
```

第一种方法是 `let title = &args[1];`, 它表示借用 `args[1]` 的值，而不移动它的所有权。
这种方式高效，不会复制数据，但是变量的类型将变为 `&String`，表示这个变量是一个 `String` 值的引用。
因此它将受到引用对象的限制。当 `args` 失效，那么它的引用也将失效。

而第二种方法是 `let title = args[1].clone();`，它表示克隆 `args[1]` 的值，
并将这个值移动到 `title` 变量中，这样 `args` 失效时，也不会影响 `title` 的使用。

因此，我们选择使用第二种方式，显式调用 `clone` 方法，克隆一份 `args[1]` 的值。

> 创建一个引用的行为叫做借用。引用则是借用这个行为的结果。

我们再次运行 `cargo run -- a b`，可以发现编译通过了。

### 可变变量

在当前实现中，每次运行程序都需要输入两个参数（标题和内容），否则程序会因索引越界而报错。
为了提升程序的健壮性，我们可以为缺失的参数设置默认值。

修改代码:

```rust
fn main() {
  let args: Vec<String> = std::env::args().collect();
  let len = args.len();
  let title = args[1].clone();
  let content = String::from("default content");

  if len > 2{
    content = args[2].clone();
  }

  println!("todo title: {}, content: {}", title, content);
}
```

以上代码中，我们对输入参数做了检查，一旦参数数量大于 2 个，就会使用第三个参数作为内容。否则就会使用默认值。

执行 `cargo run -- a`，发现又有报错了。

```bash
error[E0384]: cannot assign twice to immutable variable `content`
  --> src\main.rs:10:5
   |
7  |   let content = String::from("default content");
   |       ------- first assignment to `content`
...
10 |     content = args[2].clone();
   |     ^^^^^^^ cannot assign twice to immutable variable
   |
help: consider making this binding mutable
   |
7  |   let mut content = String::from("default content");
   |       +++
```

这是因为 Rust 出于安全性和可读性考虑，默认所有变量都是不可变的。
这段报错的意思是：不能对不可变变量 `content` 进行二次赋值，除非将它声明为可变的。

编译器已经为我们提示了。在 `let` 后面增加 `mut` 关键字即可。

```rust
fn main() {
  let args: Vec<String> = std::env::args().collect();
  let len = args.len();
  let title = args[1].clone();
  let mut content = String::from("default content");

  if len > 2{
    content = args[2].clone();
  }

  println!("todo title: {}, content: {}", title, content);
}
```

再次执行 `cargo run -- a`，成功运行。

### 变量类型

Rust 是一门强类型的语言，这意味着变量在编译时必须要有明确的类型。

类型确定方式有两种，分别是显式声明和隐式推断。

显式声明, 在变量名称后面使用 `:` 指定类型。
例如：`let args: Vec<String> = std::env::args().collect();`。将变量 `args` 的类型指定为 `Vec<String>`。

隐式推断, 编译器根据变量的值和上下文推断变量的类型。
而 Rust 有着强大的类型推断机制，使得我们在大多数情况下，不需要手动标注类型。
编译器会自动推断类型，只有当编译器无法推断类型时才需要手动标注。

例如以下代码中，我们并未显式声明 `len`, `title` 或 `content` 的类型，但它们的类型仍然是确定的：

```rust
  let args: Vec<String> = std::env::args().collect();
  let len /** usize */ = args.len();
  let title /** String */ = args[1].clone();
  let mut content /** String */ = String::from("default content");
```

Rust 支持常见的基本类型:

- 整型: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- 无符号整型: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- 浮点数: `f32`, `f64`
- 布尔值: `bool`
- 字符: `char`

需要注意的是，Rust 中，`"xxx"` 是一个字符串字面量切片，类型为 `&str`，是在编译时就固定不可变的。
而 `String` 是一个字符串类型，编译时动态分配，可变长度。

我们在前面使用的 `args` 是 `Vec<String>`, 就是一个动态字符串的集合。

## 控制流

所谓控制流, 就是控制程序的流程。

在没有控制流的情况下，程序会按顺序从上往下逐行执行。
而控制流语句可以让我们根据条件选择性地执行某段代码，或者重复执行某段代码，
从而让程序拥有判断和循环的能力。

### if/else 分支

`if`/`else` 是 Rust 中最常用的控制流语句。

它用于判断某个条件是否成立。
它的判断条件必须返回布尔值，而不是其他类型。

如果判断条件成立，则执行 `if` 后面的代码块。
如果判断条件不成立，则执行 `else` 后面的代码块。

```rust
  let mut content = String::from("default content");

  if len > 2{
    content = args[2].clone();
  }
```

当参数格式多于两个时，取第三个参数替换变量 `content` 的值。
否则 `content` 不变。

需要注意的是，Rust 中的 `if` 是一个表达式。允许有返回值。因此以上代码可以改为:

```rust
  let content = if len > 2 {
    args[2].clone()
  } else {
    String::from("default content")
  };
```

以上代码意思是，如果 `len > 2` 条件成立，就使用 `args[2].clone()` 作为 `content` 的值。
否则，就使用 `String::from("default content")` 作为 `content` 的值。

### 循环

Rust 中，循环方式如下：

- `loop` 循环会一直执行，直到遇到 `break` 语句。
- `while` 循环会在条件成立的情况下执行。
- `for` 循环会遍历一个集合中的所有元素。

我们将使用 `while` 实现一个交互式的命令行输入，逐步获取 Todo 的标题与内容，并确认是否创建该条 Todo。

修改 `main.rs` 代码如下:

```rust
fn main() {
  let mut inputs: Vec<String> = Vec::new();
  let mut ok = args[1].clone() == "create";

  while ok {
    let len = inputs.len();

    if len == 0 {
      println!("Please input todo title");

      let mut title = String::new();

      std::io::stdin()
        .read_line(&mut title)
        .expect("read line failed");

      if title.is_empty() {
        continue;
      }

      inputs.push(title.trim().to_string());

    } else if len == 1 {
      println!("Please input todo content");

      let mut content = String::new();

      std::io::stdin()
        .read_line(&mut content)
        .expect("read line failed");

      if content.is_empty() {
        continue;
      }

      inputs.push(content.trim().to_string());
    }

    else {
      println!("title:   [{}]", inputs[0].clone());
      println!("content: [{}]", inputs[1].clone());
      println!("Are you sure to create this todo? (y/n)");

      let mut sure = String::new();

      std::io::stdin()
        .read_line(&mut sure)
        .expect("read line failed");

      if sure.trim().to_lowercase() != "n" {
        ok = false;
      }else{
        inputs.clear();
      }
    }
  }

  let title = inputs[0].clone();
  let content = inputs[1].clone();

  println!("create todo title: {}, content: {}", title, content);
}
```

以上代码中，我们使用了 `while` 循环来实现一个交互式，用于创建 Todo 项的命令行程序。

我们使用了一个状态变量 `ok` 来控制循环，当 `ok` 为 `false` 时，循环会结束。
并在用户输入的内容为空时，使用 `continue` 语句来跳过当前循环。

如果改成 `loop` 循环的话如下所示:

```rust
loop {
  // 其他地方保持不变

  if sure.trim().to_lowercase() != "n" {
    ok = false;
  }else{
    inputs.clear();
  }

  if !ok {
    break
  }
}
```

`while` 和 `loop` 都可以用来循环，效果可以说是等价的。

两者区别在于:

- `while` 适合用于条件驱动的循环，比如获取用户输入并确认。
- `loop` 则更适合结构复杂，需要手动控制循环的情况。例如游戏开发。

现在，执行 `cargo run -- create` 就可以进入交互式界面来创建 Todo 项了。

### for 循环

`for` 循环常用于遍历一个数据集合。

我们将为 CLI 程序增加一个 `list` 命令，用于列出所有的 Todo 项。

修改 `main.rs` 如下：

```rust
fn main() {
  let mut todos: Vec<String> = Vec::new();
  todos.push(String::from("learn rust"));
  todos.push(String::from("work"));
  todos.push(String::from("play"));

  let args: Vec<String> = std::env::args().collect();
  let mut inputs: Vec<String> = Vec::new();

  if args[1].clone() == "list" {
    for todo in todos {
      println!("todo title: {}", todo);
    }
    return;
  }

  let mut ok = args[1].clone() == "create";

  // ...
}
```

相较于需要手动管理索引的 `while` 和 `loop`, `for` 可以更简洁安全的遍历数据集合。
是 Rust 中处理数据集合的首选方式。
