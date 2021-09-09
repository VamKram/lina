# x86_64 操作系统 write in rust

💗💗💗💗


## How to use

```sh
cargo build
```

```sh
cargo run
```

如果
![img.png](doc/img.png)

```shell
# 报错尝试
rustup component add rust-src llvm-tools-preview
```

### Commands

绑定环境

```sh
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```

**从源码编译 **

```sh
rustup component add rust-src
```

**使用**


```sh
cargo run
```

