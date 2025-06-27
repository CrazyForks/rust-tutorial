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

## 模式匹配

目前，我们的 CLI 程序包含两个命令：

- `create`：创建 Todo 项；
- `list`：查看 Todo 列表。

但随着功能逐渐扩展，代码也逐渐变得臃肿、不易维护。

为了解决这个问题，Rust 提供了一种更为优雅强大的方式，即模式匹配 `match`。

我们可以使用 `match` 匹配输入内容，根据不同的匹配进行相应的逻辑。

```rust
fn main() {
    let mut todos: Vec<String> = Vec::new();
    todos.push(String::from("learn rust"));
    todos.push(String::from("work"));
    todos.push(String::from("play"));

    let args: Vec<String> = std::env::args().collect();

    match args[1].clone().as_str() {
        "create" => {
            let mut inputs: Vec<String> = Vec::new();
            let mut ok = true;

            // ...

            println!("create todo title: {}, content: {}", title, content);
        }
        "list" => {
            for todo in todos {
                println!("todo title: {}", todo);
            }
        }
        _ => {
            println!("unknown command");
        }
    }
}
```

通过以上代码，我们不难发现，`match` 很像其他语言中的 `switch`,
但是 Rust 的 `match` 则相对于 `switch` 更加强大。它可以:

- 匹配多种可能的值。
- 支持变量绑定和解构。
- 必须覆盖所有情况，但允许使用 `_` 匹配所有。
- 它同时是表达式，可以返回值。
- 支持守卫条件，可以使用 `if` 增加条件限制。

下面是一个简单的示例:

```rust
let auth_level: i32 = 2;

let role = match auth_level {   // 返回值给变量声明
  0 => "Guest",                 // 单值匹配
  1 | 2 => "User",              // 多值匹配
  n if n >= 16 => "Admin",      // 守卫语句
  _ => "Unknow"                 // 默认分支，匹配所有剩余情况
}
```

## 结构体

目前，我们的 Todo 项分别有 Title 和 Content 两个属性。

为了更好的表达两者之间的关系，我们可以使用 Rust 中的结构体将它们组织在一起。

结构体是一种可以由我们自定义的数据类型。能够将多种字段打包在一起形成一个整体，便于管理，传递和扩展。

改造 `main.rs`。

```rust
struct TodoItem {
    title: String,
    content: String,
}

fn main() {
  let mut todos: Vec<TodoItem> = Vec::new();
  todos.push(TodoItem {
      title: "learn rust".to_string(),
      content: "read rust book".to_string(),
  });
  todos.push(TodoItem {
      title: "work".to_string(),
      content: "complete required".to_string(),
  });
  todos.push(TodoItem {
      title: "play".to_string(),
      content: "play game".to_string(),
  });

  // ...
        "list" => {
            for todo in todos {
                println!("todo title: {}, content: {}", todo.title, todo.content);
            }
        }
  // ...
}
```

以上代码中，我们定义了一个名为 `TodoItem` 的结构体，它包含了 `title` 和 `content` 两个属性，分别代表 Todo 项的标题和内容。

在 `main` 函数中，我们用一个 `Vec<TodoItem>` 来保存多个 Todo 项，每个 Todo 项都是一个结构体实例。

当匹配到 `"list"` 命令时，我们遍历 `todos` 列表，打印每个 Todo 的标题和内容，实现了简单的查看功能。

## 函数

在先前的代码中，我们定义了 `todos` 变量来存储 Todo 项，并逐个实例化 Todo 项然后添加到 `todos` 中。

我们实例化 Todo 项的代码如下，可以看到，有些繁琐：

```rust
TodoItem {
  title: "learn rust".to_string(),
  content: "read rust book".to_string(),
}
```

为了避免每次都写重复的转换和构造过程，我们可以使用 Rust 的函数。

函数是一段可以被重复调用的代码块。用于完成特定的任务。可以：

- 将某段功能独立出，从而进行复用，避免代码重复。
- 通过函数名描述功能，让代码结构清晰，提升可读性。
- 需要修改则只需要修改函数内部，并不会影响外部调用，增加了维护性和扩展性。
- 通过传递不同参数，来改变函数内部走向，实现不同的功能。
- 可以返回值，实现外部与内部交互。

```rust
fn create_todo_item(title: &str, content: &str) -> TodoItem {
    TodoItem {
        title: title.to_string(),
        content: content.to_string(),
    }
}

fn main() {
  let mut todos: Vec<TodoItem> = Vec::new();
  todos.push(create_todo_item("learn rust", "read rust book"));
  todos.push(create_todo_item("work", "complete required"));
  todos.push(create_todo_item("play", "play game"));

  // ...
}
```

在以上示例中，`create_todo_item` 接受两个 `&str` 类型的参数。返回 `TodoItem` 类型。
在它内部，实现了将两个 `&str` 参数转换为 `String` 类型的值，并绑定到 `TodoItem` 类型的实例上。

随后，我们只需要使用 `create_todo_item("title", "content");` 就可以实例化一个 `TodoItem` 类型了。

相较于先前需要手动指定结构体类型、列出所有字段并逐一进行字符串转换的写法，使用函数可以大大减少重复代码，提升开发效率。

通过封装 `create_todo_item` 函数，我们只需要传入标题和内容两个参数，就能快速创建一个 `TodoItem` 实例，既简洁，又易于阅读和维护。

这样的封装方式在实际开发中非常常见，也体现了函数抽象的核心思想：隐藏实现细节，对外暴露清晰的接口。

## 模块化

随着程序逐渐复杂, 我们的 `main.rs` 文件中的代码越来越多，所有的逻辑都堆在一起，不仅可读性差，也不利于维护和扩展。

在 Rust 中，模块化是一种常见的代码组织方式。通过将代码拆分成多个文件，让每个文件负责不同的功能。
可以让代码结构更加清晰，职责划分明确。

目前，我们的项目结构如下:

```bash
|- src/
  |- main.rs
```

新建一些文件，项目结构如下：

```bash
|- src/
  |- todo/        # todo 模块目录
    |- core.rs    # todo 核心逻辑
    |- create.rs  # create todo 命令
    |- list.rs    # list todo 命令
  |- main.rs      # 程序入口
  |- todo.rs      # 子模块声明
```

### 访问修饰符

```rust
// src/todo/core.rs
pub struct TodoItem {
    pub title: String,
    pub content: String,
}

pub fn create_todo_item(title: &str, content: &str) -> TodoItem {
    TodoItem {
        title: title.to_string(),
        content: content.to_string(),
    }
}

pub fn get_todo_list() -> Vec<TodoItem> {
    let mut todos: Vec<TodoItem> = Vec::new();

    todos.push(create_todo_item("learn rust", "read rust book"));
    todos.push(create_todo_item("work", "complete required"));
    todos.push(create_todo_item("play", "play game"));

    return todos;
}
```

在以上代码中，我们可以看见，不论是结构体还是函数，在声明前面都有着一个 `pub` 关键字。`pub` 表示该结构体或函数是公开的，其他模块可以访问。

Rust 中，默认所有内容都是私有的，如果不添加 `pub`，则该内容只能在当前模块中访问。

```rust
// src/todo/create.rs
pub fn create_todo() {
    let mut inputs: Vec<String> = Vec::new();
    let mut ok = true;

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
        } else {
            println!("title:   [{}]", inputs[0].clone());
            println!("content: [{}]", inputs[1].clone());
            println!("Are you sure to create this todo? (y/n)");

            let mut sure = String::new();
            std::io::stdin()
                .read_line(&mut sure)
                .expect("read line failed");

            if sure.trim().to_lowercase() != "n" {
                ok = false;
            } else {
                inputs.clear();
            }
        }
    }

    let inputs_len = inputs.len();

    let title = if inputs_len > 0 {
        inputs[0].clone()
    } else {
        String::from("default title")
    };
    let content = if inputs_len > 1 {
        inputs[1].clone()
    } else {
        String::from("default content")
    };

    println!("create todo title: {}, content: {}", title, content);
}
```

### 模块路径解析

```rust
// src/todo/list.rs
use super::core::TodoItem;

pub fn list_todo(todos: &Vec<TodoItem>) {
    for todo in todos {
        println!("todo title: {}, content: {}", todo.title, todo.content);
    }
}
```

在以上代码中，可以看见 `use super::core::TodoItem;` 这行语句。
这是在引用其他模块的内容。

Rust 使用类型文件夹路径的方式来引用不同的模块内容，并提供了三种路径前缀：

- `super`, 表示当前模块的父模块。
- `self`, 表示当前模块自身。
- `crate`, 表示当前根模块，即 `src` 目录。

回到 `use super::core::TodoItem;`, 我们可以得知它的作用是引用 `list` 模块的父模块下的子模块 `core` 的 `TodoItem` 并使用它。
换句话说，它的作用是从兄弟模块 `core` 引入 `TodoItem` 并使用。

> 引用的内容必须使用 `pub` 关键字公开，否则无法引用。

### 模块声明

```rust
// src/todo.rs
pub mod core;
pub mod create;
pub mod list;
```

我们可以使用 `mod` 关键字来声明子模块。同样的，子模块需要 `pub` 关键字修饰来公开给外部访问。

在 Rust 中，每个模块都有一个 `mod.rs` 文件，该文件是模块的入口文件。
在 `mod.rs` 文件中，我们可以定义模块的公开内容，如结构体、函数、模块等。

如果使用的 `rustc` 版本在 1.30 以前，那么这是唯一声明模块入口的方法。

但如果实在 1.30 以后，那么可以在模块同级位置创建一个同名的 `.rs` 文件来作为模块入口声明。

这里使用的就是使用与模块同名的 `.rs` 文件作为模块入口声明。

```rust
// src/main.rs
mod todo;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let todos = todo::core::get_todo_list();

    match args[1].clone().as_str() {
        "create" => todo::create::create_todo(),
        "list" => todo::list::list_todo(&todos),
        _ => {
            println!("unknown command");
        }
    }
}
```

## 数据持久化

目前，我们的任务数据是保存在内存中的。当程序退出时，这些数据会随之消失。

为了让用户的数据在下次启动程序时依然可用，我们需要将数据持久化，也就是保存到磁盘上。

一个简单且常见的做法是：将数据保存到一个文件中。程序启动时从文件中读取任务列表，退出或修改数据时再将更新后的任务保存回文件。

要实现这一功能，我们需要先让数据支持序列化与反序列化。

- 序列化是指将结构体等内存对象转换为可保存的格式。
- 反序列化则是将这些格式转换回结构体对象。

### 添加依赖

在实际开发中，我们通常会将常用的一些功能给封装起来，方便后续重复使用。

更进一步的就是将这些功能封装为一个库包给发布到网络中，让其他人也可以使用。
如果项目中有用到某个库包，那么说明项目依赖于这个库包。这个库包就是项目的依赖。

`cargo` 就是 Rust 的库包管理工具。我们可以通过它安装项目依赖库包。

为了让 `TodoItem` 能被正确地序列化/反序列化, 我们需要引入第三方库 `Serde` 以及 `serde_json`。

在项目根目录下执行命令:

```bash
cargo add serde --features derive     # 增加 serde 依赖，并开启 derive 功能
cargo add serde_json                  # 增加 serde_json 依赖
```

### 为结构体实现方法

在 Rust 中，我们可以使用 `impl` 关键字为结构体定义方法，将结构体和它的行为组织在一起。

我们来为 `TodoItem` 添加创建、序列化和反序列化的方法：

```rust
// src/todo/core.rs
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize)]
pub struct TodoItem {
    pub title: String,
    pub content: String,
}

impl TodoItem {
    pub fn new(title: &str, content: &str) -> Self {
        create_todo_item(title, content)
    }

    pub fn serializer(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn deserializer(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }
}
```

我们在结构体上添加了 `#[derive(Serialize, Deserialize)]` 派生宏，
这会自动为 `TodoItem` 实现 `Serde` 所需的转换逻辑。避免了手动实现的复杂性。

此外，我们还添加了：

- `new` 方法：用于创建一个新的 `TodoItem`，现在可以直接用 `TodoItem::new(...)` 替代之前的 `create_todo_item(...)`。
- `serializer` 方法：将当前实例转换为 JSON 字符串。
- `deserializer` 方法：从 JSON 字符串还原为 `TodoItem` 实例。

通过这种方式，我们就为 `TodoItem` 实现了基本的序列化与反序列化功能。接下来，我们就可以在程序中使用文件来保存和读取任务数据了。

### 文件操作

现在，我们已经实现了 `TodoItem` 的序列化和反序列化。接下来，我们需要将数据存储到文件中，实现持久化。

Rust 提供了标准库 `std::fs` 用于文件读写。我们将使用它实现以下两个功能：

- 保存 Todo 列表到文件。
- 读取 Todo 列表到程序。

增加 `src/todo/storage.rs` 文件，并在 `src/todo.rs` 中声明并公开该模块。

```rust
// src/todo/storage.rs
use super::core::TodoItem;
use std::fs;

pub fn read_todo_list(save_file: &str) -> Vec<TodoItem> {
    let mut result: Vec<TodoItem> = Vec::new();

    if let Ok(content) = fs::read_to_string(save_file) {
        if let Ok(mut list) = serde_json::from_str(content.as_str()) {
            result.append(&mut list)
        }
    };

    // 如果没有读取到任何数据，提供默认示例
    if result.len() == 0 {
        result.push(TodoItem::new("learn rust", "read rust book"));
        result.push(TodoItem::new("work", "complete required"));
        result.push(TodoItem::new("play", "play game"));
    }

    return result;
}

pub fn save_todo_list(save_file: &str, todos: &Vec<TodoItem>) {
    fs::write(save_file, serde_json::to_string(todos).unwrap()).unwrap();
}
```

```rust
// src/main.rs
use crate::todo::storage::{read_todo_list, save_todo_list};

mod todo;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let save_file = "todo.json";
    let mut todos = read_todo_list(save_file);

    match args[1].clone().as_str() {
        "create" => todo::create::create_todo(&mut todos),
        "list" => todo::list::list_todo(&todos),
        _ => {
            println!("unknown command");
        }
    }

    save_todo_list(save_file, &todos);
}
```

这样，当我们执行以下命令时：

```bash
cargo run -- list    # 显示 Todo 列表（包括初始默认内容）
cargo run -- create  # 添加 Todo 项（修改会被保存）
```

数据将自动从 `todo.json` 读取并写入，实现完整的本地持久化。
