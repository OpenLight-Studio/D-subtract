# D Substract (D--) 编程语言
### 一个专门为Bigenner编写的编程语言
###### Designed by OpenLight LLC with heart

## 功能
- 解析和执行程序：支持变量声明（let）、数学表达式（+ - * / 括号）、while 循环
- 编译为 x86-64 机器码：直接生成二进制代码

## 语法
```
program ::= statement*
statement ::= let id = expr | def id() { program } | if expr { program } [else { program }] | while expr { program } | id = expr | expr
expr ::= term ((+|-) term)*
term ::= factor ((*|/) factor)*
factor ::= number | float | id | id() | ( expr )
```

注释：// 行注释

## 使用
编译器将程序编译为 x86-64 机器码并写入 output.bin，然后直接运行。

示例：
```
./target/release/dsubtract "let x = 5"
./target/release/dsubtract example.ds
```
生成机器码到 output.bin 并执行，打印退出码。

## 构建
```bash
cd dsubtract
cargo build --release
```
