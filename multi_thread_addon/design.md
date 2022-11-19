# 个人信息

姓名：李远铄

学号：19211332



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

理论上来说，在这个简易任务里，Peterson的本质忙等的效率应该是高于Mutex的等待队列机制的，但是实际上测出来是Mutex的方式更快。

目前想到的一些原因：

* 据zky同学说，pthread_mutex_t可能会选择忙等而不是等待队列的方式。

  但是我没有在[Linux man page](https://linux.die.net/man/3/pthread_mutex_lock)找到相关描述，使用"pthread_mutex_t poll"和"pthread_mutex_t 忙等"分别在Google、百度上搜索都没有搜到相关结果。

* 内存屏障的问题。

  内存屏障会导致执行顺序不变，但是这是以效率下降为代价的。