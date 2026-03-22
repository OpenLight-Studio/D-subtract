#include <iostream>
#include <string>
#include <cctype>
#include <stdexcept>

// 【定义零件的种类】
// 想象我们在给乐高积木分类，每一类都有一个名字
enum class TokenType {
    NUMBER,         // 数字积木 (如 123)
    PLUS, MINUS,    // 加号、减号
    STAR, SLASH,    // 乘号(*)、除号(/)
    LPAREN, RPAREN, // 左括号(、右括号)
    END             // 结束标记 (表示没有积木了)
};

// 【定义零件的结构】
// 每个零件都有两个属性：它是什么类型(type)，以及如果是数字的话，它的值是多少(value)
struct Token {
    TokenType type;
    long value; // 只有当类型是 NUMBER 时，这个值才有用
};

// 【词法分析器类】
// 这个类就像一个“智能阅读器”，它负责拿着字符串，一个一个地把零件读出来
class Lexer {
    std::string input; // 存放我们要阅读的整行代码（比如 "12 + 3"）
    size_t pos = 0;    // 手指的位置，指向当前正在读的字符

public:
    // 构造函数：开始阅读前，先把字符串交给我们
    Lexer(const std::string& s) : input(s) {}

    // 【偷偷看一眼 (peek)】
    // 看看当前位置是什么字符，但不要把指针移走（不拿走字符）
    char peek() {
        if (pos >= input.size()) return '\0'; // 如果手指超出了字符串长度，返回结束符 '\0'
        return input[pos];                    // 否则返回当前的字符
    }

    // 【拿走字符 (consume)】
    // 把当前字符拿走，并且把手指移到下一个位置
    char consume() {
        return input[pos++]; // 返回当前字符，然后 pos 自动加 1
    }

    // 【跳过空格】
    // 就像整理桌面一样，把没用的空格和制表符都跳过，直到遇到第一个有用的字符
    void skipWhitespace() {
        while (peek() == ' ' || peek() == '\t') {
            consume(); // 吃掉空格
        }
    }

    // 【核心功能：获取下一个零件 (nextToken)】
    // 这个函数每次调用，都会从字符串里切出一个完整的意义单元（数字或符号）
    Token nextToken() {

        // 【第一步：清理桌面】
        // 有时候数字和符号之间会有空格，比如 "12 + 3"。
        // 我们不需要空格，所以先跳过它们。
        skipWhitespace();

        // 【第二步：偷偷看一眼】
        // 先用 peek() 看看当前字符是什么，存到变量 c 里，但先不拿走。
        char c = peek();

        // 【第三步：检查是否读完了】
        // 如果 c 是 '\0'（代表字符串结束了），
        // 我们就返回一个“结束标记”(END)，告诉程序：“好啦，题目读完了！”
        if (c == '\0') return {TokenType::END, 0};

        // 【第四步：如果是数字，就把它整个读出来】
        // std::isdigit(c) 是在问：“当前这个字符是 0-9 的数字吗？”
        if (std::isdigit(c)) {
            long num = 0; // 准备一个小桶，用来装我们读到的数字，初始值是 0

            // 【循环读数】
            // 只要接下来看到的字符还是数字，我们就一直读下去。
            // 比如看到 "123"，我们要把 1、2、3 连起来变成一个数，而不是分开。
            while (std::isdigit(peek())) {
                // 核心魔法公式：num = num * 10 + 新数字
                // 解释：
                // 1. num * 10：把桶里现有的数字扩大 10 倍（比如原来是 1，变成 10，给个位腾出位置）
                // 2. consume() - '0'：拿走当前字符，并把它从“字符”变成真正的“数字”。
                //    (在电脑里，字符 '0' 其实是个编号，减去 '0' 就能得到真正的数值 0)
                num = num * 10 + (consume() - '0');
            }

            // 读完整串数字后（比如读完了 "123"），返回一个“数字类型的零件”，里面装着 123。
            return {TokenType::NUMBER, num};
        }

        // 【第五步：如果是符号，直接拿走】
        // 如果不是数字，那肯定就是 + - * / ( ) 这些符号了。
        // consume() 把这个符号正式“吃掉”（拿走），指针移到下一个位置。
        consume();

        // 【第六步：辨认符号身份】
        // 看看刚才拿走的那个字符 c 到底是什么符号，然后返回对应的“符号零件”。
        switch (c) {
            case '+': return {TokenType::PLUS, 0};      // 是加号
            case '-': return {TokenType::MINUS, 0};     // 是减号
            case '*': return {TokenType::STAR, 0};      // 是乘号
            case '/': return {TokenType::SLASH, 0};     // 是除号
            case '(': return {TokenType::LPAREN, 0};    // 是左括号
            case ')': return {TokenType::RPAREN, 0};    // 是右括号

            // 【第七步：出错处理】
            // 如果既不是数字，也不是上面这些符号（比如碰到了字母 'a' 或 '@'），
            // 那就出错了！程序会大声报错：“遇到了意想不到的字符！”
            default: throw std::runtime_error("Unexpected char");
        }
    }
};


class Parser {
    Lexer lexer;
    Token current; // 当前读到的那个“词”

public:
    // 初始化：启动 Lexer，并先读第一个词放到 current 里
    Parser(const std::string& input) : lexer(input) {
        current = lexer.nextToken();
    }

    // 【核心工具：eat】
    // 作用：检查当前词是不是我想要的？
    // 是 -> 吃掉它，并读取下一个词放到 current；
    // 否 -> 报错（说明语法不对）。
    void eat(TokenType type) {
        if (current.type == type) {
            current = lexer.nextToken();
        } else {
            throw std::runtime_error("Parse error");
        }
    }

    // 【第二层：处理乘除 (Term)】
    // 逻辑：先算出一个“因子”(Factor)，然后看看后面有没有 '*' 或 '/'。
    // 如果有，就再算一个因子，立刻乘/除起来。循环直到没有乘除号。
    // 为什么单独写？因为乘除优先级比加减高，要先算完。
    long parseTerm() {
        long result = parseFactor(); // 1. 先拿到第一个数（或括号里的结果）

        // 2. 只要后面跟着 '*' 或 '/'，就一直循环算下去
        while (current.type == TokenType::STAR || current.type == TokenType::SLASH) {
            TokenType op = current.type; // 记下是乘还是除
            eat(op);                     // 吃掉这个符号
            long right = parseFactor();  // 去拿右边的数

            // 立刻计算，把结果存回 result
            if (op == TokenType::STAR) result *= right;
            else result /= right;
        }
        return result;
    }

    // 【最底层：处理数字和括号 (Factor)】
    // 逻辑：这是递归的尽头。只处理两种情况：
    // 1. 是个纯数字 -> 拿走，返回值。
    // 2. 是个左括号 '(' -> 说明里面藏着一个完整的表达式，调用 parseExpression 回去重新算一遍。
    long parseFactor() {
        if (current.type == TokenType::NUMBER) {
            long val = current.value;
            eat(TokenType::NUMBER); // 吃掉数字
            return val;
        }
        else if (current.type == TokenType::LPAREN) {
            eat(TokenType::LPAREN);       // 吃掉 '('
            long val = parseExpression(); // 【递归点】括号里可能很复杂，交给 expression 去处理
            eat(TokenType::RPAREN);       // 括号算完了，必须遇到 ')' 否则报错
            return val;
        }
        else {
            throw std::runtime_error("Expected factor");
        }
    }

    // 【第一层：处理加减 (Expression)】
    // 逻辑：先算出一个“项”(Term)（因为它可能包含乘除），然后看看后面有没有 '+' 或 '-'。
    // 如果有，再算一个 Term，立刻加/减起来。
    long parseExpression() {
        long result = parseTerm(); // 1. 先算出左边这部分（可能包含乘除）

        // 2. 只要后面跟着 '+' 或 '-'，就一直循环
        while (current.type == TokenType::PLUS || current.type == TokenType::MINUS) {
            TokenType op = current.type;
            eat(op);                // 吃掉符号
            long right = parseTerm(); // 去算右边的部分（注意：右边也可能是乘除混合，所以调用 parseTerm）

            if (op == TokenType::PLUS) result += right;
            else result -= right;
        }
        return result;
    }

    // 对外接口：直接返回最终计算结果
    long evaluate() {
        return parseExpression();
    }
};
