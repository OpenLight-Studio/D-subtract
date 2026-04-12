# D-Subtract 编程语言
### 一个专门为初学者编写的简单编程语言
###### Designed by OpenLight LLC with heart

## 功能
- 解析和执行程序：支持类型化变量声明（int、double、string、bool）、数组、参数化函数、类和对象、控制流（if/else、while）、异常处理（try/catch/throw）、导入语句、窗口声明、内置函数（input、output）
- 编译为 x86-64 机器码：直接生成二进制代码

## 语法概述
```
program ::= import* (declaration | function | class)* main
statement ::= declaration | function | class | if expr { statement* } [else { statement* }] | while expr { statement* } | try { statement* } catch { statement* } | throw expr ; | id = expr ; | expr ; | function_call ;
expr ::= term ((+|-) term)*
term ::= factor ((*|/) factor)*
factor ::= number | string_literal | id | id.id | id(expr_list) | new id() | ( expr )
```

详细语法见 SYNTAX.md。

注释：// 行注释

详细语法请见 SYNTAX.md。

## 使用
编译器将程序编译为 x86-64 机器码并写入 output.bin，然后直接运行。

示例：
```
./target/release/dsubtract example.ds
```
生成机器码到 output.bin 并执行，打印退出码。

## 构建
```bash
cargo build --release
```
