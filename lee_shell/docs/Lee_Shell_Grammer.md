# Before all

We will replace `$` variables first.(`\${HOME}` WILL be replaced!)

Like `${HOME}/Main`, will be expanded to `/Users/lee/Main` in my PC.



# Grammar

## Some Basic Rule

* I will not support [escape character](https://en.wikipedia.org/wiki/Escape_character) for now. It is too difficult and has little meaning for my OS understanding.

  So anything like 

  * Run `ls\ all.sh`.

  is not supported!

  Escape character in quoted string may be supported.

Here we will define a grammar for our shell scripts. Use [BNF](https://en.wikipedia.org/wiki/Backus–Naur_form) notation.

Lex define: (Use [Regular Expression](https://en.wikipedia.org/wiki/Regular_expression))

```pascal
<BUILTIN>:=cd|exit|exec|kill|export|echo|pwd
<OUTER_CMD>:=[^./\s]+
<SPEC_EXE>:=\.{0,2}(/[^/\s]+)
<QUOTED_STR>:=\"(\\"|[^"])*\"
<NO_BLANK_STR>:=[\w-.]+
```



Grammar define: 

```pascal
<SCRIPTS> := 
	<CMD>					//	ls
	<CMD> "|" <SCRIPTS>		//	ls -a | grep "a"
<CMD> := 
	<CMD_NAME>				//	ls
	<CMD_NAME> <ARGS>		//	ls -a
<ARGS> :=
	<ARG>					{*	Single argument	*}
	<ARG> <ARGS>			
<CMD_NAME> :=
	<BUILTIN>				{*	Built-in commands	*}	
    <OUTER_CMD>				{*	Outer commands like `sleep`	*}
	<SPEC_EXE>				//	./main
							//	/usr/bin/gcc
<ARG> := 
	<QUOTED_STR>
	<NO_BLANK_STR>
```

