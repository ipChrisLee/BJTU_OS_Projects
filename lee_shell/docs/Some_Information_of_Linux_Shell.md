# Some Reference Link

List some website that may help building this project.

* Some posix shell implemented with Rust:

  * [cicada, an old-school bash-like Unix shell written in Rust.](https://github.com/mitnk/cicada)
  * [nsh, a command-line shell like fish, but POSIX compatible.](https://github.com/nuta/nsh)

* Some useful crates:

  * [nix, which provides friendly bindings to various *nix platform APIs (Linux, Darwin, ...).](https://docs.rs/crate/nix/0.25.0)

  * [crossterm, a pure-rust, terminal manipulation library that makes it possible to write cross-platform text-based interfaces.](https://crates.io/crates/crossterm)

  * [syscall](https://docs.rs/syscalls/0.6.6/syscalls/)

    It is not recommended to use this library, since the version is low, the download volume is low, and it has not been updated for three years, said ZX.

* Some usefule tools:

  * [pest, the elegant parser.](https://pest.rs)



# Information of Shell

## About Builtin Commands

### Why Some Commands are Set to be builtin?

From [answer on Stackoverflow](https://unix.stackexchange.com/a/1359):

> There are two classes of builtins:
>
> 1. Some commands have to be built into the shell program itself because they cannot work if they are external.
>
>    `cd` is one such since if it were external, it could only change its own directory; it couldn't affect the current working directory of the shell. (See also: [Why is `cd` not a program?](https://unix.stackexchange.com/questions/38808/why-is-cd-not-a-program))
>
> 2. The other class of commands are built into the shell purely for efficiency.
>
>    The [`dash`](http://en.wikipedia.org/wiki/Debian_Almquist_shell) [man page](http://linux.die.net/man/1/dash) has a section on builtins which mentions `printf`, `echo`, and `test`as examples of commands in this class.

See [this website](http://c.biancheng.net/view/1136.html) to see all shell builtin commands.

We will list some important builtin commands that we may plan to have:

1. For commands must be builtin:

   * `cd`: Change dir.

   * `exec`: This just finish shell process to doing what is passed to `exec`.

   * `.`: Run executable file. (Yes, this is a builtin.)

   * `exit`: Exit shell with specified status code.

   * `source`: Read and execute commands on a file.

   * For sub job and kill job/process:

     * `jobs`: List the jobs that you are running in the background and in the foreground.

       Notice: To run job on background, you can add `&` to command. e.g. `sleep 1000 &` and `vivado &`.

       To know more about some other information of background/foreground process, see [this webpage](https://www.geeksforgeeks.org/process-control-commands-unixlinux/).

     * `kill`: Kill a process. To know why `kill` should be builtin, see [this](https://unix.stackexchange.com/a/509700) website.

       Since you may need to specify what background job you want to kill (like `kill %1`), `kill` is builtin just like `jobs`.

   * `pushd`, `dirs`, `popd`: 

     First, from [GNU tutorial](https://www.gnu.org/software/bash/manual/html_node/The-Directory-Stack.html), the directory stack is a list of recently-visited directories.

     And, the `pushd` builtin adds directories to the stack as it changes the current directory, and the `popd` builtin removes specified directories from the stack and changes the current directory to the directory removed. The `dirs` builtin displays the contents of the directory stack. The current directory is always the "top" of the directory stack.

   * `wait`: Wait specified process finished, and return its status code.

   * Some commands for alias, env variable, 

     * `alias`, `unalias`
     * `set`, `unset`
     * `export`

     Difference between `set` and `export`: 

     > From [answer from stackexchange](https://unix.stackexchange.com/a/71145):
     >
     > See `help set`: set is used to set shell attributes and positional attributes.
     >
     > Variables that are not exported are not inherited by child processes. `export` is used to mark a variable for export.

   * Some other interesting commands:

     * `times`: Calculate time consumption.
     * `history`: Show history of commands.
     * `trap`: See [Bash trap command](https://linuxhint.com/bash_trap_command/).
     * `type`: Show type of command. (`type` can be used to figure out what is builtin command.)

2. For commands that being builtin is convenience for user:

   * `echo`: Just print.



## About Some Other Feature of Shell

### Run in Background

Use `&` at the end of command, you can make command run in background. Like:

```shell
tree | grep "a" > /dev/null &
```

In my machine, this output:

```
[1] 11901 11902
```

Here `[1]` means the job number, and you can see this process in what `jobs` outputs, and you can kill this job by `kill %1`, as memtioned when introducing shell builtin.

`11901` and `11902` are process id. You can kill two process by these id like `kill 11901` and `kill 11902`.



### Pipe

About why pipe so imporant, see [this video](https://youtu.be/tc4ROCJYbm0), from 6 minutes.

You can see introduction of `pipe` in [wikipedia](https://en.wikipedia.org/wiki/Pipeline_(Unix)).

It is worth highlighting that every command is in progress at the same time.

 

### Logical Operator

Like `&&`, `||`. Just like logical operator in C.



# Rust Support

## System Call

[system call crate](https://docs.rs/linux/0.0.1/linux/syscall/index.html)

* `chdir`



