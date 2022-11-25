# 个人信息

学号：19211332

姓名：李远铄



# 程序设计

就模拟了虚拟内存的控制过程。

两个程序，src/vm_manager.cpp是backing store和memory相同的情况下的模拟，src/vm_manager_with_pg_repl.cpp是backing store比memory大的情况，也就是需要页面替换的情况。

代码使用cmake作为编译环境。



# 运行结果

直接cmake跑一下就行：

```shell
cd vm_manager
rm -rf cmake-build-debug && mkdir cmake-build-debug
cmake -S . -B cmake-build-debug -G Ninja
cd cmake-build-debug && ninja && cd ..
cmake-build-debug/vm_manager data/addresses.txt
diff data/correct_value.txt data/my_ans_value.txt
cmake-build-debug/vm_manager_with_pg_repl data/addresses.txt
diff data/correct_value.txt data/my_ans_value.txt
```

需要ninja支持，如果使用make的话可以将上述命令中的ninja部分替换成make相关。

`diff`可以用于比较答案和我们的答案。

两次运行的结果是：

```
totalAccessCnt=1000
pageFaultCnt=244
tlbHitCnt=11
```



```
totalAccessCnt=1000
pageFaultCnt=538
replCnt=410
tlbHitCnt=11
```



可以看出需要替换的情况下，页错误的频率会增高。



# 编程中的问题

没有。。。

硬要说的话，把`%`写成`&`可能算一个吧。





