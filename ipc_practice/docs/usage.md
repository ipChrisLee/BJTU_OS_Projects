1. `pipe`方式：

   1. Navie的pipe实现：

      ```shell
      cargo run --bin ipc_practice demo/ANNA_KARENINA.txt demo/grep.txt Vengeance pipe_navie
      ```

      这里是先查找"Vengeance"这个单词，可以发现能够正常运行。

      但是如果查找一些非常常见的单词，比如"the"，程序就会死锁：

      ```shell
      cargo run --bin ipc_practice demo/ANNA_KARENINA.txt demo/grep.txt the pipe_navie
      ```

      可以通过`ps`查找这个进程的pid，然后再`ps -l {pid}`来查这个进程的状态。可以看出来目前是`S+`状态，也就是正在sleeping。

   2. 使用Buffer的pipe实现：

      ```shell
      cargo run --bin ipc_practice demo/ANNA_KARENINA.txt demo/grep.txt the pipe_with_big_buffer
      ```

      这次可以实现了。

   3. 使用半双工方式实现：

2. `socket`方式

   ```shell
   rm demo/tmp.sock
   cargo run --bin ipc_practice demo/ANNA_KARENINA.txt demo/grep.txt abc socket_method
   ```

3. `shmem`方式

   ```shell
   cargo run --bin ipc_practice demo/ANNA_KARENINA.txt demo/grep.txt the shmem_method
   ```



