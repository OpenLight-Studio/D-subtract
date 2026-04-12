# D-Subtract 编程语言语法文档

## 简介

D-Subtract 是一种为初学者设计的简单编程语言，由 OpenLight LLC 开发。它支持变量声明、数学运算、控制流和函数定义。D-Subtract 代码被编译为 x86-64 机器码并直接执行。

## 基本语法结构

D-Subtract 程序由一系列语句组成，每个语句以分号或块结束。

### 程序结构
```
program ::= import* (declaration | function | class)* main
```

### 语句类型
D-Subtract 支持以下语句：

1. **Import 语句**：`import <file>` 或 `import "file"`
2. **变量声明**：`type id [= init] [, id [= init]]* ;`
3. **函数定义**：`return_type id(param_list) { statement* }`
4. **类定义**：`class id { (type id; | return_type id(param_list) { statement* })* }`
5. **条件语句**：`if expr { statement* } [else { statement* }]`
6. **循环语句**：`while expr { statement* }`
7. **赋值语句**：`id = expr ;`
8. **表达式语句**：`expr ;`
9. **异常处理**：`try { statement* } catch { statement* }`
10. **抛出异常**：`throw expr ;`
11. **Window 声明**：`window [name] [icon] [theme] { statement* }`

### 表达式
表达式用于计算值，支持数学运算、变量引用、对象创建和方法调用。

```
expr ::= term ((+|-) term)*
term ::= factor ((*|/) factor)*
factor ::= number | string_literal | id | id.id | id(expr_list) | new id() | ( expr )
```

- **运算符优先级**：
  - 高：`*`, `/`
  - 中：`+`, `-`
  - 低：比较（如 `==`, `!=`, `<`, `<=`, `>`, `>=`）（条件使用非零为真）

## 数据类型

- **int**：64 位有符号整数，如 `5`, `10`
- **double**：64 位浮点数，如 `3.14`
- **string**：字符串，如 `"hello"`
- **bool**：布尔值，`true` 或 `false`

变量必须显式声明类型。支持数组：`type id[size] = {init_list}`

## 关键字

- `int`：整数类型
- `double`：浮点类型
- `string`：字符串类型
- `bool`：布尔类型
- `void`：无返回值类型
- `class`：类定义
- `new`：对象创建
- `this`：当前对象引用
- `if`：条件语句
- `else`：条件语句可选部分
- `while`：循环语句
- `try`：异常处理块
- `catch`：异常捕获块
- `throw`：抛出异常
- `window`：窗口声明
- `import`：导入文件

## 标识符

标识符由字母、数字、下划线组成，不能以数字开头。大小写敏感。

## 注释

D-- 支持行注释，以 `//` 开始，到行尾结束。

```d
// 这是一个注释
let x = 5  // 变量声明
```

## 示例程序

### 简单变量和运算
```
int x = 5;
double y = 3.14;
string s = "hello";
bool b = true;
int arr[5] = {1,2,3,4,5};
```

### 函数定义和调用
```
int sonProgram(int a){
    a = a + 1;
    return a;
}
void aaa(){
    s = input();
}
void main(){
    aaa();
    output(sonProgram(5), s);
}
```

### 类定义和对象
```
class MyClass {
    int value;
    void setValue(int v) {
        this.value = v;
    }
    int getValue() {
        return this.value;
    }
}
void main(){
    MyClass obj = new MyClass();
    obj.setValue(10);
    output(obj.getValue());
}
```

### 异常处理
```
void riskyFunction() {
    if (true) {
        throw "Error occurred";
    }
}
void main(){
    try {
        riskyFunction();
    } catch {
        output("Caught exception");
    }
}
```

### Window 声明
```
window [MainWindow] [icon.png] [dark] {
    // window applets
}
```

### Window 声明
```
window [MainWindow] [icon.png] [dark] {
    // window applets
}
```

### Import
```
import <math.dh>
import "/lib/io.dh"
```

## 语法规则细节

- **语句分隔**：语句以分号 `;` 结束。
- **块**：使用 `{}` 包围语句块。
- **作用域**：变量在声明后可见，函数内变量局部，类成员在对象中。
- **对象导向**：支持类定义、对象实例化和方法调用。
- **异常处理**：try块中抛出的异常可在catch块中处理。
- **错误处理**：语法错误会导致编译失败，运行时错误通过异常处理。
- **限制**：
  - 支持函数参数和返回值。
  - 支持字符串和数组。
  - 支持浮点运算。
  - 支持类和对象，但无继承。
  - 递归调用可能导致栈溢出。

## 扩展建议

未来可以添加：
- 继承和多态。
- 更多内置类型。
- 垃圾回收。
- 并发支持。

## 编译和运行

使用 Rust 编译器构建，然后运行生成的二进制文件。

```bash
cargo build --release
./target/release/dsubtract example.ds
```

更多详情见 README.md。