# D-Subtract 编程语言
### 一个专门为初学者编写的简单编程语言
###### Designed by OpenLight LLC with heart

## 功能
- 解析和执行程序：支持类型化变量声明（int、double、string、bool）、数组、参数化函数、类和对象、控制流（if/else、while、for）、异常处理（try/catch/throw）、导入语句、窗口声明、内置函数（input、output）
- 编译为 x86-64 机器码：直接生成二进制代码

## 语法概述
```
program ::= import* (declaration | function | class)* void main(){ statement* }
statement ::= declaration | function | class | if expr { statement* } [else { statement* }] | while expr { statement* } | for(type id=expr; expr; id+=expr) { statement* } | try { statement* } catch { statement* } | throw expr ; | id = expr ; | id += expr ; | expr ; | function_call ;
expr ::= relational ((+|-) relational)*
relational ::= term ((<|>|<=|>=|==|!=) term)*
term ::= factor ((*|/) factor)*
factor ::= number | string_literal | id | id.id | id(expr_list) | new id() | ( expr )
```

详细语法见 SYNTAX.md。

注释：// 行注释

## 语法示例

### Hello World
```
void main(){
    output<<"Hello, World!"<<endl;
}
```

### 变量和循环
```
import <dsub:libio>
void main(){
    int a;
    input<<"Please enter a number: ">>a;
    for(int i=1;i<=a;i++)a+=i;
    output<<"The sum is: "<<a<<endl;
}
```

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

## 支持的特性

### 数据类型
- int - 整数
- double - 浮点数
- string - 字符串
- bool - 布尔值
- void - 空类型

### 控制流
- if/else - 条件语句
- while - 循环
- for - for 循环

### 运算符
- 算术：+、-、*、/
- 关系：<、>、<=、>=、==、!=
- 赋值：=、+=

### 流操作
- << - 输出流操作符
- >> - 输入流操作符

### 内置函数
- input - 输入
- output - 输出
- endl - 换行