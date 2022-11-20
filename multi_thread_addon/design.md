# 个人信息

姓名：李远铄

学号：19211332

本机系统：macOS Monterey 12.5.1



# 错误实现

错误实现可能出现死循环或者出错。

见文件夹下make_chaos_main.c文件内容。



# Mutex实现

见文件夹下mutex_sol.c。

## 假设把加锁和开锁操作分别放置在第一个写操作之前和 第二个写操作之后，能否实现临界区 的保护，为什么？

不行，`while`的条件也属于临界区，也需要保护。



# Peterson实现

见同文件夹下peterson_sol.c。

## 请比较 mutex方案和软件方案的效率。

Mutex的效率明显更高。

理论上来说，在这个简易任务里，Peterson的本质忙等的效率应该是高于Mutex的等待队列机制的，因为后者需要陷入内核态，这个过程中的上下文的切换是非常困难的，但是实际上测出来是Mutex的方式更快。

可能的原因；

* `pthread_mutex_lock`可能会选择忙等（自旋）而不是等待队列的方式。

  我没有找到macOS的clib实现的开源代码，但是musl-libc的实现是这样的：

  * musl-libc的`__pthread_mutex_lock`函数在[pthread_mutex_lock.c](https://git.musl-libc.org/cgit/musl/tree/src/thread/pthread_mutex_lock.c)里，可以看出会调用`__pthread_mutex_timedlock`这个函数。
  * 而`__pthread_mutex_timedlock`函数的实现在[pthread_mutex_timedlock.c](https://git.musl-libc.org/cgit/musl/tree/src/thread/pthread_mutex_timedlock.c)里，可以看70-71行，在一定情况下，这里是会忙等100次。

  这里的自旋都是使用Atomic指令实现的，效率并不低。

* 内存屏障的问题。

  内存屏障强制执行顺序会降低程序运行的效率，这也可能是Peterson的效率更低的原因之一。



# 致谢

在这次作业中，Peterson实现的效率问题我询问了一些身边的同学，特此感谢：

* 感谢张ky同学告知我Windows下的结果，并指出pthread_mutex_lock的实现方式可能导致效率区别。
* 感谢zx同学告诉我musl-libc的存在，使我找到了相关的代码实现。