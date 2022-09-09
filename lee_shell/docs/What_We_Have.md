# Intro

It's too ['fly-bitch'](https://zhidao.baidu.com/question/2017042297888022588.html) to implement a real shell in a sub project of OS course, so [project tutorial](https://www.icourse163.org/question/title/attachment.htm?key=1FACA555A5967A1A3F0864D1C6F89C25-1614176993452&qid=3323784653) mentioned just much less functions your shell should have than real shell.

> Linux 命令解释程序功能设计要求：
>
> （1）选取和设计实现一组内部命令（五条以上）；
>
> （2）外部命令执行采用直接调用 exec 系统调用的方式来实现；
>
> （3）至少一条内部命令采用直接调用相应系统调用的方式来实现；
>
> （4）系统环境变量（至少包括用户主目录 HOME 和可执行程序搜索路径目录 PATH）支 持；
>
> （5）在 Linux 操作系统上启用（或替换原命令解释程序 Shell）并测试验证。

Our lee_shell will:

* Keep environment variables.

  This maybe difficult to implement, but we should have it to run outer command like `sleep` (by run `which sleep`, you can know that the executable file of `sleep` is `/bin/sleep`).

  `~` will **NOT** be supported.

* Support these builtin commands:
  
  * `cd`
  
    This can be implemented with `chdir` system call.
  
  * `exit`
  
  * `exec`
  
  * `.`
  
  * `kill`
  
  * `export`: To keep environment variables.
  
  * `echo`: Just print arguments `echo` get.

  * `pwd`
  
* Run command from executable files.

  In my PC, when I run `ps`, it is `/bin/ps` that is running. When I run `apt`, it is `/usr/bin/apt` that is running.

  So where does shell find what executable file to run when you input a command? `$PATH`.

  You can `echo $PATH` to see where shell will find command executable files.

  So, to run outer commands, you should keep `$PATH` in your shell.

* Pipe

  Pipe is interesting, and I think to implement a pipe is a great chance for us to learn more about system call related to file and file descriptor. So I include pipe implementation on my shell.

* Redirect

  Same reason as pipe.



