Try a little scripts:

```txt
/Users/lee/Main/Proj/On_BJTU/OS_Projects:touch buffer.txt
/Users/lee/Main/Proj/On_BJTU/OS_Projects:ls -a
.               .DS_Store       .gitignore      Cargo.toml      README.md       lee_shell       log4rs_lsh.yaml
..              .git            Cargo.lock      LICENSE.md      buffer.txt      log             target
/Users/lee/Main/Proj/On_BJTU/OS_Projects:ls -a | grep "a" | sort > buffer.txt
/Users/lee/Main/Proj/On_BJTU/OS_Projects:cat buffer.txt
Cargo.lock
Cargo.toml
log4rs_lsh.yaml
target
/Users/lee/Main/Proj/On_BJTU/OS_Projects:grep "Cargo" < buffer.txt
Cargo.lock
Cargo.toml
/Users/lee/Main/Proj/On_BJTU/OS_Projects:grep "pest" < Cargo.lock > buffer.txt
/Users/lee/Main/Proj/On_BJTU/OS_Projects:cat buffer.txt
 "pest",
 "pest_derive",
name = "pest"
name = "pest_derive"
 "pest",
 "pest_generator",
name = "pest_generator"
 "pest",
 "pest_meta",
name = "pest_meta"
 "pest",
/Users/lee/Main/Proj/On_BJTU/OS_Projects:history
7 : touch "buffer.txt"
6 : ls "-a"
5 : ls "-a" | grep "a" | sort > buffer.txt
4 : cat "buffer.txt"
3 : grep "Cargo" < buffer.txt
2 : grep "pest" < Cargo.lock > buffer.txt
1 : cat "buffer.txt"
0 : history
/Users/lee/Main/Proj/On_BJTU/OS_Projects:! 6
.               .DS_Store       .gitignore      Cargo.toml      README.md       lee_shell       log4rs_lsh.yaml
..              .git            Cargo.lock      LICENSE.md      buffer.txt      log             target
/Users/lee/Main/Proj/On_BJTU/OS_Projects:echo "a" > buffer.txt
/Users/lee/Main/Proj/On_BJTU/OS_Projects:history
10 : touch "buffer.txt"
9 : ls "-a"
8 : ls "-a" | grep "a" | sort > buffer.txt
7 : cat "buffer.txt"
6 : grep "Cargo" < buffer.txt
5 : grep "pest" < Cargo.lock > buffer.txt
4 : cat "buffer.txt"
3 : history
2 : ls "-a"
1 : echo "a" > buffer.txt
0 : history
/Users/lee/Main/Proj/On_BJTU/OS_Projects:! 4
a
/Users/lee/Main/Proj/On_BJTU/OS_Projects:!!
a
/Users/lee/Main/Proj/On_BJTU/OS_Projects:tree / | grep "admin" > buffer.txt &
/Users/lee/Main/Proj/On_BJTU/OS_Projects:ps -a
  PID TTY           TIME CMD
24421 ttys000    0:00.03 login -pfl lee /bin/bash -c exec -la zsh /bin/zsh
24423 ttys000    0:00.23 -zsh
75672 ttys001    0:00.12 /bin/zsh -il
23100 ttys002    0:00.12 target/debug/lee_shell
23348 ttys002    0:00.00 (lee_shell)
23349 ttys002    0:00.98 /usr/local/bin/tree /
23350 ttys002    0:00.04 /usr/bin/grep admin
23358 ttys002    0:00.00 /bin/ps -a
44759 ttys004    0:00.02 login -pfl lee /bin/bash -c exec -la zsh /bin/zsh
44760 ttys004    0:00.14 -zsh
/Users/lee/Main/Proj/On_BJTU/OS_Projects:wc -l buffer.txt
     119 buffer.txt
/Users/lee/Main/Proj/On_BJTU/OS_Projects:cd ..
/Users/lee/Main/Proj/On_BJTU:ls
OS_Projects
/Users/lee/Main/Proj/On_BJTU:ls OS_Projects
Cargo.lock      LICENSE.md      buffer.txt      log             target
Cargo.toml      README.md       lee_shell       log4rs_lsh.yaml
/Users/lee/Main/Proj/On_BJTU:cd OS_Projects
/Users/lee/Main/Proj/On_BJTU/OS_Projects:exit
```
