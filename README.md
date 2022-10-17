
<img src="https://cdn.dribbble.com/users/763/screenshots/1409522/her_dribbble.jpg" />

# x86_64 操作系统 write in rust

💗💗💗💗


## How to use

```sh
cargo build

# 运行
cargo run
```

如果出现这个报错
![img.png](doc/img.png)

```shell
# 报错尝试增加 llvm tools 到项目
rustup component add rust-src llvm-tools-preview
```

参考下这个问题 解决一些不同架构的思路
![img.png](doc/img1.png)

### Commands

绑定环境

```sh
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```

** 从源码编译 **

```sh
rustup component add rust-src
```

** 使用 **


```sh
cargo run
```


> inspiration by https://os.phil-opp.com/
