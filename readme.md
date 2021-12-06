# Rust 处理错误的机制

给大家简单介绍一下如何用比较好的姿势，处理Rust中的错误捕获和处理。

先看项目的依赖文件 Cargo.toml

```
[package]
name = "rust-anyhow-example"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.44"
```

依赖中 anyhow 是必须的，错误处理主要用到了它的机制。 serde是序列化相关的，这里是用它来展示一个简单的应用案例，你可以自己写自己的例子。

在我们的例子里，我们从文件中读取一段文本，然后反解析json格式为rust的数据结构体。

源代码如下：
```rust
extern crate anyhow;
extern crate serde;
extern crate serde_json;

use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ClusterMap {
    name: String,
    group: i32,
}

fn get_cluster_info() -> Result<ClusterMap> {
    let config = std::fs::read_to_string("cluster.json")
        .context("failed to read config file")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}

fn main() {
    let _ = match get_cluster_info() {
        Ok(cm) => println!("{:?}", cm),
        Err(err) => println!("{:?}", err),
    };
}
```

先解释一下程序原理：当解析出错的时候，我们打印一行：failed to read config file, 而当工作正常的时候，我们打印配置文件的内容。

好，先看main函数，main函数是我们所有rust编程应用里面必须有的一个入口main函数。

好，看里面写了什么，一个match 模式匹配，匹配的是get_cluster_info函数的结果， 结果要么成功，要么报错。

结果Ok，Err来自于anyhow的Result对象。 感兴趣的可以深入进去看。这里我们只需要知道，它会响应Ok和Err两种结构。
成功的是，当然是Ok。
失败的时候，当然是Err。

再看进去 get_cluster_info方法里面的实现，两行代码，每行都是用？问号结尾，问号是个简略缩写。它的意思是，这里可能是正常执行，也是有可能报错，报什么错？Err。
当触发Err的时候，anyhow给你封装了，你就不用自己逐层抛出异常Err了。一个问号解决问题。

如果你想自定义错误messgage，可以借用anyhow::context来把错误message替换掉。是不是很简单？

好，最核心的错误处理机制，已经讲清楚了。

我们在main当中需要用match捕获Err，该中断业务，你就中断业务，该写日志什么的，你就写日志，全部取决于你。

如果你对panic无所谓，你也可以不用match，崩了就崩了吧。完全取决于你。

我还是喜欢把每一个Err，都用match 捕获起来。每个函数，要么执行成果，返回Ok，要么执行失败，返回Err，这个不难理解吧。
