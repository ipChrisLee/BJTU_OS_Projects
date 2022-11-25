# 个人信息

学号：19211332

姓名：李远铄



# 程序设计

简单的模拟。使用双向链表维护关系。

代码使用cmake作为编译环境。



# 输入格式

这里先定义下输入格式：

1. 请求连续内存分配：

   ```
   RQ ${process_name} ${size} ${mode}
   ```

   要求：

   * `${process_name}`：为请求内存的进程名字，必须保证满足正则表达式`[0-9a-zA-Z]+`。
   * `${size}`：为请求的内存大小，必须满足正则表达式`[1-9][0-9]*`。
   * `${mode}`：为`F`（first fit）、`B`（`best fit`）或者`W`（`worst fit`）中的一个。

   会按照`${mode}`模式为进程`${process_name}`分配连续的大小为`${size}`的内存。

   如果失败，会输出`RQ: failed`。

2. 释放进程所有存储：

   ```
   RL ${process_name}
   ```

   要求：

   * `${process_name}`：为想要释放存储的的进程名字，必须保证满足正则表达式`[0-9a-zA-Z]+`。

   会释放所有已经分配给`${process_name}`的存储。

3. 将所有未分配存储整理到一个快内：

   ```
   C
   ```

4. 展示所有存储的状态：

   ```
   STAT ${comment}
   ```

   要求：

   * `${comment}`：为这次状态输出的的注释，满足正则表达式`[0-9a-zA-Z]+`。

   输出格式类似：

   ```
   ${comment} :
   Addresses [0:315001) Process P1
   Addresses [315001: 512501) Process P3
   Addresses [512501:625575) Unused
   Addresses [625575:725100) Process P6
   ```



# 运行结果

编译代码：

```shell
cd cont_memory_allocation
rm -rf cmake-build-debug && mkdir cmake-build-debug
cmake -S . -B cmake-build-debug -G Ninja
cd cmake-build-debug && ninja && cd ..
```

这里准备了一个demo，完成上述过程后执行：

```
cmake-build-debug/cont_memory_allocation 1024 < data/data.in
```

就可以看到demo数据的输出。这里按顺序解释一下：

1. 输入：

   ```
   RQ P0 512 F
   RQ P1 256 F
   STAT 1
   ```

   输出：

   ```
   1: 
   Addresses [0:512) Process P0
   Addresses [512:768) Process P1
   Addresses [768:1024) Unused
   ```

2. 输入：

   ```
   RL P0
   STAT 2
   RQ P0 128 F
   STAT 3
   ```

   输出：

   ```
   2: 
   Addresses [0:512) Unused
   Addresses [512:768) Process P1
   Addresses [768:1024) Unused
   3: 
   Addresses [0:128) Process P0
   Addresses [128:512) Unused
   Addresses [512:768) Process P1
   Addresses [768:1024) Unused
   ```

   解释：

   这里展示了存储释放和再分配。

3. 输入：

   ```
   RL P0
   STAT 4
   RQ P0 128 B
   STAT 5
   ```

   输出：

   ```
   4: 
   Addresses [0:512) Unused
   Addresses [512:768) Process P1
   Addresses [768:1024) Unused
   5: 
   Addresses [0:512) Unused
   Addresses [512:768) Process P1
   Addresses [768:896) Process P0
   Addresses [896:1024) Unused
   ```

   解释：

   这里展示了best hit的策略。

4. 输入：

   ```
   RQ P3 510 F
   RQ P4 1 W
   STAT 6
   ```

   输出：

   ```
   6: 
   Addresses [0:510) Process P3
   Addresses [510:512) Unused
   Addresses [512:768) Process P1
   Addresses [768:896) Process P0
   Addresses [896:897) Process P4
   Addresses [897:1024) Unused
   ```

   解释：

   这里展示了worst hitd的策略。

5. 输入：

   ```
   C
   STAT 7
   ```

   输出：

   ```
   7: 
   Addresses [0:510) Process P3
   Addresses [510:766) Process P1
   Addresses [766:894) Process P0
   Addresses [894:895) Process P4
   Addresses [895:1024) Unused
   ```

   解释：

   这里展示的是聚集空存储的功能。

6. 输入：

   ```
   RL P0
   RL P3
   STAT 8
   RL P1
   STAT 9
   RL P4
   STAT 10
   ```

   输出：

   ```
   8: 
   Addresses [0:510) Unused
   Addresses [510:766) Process P1
   Addresses [766:894) Unused
   Addresses [894:895) Process P4
   Addresses [895:1024) Unused
   9: 
   Addresses [0:894) Unused
   Addresses [894:895) Process P4
   Addresses [895:1024) Unused
   10: 
   Addresses [0:1024) Unused
   ```

   解释：

   这里是展示一种特殊情况，如果释放一个两边都是空的存储时，应该能将存储合并成一大块。

7. 输入：

   ```
   C
   STAT 11
   ```

   输出：

   ```
   11: 
   Addresses [0:1024) Unused
   ```

   解释：

   展示了对完整存储块的C操作。

8. 输入：

   ```
   RQ P0 512 F
   RQ P1 1024 F
   STAT 12
   RQ P1 512 F
   STAT 13
   ```

   输出：

   ```
   RQ: Failed
   12: 
   Addresses [0:512) Process P0
   Addresses [512:1024) Unused
   13: 
   Addresses [0:512) Process P0
   Addresses [512:1024) Process P1
   ```

   解释：

   这里展现对于超出存储能力的需求的应对。

9. 输入：

   ```
   C
   STAT 14
   ```

   输出：

   ```
   14: 
   Addresses [0:512) Process P0
   Addresses [512:1024) Process P1
   ```

   解释：

   这里展现对于满存储块的C操作。

这个demo应该算把一部分需求都满足了。



