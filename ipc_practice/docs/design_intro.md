# 总体介绍

总的来说，就是使用各种进程间通信方法，完成：

1. 一个进程完成：从文件读内容、传内容到另一个进程、从另一个进程读筛选过后的内容、将从子进程得到的内容写到目标文件。

2. 一个进程完成：从另一个进程接收字符串，返回给另一个进程所有包含特定单词的行。

   注意这里是特定“单词”，比如"their"这一行就不能算"the"出现过。

> 我在作业接近ddl时才发现需要将输出按字典序排序。。。

**代码见[公开仓库](https://github.com/ipChrisLee/BJTU_OS_Projects)。**

**代码见[公开仓库](https://github.com/ipChrisLee/BJTU_OS_Projects)。**

**代码见[公开仓库](https://github.com/ipChrisLee/BJTU_OS_Projects)。**

使用了Rust编写，初学Rust，如有错误还请不吝赐教。



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

在一开始只有上通道运行，child在拿到转发的内容后直接推到自己的一个大buffer里，此时下通道处于闲置状态。

```
src ----> parent >>>>>>> child [ LAAAAAAAAAAAAAAAAAARGE BUFFER ]
          |  ^              |
dst <-----|  |- - - - - - - |
```

在转发结束、处理结束后，child才开始将结果发回parent，此时上通道处于闲置状态。

```
src ----> parent - - - - - child [ LAAARGE BUFFER ]
          |  ^              |
dst <-----|  |<<<<<<<<<<<<<<|
```

这样很明显可以避免死锁，但是也有一定缺点，等下会谈到。



### 实验

在我们项目代码根目录执行：

```shell
cargo run --bin ipc_practice demo/ANNA_KARENINA.txt demo/grep.txt the pipe_with_big_buffer
```

可以正常运行。



## 半双工

### 实现方式简述

使用大buffer自然是一种方式，但是它也是有缺点的：这强制要求child拥有几乎和原文件大小相同的空间，但是如果源文件非常大，甚至可能是无穷大的，这个方式就不合适了。

但是它解决的“我们同时写两个不同的pipe，且它们的剩余容量只会越来越少，这必然会导致风险”的问题还是值得我们考虑。这个方案就借用了通信里的“半双工”的思想——在同一时刻只有一个通道在工作。

怎么保证呢？我们不妨以行为切入点——我们规定在任何一个时刻，要么是上通道在转发一行内容，要么是下通道在发送子进程处理完的内容。

这样，整个工作流程就是线形的了：

1. parent通过上通道转发src的一行内容到child。之后parent在下通道等待信息。
2. child判断是否有target_str，如果有话就发送到下通道；没有的话也发送一些信息表示没有给下通道。
3. parent在child发送到下通道的同时接收信息。



### 实验

```shell
cargo run --bin ipc_practice demo/ANNA_KARENINA.txt demo/grep.txt the pipe_half_duplex
```



## 全双工

### 引入

半双工自然也是可以正常工作的，并且避免了大内存的问题，但是它牺牲了很高的效率——对于在ANNA_KARENINA里找"the"这个任务，在本机测试，半双工方式花了646.02mS，而大buffer花费了304.34mS，这个效率是怎么被牺牲的呢？

我们可以画一个时间流程图分析：

```
parent:
	[1](read from src, transfer to child)[2](wait          )[3](transfer from lower pipe to dst)
child:
	    [1](read from upper pipe           )[2](filter)[3](write to lower pipe              )
```

可以看出，parent在第[1]阶段传输完之后必须等待child判断，这个时间其实可以再从src传输一点内容到upper pipe。此外由于系统调用占用时间之类原因，在child[3]往lower pipe写了一定内容之后，parent才能开始转发，且等到child写完之后一会才能结束转发，这里其实也浪费了等待的时间。

那么有没有一种既不会堵塞，又减少parent等待时间的思路呢？有！这就是全双工模式。



### 实现方式简述

注意，在全双工里，parent的pipe端是Non-blocking的，也就是parent写上通道发现pipe的剩余buffer不够时、parent读下通道发现没有内容时，都是会直接返回的。

全双工其实最重要的就是利用了一个事实：pipe是buffer的，你可以利用这个buffer，而前面无论加buffer还是半双工都没有利用这个性质。因此，现在的pipe通信需要看成这样：

```
src ----> parent ---[    ]---> child (filter on every line)
          |  ^                   |
dst <-----|  |------[    ]-------|
```

我们的设计是这样的：

* 对于parent来说：

  循环以下过程：

  1. 如果src还有内容，转发src内容到上通道，直到上通道满。注意，前面说过这里parent地pipe端是Non-blocking的，所以这里parent不会blocking。
  2. 转发下通道内容到dst，直到下通道没有内容（没有内容不等于关闭）。注意，前面说过这里parent地pipe端是Non-blocking的，所以这里parent不会blocking。

  如果发现src没有内容了，就停止循环第1步，同时关闭上通道。

  直到下通道被关闭。

  关于“上通道上次没写完的东西怎么再写”这些细节参考代码，这里不深入介绍。

* 对于child来说：

  不断从上通道读一行，判断是否包含target_str，如果包含就转发下通道。直到上通道关闭，此时关闭下通道。

如果做个比喻的话，parent现在就像店小二，既要上菜又要收碗，而且轮着进行，且店小二也不会因为现在没碗收了就等着，也不会因为没有顾客就不收碗。

而child就像顾客，不断吃就行了。直到店小二发现店里没菜上了，顾客就不再来店里，只剩下还没吃完的顾客，把最后一个碗给了店小二之后，店小二顺便关上门。

这样的效率是比半双工更高的——因为我们尽可能避免了等待的情况。而和大buffer方式比起来，实验表明双工的方式效率是不相上下的，并且双工不会需要大内存。



### 实验

```shell
rm demo/fifo_*
cargo run --bin ipc_practice demo/ANNA_KARENINA.txt demo/grep.txt the pipe_duplex
```



## 总结

我们针对几种不同的target_str总结了各个方式的时间消耗（大致取了个平均）：

| 方式         | Vengeance | the   |
| ------------ | --------- | ----- |
| 错的实现     | 290mS     | inf   |
| 加个大buffer | 285mS     | 290mS |
| 半双工       | 660mS     | 750mS |
| 全双工       | 307mS     | 275mS |

关于为什么全双工在查找更频繁的单词上速度反而快。。。我看不懂，但是大受震撼。



# 使用socket通信

## 实现简介

和pipe相比，socket的特点是读socket的时候除非另一端结束通信，否则永远不会结束，所以这里一般是在传输时先传输一个大小，再传输内容。

我们套用了pipe中的“加个大buffer”方法实现了socket方式的通信。



## 实验

```shell
rm demo/tmp.sock
cargo run --bin ipc_practice demo/ANNA_KARENINA.txt demo/grep.txt abc socket_method
```



## 时间

找"the"平均耗时670mS。

找"Vengeance"平均耗时720mS。



# 使用共享内存通信

## 实现简介

共享内存方式相比之下更复杂些，因为它涉及到一个时序的问题——必须保证parent写完之后child才从内存中读，不然就会导致读到垃圾内容的可能性。

除此之外和“加个大buffer”也没有区别。



## 实验

```shell
cargo run --bin ipc_practice demo/ANNA_KARENINA.txt demo/grep.txt the shmem_method
```



## 时间

找"the"平均耗时400mS。

找"Vengeance"平均耗时310mS。

耗时更小还有一个原因是这个实验做了多线程（以及Rust使用裸指针访问共享内存，在效率上会高于一般程序）。

