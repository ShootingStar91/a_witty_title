# a_witty_title

This is an *interpreter* for some kind of a language. Idk what

Let's start with a simple CFG, implement lexer, parser, executor and tests for that with tests and then add something.


```
prog -> <stmts>

stmts -> <stmt> \n <stmts>

stmt -> print <expr>

expr -> <int_value>
```

Example code

```
print 123
print 456
```

Currently done for the above CFG:
- [x] Scanner
- [x] Nodes
- [x] Parser
- [x] Typecheck
- [x] Execute
- [ ] Error printing
- [ ] Tests


