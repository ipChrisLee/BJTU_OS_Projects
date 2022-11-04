# 总体介绍

总的来说，就是使用各种进程间通信方法，完成：

1. 一个进程完成：从文件读内容、传内容到另一个进程、从另一个进程读筛选过后的内容、将从子进程得到的内容写到目标文件。
2. 一个进程完成：从另一个进程接收字符串，返回给另一个进程所有包含特定单词的行。

代码见[公开仓库](https://github.com/ipChrisLee/BJTU_OS_Projects)。



# 使用pipe通信

## 综述

使用pipe通信可以看成这种关系：

```
src ----> parent ------> child (filter on every line)
          |  ^              |
dst <-----|  |------<-------|
```

我们给这个模式中的各个元素做一个命名，方便后续阐述：

* parent到child的管道称为上管道，或者`pipe_from_parent_to_child`。
* child到parent的管道称为下管道，或者`pipe_from_child_to_parent`。
* 被查找的字符串叫`target_str`。

可以看出，由于pipe是单向的，所以在parent和child之间至少需要两个管道，这里存在一个问题：这两个管道到底应该如何工作？

我们先阐述一种错的方式，之后再介绍一些不同的正确实现。

除非特殊声明，我们这里假设pipe都是blocking等待，即`O_NON_BLOCKING`没有设置。



## 错的实现

### 实现方式简述

一个实现是这样的（使用简化的Rust描述。后续的代码都是这种模式，不再备注）：

```rust
fn parent(){
    let mut buf = String::new();
    while src().read_line(&mut buf).unwrap() > 0 {
        write(pipe_from_parent_to_child.1, buf.as_bytes()).unwrap();
        buf.clear();
    }
    buf.clear();
    //	...	
}
fn child(){
    let mut buf = String::new();
    while pipe_from_parent_to_child().read_line(&mut buf).unwrap() > 0 {
        if buf.split_whitespace().any(|word| word.eq(target_str)) {
            write(pipe_from_child_to_parent.1, buf.as_bytes()).unwrap();
        }
        buf.clear();
    }
}
```

也就是：parent一直转发文件内容到上通道，child边从上通道接收内容并选取合适的内容转发给下通道。

这会导致死锁，原因如下：

由于下通道在parent没有转发完所有src内容前都不会被parent读取，而child会一直写入被选出来的行到下通道。一开始可能下通道的buffer可以承受。

```
src ----> parent ------> child (filter on every line)
          |  ^              |
dst <-----|  |====----------|
```

但是一旦被选出的内容过多，会导致下通道的buffer被占满：

```
src ----> parent ------> child (filter on every line)
          |  ^              |
dst <-----|  |==============|
```

注意此时child被blocking了！它会一直等待下通道被疏通！但是因为parent还在做转发，不会清理下通道，所以parent转发的内容会在上通道积累：

```
src ----> parent ---===> child (filter on every line)
          |  ^              |
dst <-----|  |==============|
```

因为child被blocking了，不会接收上通道的内容，最终上通道越积累越多，最后就是parent也被blocking了。

```
src ----> parent ======> child (filter on every line)
          |  ^              |
dst <-----|  |==============|
```

这下就会陷入死锁：parent在等待上通道被清，child在等待下通道被清，最终谁都等不到谁。



### 实验

在我们项目代码根目录执行：

```shell
cargo run --bin ipc_practice demo/ANNA_KARENINA.txt demo/grep.txt Vengeance pipe_navie
```

会发现可以正常运行——因为此时上述问题的起因“下通道buffer满”这件事不会发生。

但是如果搜索一些常用词，比如：

```shell
cargo run --bin ipc_practice demo/ANNA_KARENINA.txt demo/grep.txt the pipe_navie
```

就会发现代码无法结束执行，使用`ps`和`ps -l`查看进程状态，发现两个进程都处于睡眠状态——这就是错误实现的最终结果，两个进程都blocking了。



## 加个大buffer

### 实现方式简述

很明显，错误实现的问题在于，我们同时写两个不同的pipe，且它们的剩余容量只会越来越少，这必然会导致风险。

基于此，首先提出一种比较简单的想法：我们不如规定，我们只顺序地使用两个管道。

在一开始只有上通道运行，child在拿到转发的内容后直接推到自己的一个大buffer里。此时下通道处于闲置状态。

```
src ----> parent >>>>>>> child [ LAAAAAAAAAAAAAAAAAARGE BUFFER ]
          |  ^              |
dst <-----|  |- - - - - - - |
```

在转发结束、处理结束后，





## 总结

时间

TBD

