# Before all

We will replace `$` variables first.(`\${HOME}` WILL be replaced!)

Like `${HOME}/Main`, will be expanded to `/Users/lee/Main` in my PC.



# Grammar

## Some Basic Rule

* I will not support [escape character](https://en.wikipedia.org/wiki/Escape_character). It is too difficult and has little meaning for my OS understanding.

  So anything like 

  * Run `ls\ all.sh`.

  is not supported!

  Escape character in quoted string is supported  (My shell will just forward it to `exec`.).
  
* For our shell, `ls -a | grep "a" << buffer.txt` is a valid script but will cause runtime error. This is because I don't check if a subcommand is redirected twice.

* My shell will not do something special for command running on background. That means, I'm not dealing with possible preemption of stdin and stdout.

We will define a grammar for our shell scripts. Use [BNF](https://en.wikipedia.org/wiki/Backus–Naur_form) notation and [Pest support](https://pest.rs/book/intro.html) in `src/lsh.pest`.

