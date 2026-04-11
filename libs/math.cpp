#include <iostream>
#include <string>
#include <cctype>
#include <stdexcept>
#include <vector>
#include <cstdint>
#include <map>

// 【定义零件的种类】
// 想象我们在给乐高积木分类，每一类都有一个名字
enum class TokenType {
    NUMBER,         // 数字积木 (如 123)
    FLOAT,          // 浮点数
    IDENTIFIER,     // 标识符 (如 x)
    PLUS, MINUS,    // 加号、减号
    STAR, SLASH,    // 乘号(*)、除号(/)
    LPAREN, RPAREN, // 左括号(、右括号)
    LBRACE, RBRACE, // 左大括号{、右大括号}
    LET,            // let 关键字
    DEF,            // def
    IF, ELSE,       // if else
    WHILE,          // while
    ASSIGN,         // =
    END             // 结束标记 (表示没有积木了)
};

// 【定义零件的结构】
// 每个零件都有两个属性：它是什么类型(type)，以及如果是数字的话，它的值是多少(value)
struct Token {
    TokenType type;
    long value; // 只有当类型是 NUMBER 时，这个值才有用
    double fvalue; // 浮点数
    std::string lexeme; // 标识符名
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
    // 就像整理桌面一样，把没用的空格、制表符、换行符都跳过，直到遇到第一个有用的字符
    void skipWhitespace() {
        while (peek() == ' ' || peek() == '\t' || peek() == '\n' || peek() == '\r') {
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

        // 【第二步：处理注释】
        char c = peek();
        if (c == '/' && input[pos + 1] == '/') {
            consume(); consume(); // skip //
            while (peek() != '\n' && peek() != '\0') consume();
            return nextToken(); // recurse
        }

        // 【第三步：偷偷看一眼】
        c = peek();

        // 【第三步：检查是否读完了】
        // 如果 c 是 '\0'（代表字符串结束了），
        // 我们就返回一个“结束标记”(END)，告诉程序：“好啦，题目读完了！”
        if (c == '\0') return {TokenType::END, 0, 0.0, ""};

        // 【第四步：如果是数字，就把它整个读出来】
        // std::isdigit(c) 是在问：“当前这个字符是 0-9 的数字吗？”
        if (std::isdigit(c)) {
            std::string numStr;
            while (std::isdigit(peek()) || peek() == '.') {
                numStr += consume();
            }
            if (numStr.find('.') != std::string::npos) {
                return {TokenType::FLOAT, 0, std::stod(numStr), numStr};
            } else {
                return {TokenType::NUMBER, std::stoll(numStr), 0.0, numStr};
            }
        }

        // 【第五步：如果是标识符或关键字】
        if (std::isalpha(c)) {
            std::string lex;
            while (std::isalnum(peek())) {
                lex += consume();
            }
            if (lex == "let") return {TokenType::LET, 0, 0.0, lex};
            else if (lex == "if") return {TokenType::IF, 0, 0.0, lex};
            else if (lex == "else") return {TokenType::ELSE, 0, 0.0, lex};
            else if (lex == "while") return {TokenType::WHILE, 0, 0.0, lex};
            else if (lex == "def") return {TokenType::DEF, 0, 0.0, lex};
            else return {TokenType::IDENTIFIER, 0, 0.0, lex};
        }

        // 【第六步：如果是符号，直接拿走】
        // consume() 把这个符号正式“吃掉”（拿走），指针移到下一个位置。
        consume();

        // 【第七步：辨认符号身份】
        // 看看刚才拿走的那个字符 c 到底是什么符号，然后返回对应的“符号零件”。
        switch (c) {
            case '+': return {TokenType::PLUS, 0, 0.0, "+"};      // 是加号
            case '-': return {TokenType::MINUS, 0, 0.0, "-"};     // 是减号
            case '*': return {TokenType::STAR, 0, 0.0, "*"};      // 是乘号
            case '/': return {TokenType::SLASH, 0, 0.0, "/"};     // 是除号
            case '(': return {TokenType::LPAREN, 0, 0.0, "("};    // 是左括号
            case ')': return {TokenType::RPAREN, 0, 0.0, ")"};    // 是右括号
            case '{': return {TokenType::LBRACE, 0, 0.0, "{"};    // 左大括号
            case '}': return {TokenType::RBRACE, 0, 0.0, "}"};    // 右大括号
            case '=': return {TokenType::ASSIGN, 0, 0.0, "="};    // 赋值

            // 【第八步：出错处理】
            // 如果既不是数字，也不是上面这些符号（比如碰到了字母 'a' 或 '@'），
            // 那就出错了！程序会大声报错：“遇到了意想不到的字符！”
            default: throw std::runtime_error("Unexpected char");
        }
    }
};


class Parser {
    Lexer lexer;
    Token current; // 当前读到的那个“词”
    std::map<std::string, int> variables; // name to stack offset
    int stackSize = 0;
    int labelCount = 0;

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

    int allocateVar(const std::string& name) {
        if (variables.count(name)) throw std::runtime_error("Variable already declared: " + name);
        stackSize += 8;
        variables[name] = stackSize;
        return stackSize;
    }

    std::string newLabel() {
        return "L" + std::to_string(labelCount++);
    }



    // 对外接口：直接返回最终计算结果
    long evaluate() {
        return parseExpression();
    }

    // 生成汇编代码
    std::string generateCode() {
        asmCode.clear();
        variables.clear();
        stackSize = 0;
        // prologue
        emit("push rbp\n");
        emit("mov rbp, rsp\n");
        parseProgramForCode();
        if (stackSize > 0) {
            emit("sub rsp, " + std::to_string(stackSize) + "\n");
        }
        // epilogue
        emit("mov rsp, rbp\n");
        emit("pop rbp\n");
        emit("ret\n");
        return asmCode;
    }

private:
    std::vector<uint8_t> code;
    std::map<std::string, std::vector<size_t>> unresolved;
    std::map<std::string, size_t> labels;

    void emit(const std::string& s) {
        asmCode += s;
    }



    void emit64(uint64_t v) {
        emit32(v & 0xFFFFFFFF);
        emit32(v >> 32);
    }

    void emitMovRaxImm(long val) {
        emit(0x48); emit(0xB8); // mov rax, imm64
        emit64(val);
    }

    void emitPushRax() {
        emit(0x50); // push rax
    }

    void emitPopRbx() {
        emit(0x5B); // pop rbx
    }

    void emitPopRax() {
        emit("pop rax\n");
    }

    void emitMovRaxImm(long val) {
        emit("mov rax, " + std::to_string(val) + "\n");
    }

    void emitPushRax() {
        emit("push rax\n");
    }

    void emitPopRbx() {
        emit("pop rbx\n");
    }

    void emitAddRaxRbx() {
        emit("add rax, rbx\n");
    }

    void emitSubRaxRbx() {
        emit("sub rax, rbx\n");
    }

    void emitMulRbx() {
        emit("imul rbx\n");
    }

    void emitDivRbx() {
        emit("idiv rbx\n");
    }

    void emitMovRaxRbpOffset(int offset) {
        emit("mov rax, [rbp - " + std::to_string(offset) + "]\n");
    }

    void emitMovRbpOffsetRax(int offset) {
        emit("mov [rbp - " + std::to_string(offset) + "], rax\n");
    }

    void emitTestRax() {
        emit("test rax, rax\n");
    }

    void emitJne(const std::string& label) {
        emit("jne " + label + "\n");
    }

    void emitJe(const std::string& label) {
        emit("je " + label + "\n");
    }

    void emitJmp(const std::string& label) {
        emit("jmp " + label + "\n");
    }

    void emitCall(const std::string& label) {
        emit("call " + label + "\n");
    }

    void emitRet() {
        emit("ret\n");
    }

    void emitLabel(const std::string& label) {
        emit(label + ":\n");
    }

    void emitMovRaxRbpOffset(int offset) {
        emit(0x48); emit(0x8b); emit(0x85); emit32(-offset); // mov rax, [rbp - offset]
    }

    void emitMovRbpOffsetRax(int offset) {
        emit(0x48); emit(0x89); emit(0x85); emit32(-offset); // mov [rbp - offset], rax
    }

    // 解析程序
    void parseProgramForCode(TokenType endType = TokenType::END) {
        while (current.type != endType) {
            parseStatementForCode();
        }
    }

    // 解析语句
    void parseStatementForCode() {
        if (current.type == TokenType::LET) {
            eat(TokenType::LET);
            if (current.type != TokenType::IDENTIFIER) throw std::runtime_error("Expected identifier");
            std::string name = current.lexeme;
            eat(TokenType::IDENTIFIER);
            eat(TokenType::ASSIGN);
            parseExpressionForCode();
            int offset = allocateVar(name);
            emitMovRbpOffsetRax(offset);
        } else if (current.type == TokenType::IF) {
            parseIfForCode();
        } else if (current.type == TokenType::WHILE) {
            parseWhileForCode();
        } else {
            parseExpressionForCode();
            // 结果在 rax，可用于最后输出
        }
    }

    void parseIfForCode() {
        eat(TokenType::IF);
        std::string elseLabel = newLabel();
        std::string endLabel = newLabel();
        parseExpressionForCode();
        emitTestRax();
        emitJe(elseLabel);
        eat(TokenType::LBRACE);
        parseProgramForCode(TokenType::RBRACE);
        eat(TokenType::RBRACE);
        if (current.type == TokenType::ELSE) {
            eat(TokenType::ELSE);
            emitJmp(endLabel);
            emitLabel(elseLabel);
            eat(TokenType::LBRACE);
            parseProgramForCode(TokenType::RBRACE);
            eat(TokenType::RBRACE);
            emitLabel(endLabel);
        } else {
            emitLabel(elseLabel);
        }
    }

    void parseWhileForCode() {
        eat(TokenType::WHILE);
        std::string startLabel = newLabel();
        std::string endLabel = newLabel();
        emitLabel(startLabel);
        parseExpressionForCode();
        emitTestRax();
        emitJe(endLabel);
        eat(TokenType::LBRACE);
        parseProgramForCode(TokenType::RBRACE);
        eat(TokenType::RBRACE);
        emitJmp(startLabel);
        emitLabel(endLabel);
    }

    void emitAddRaxRbx() {
        emit(0x48); emit(0x01); emit(0xD8); // add rax, rbx
    }

    void emitSubRaxRbx() {
        emit(0x48); emit(0x29); emit(0xD8); // sub rax, rbx
    }

    void emitMulRbx() {
        emit(0x48); emit(0xF7); emit(0xEB); // imul rbx
    }

    void emitDivRbx() {
        emit(0x48); emit(0x99); // cqo
        emit(0x48); emit(0xF7); emit(0xFB); // idiv rbx
    }

    // 为代码生成修改的解析函数
    void parseExpressionForCode() {
        parseTermForCode();

        while (current.type == TokenType::PLUS || current.type == TokenType::MINUS) {
            TokenType op = current.type;
            eat(op);
            parseTermForCode();

            emitPopRbx(); // pop right
            emitPopRax(); // pop left
            if (op == TokenType::PLUS) {
                emitAddRaxRbx();
            } else {
                emitSubRaxRbx();
            }
            emitPushRax();
        }
    }

    void parseTermForCode() {
        parseFactorForCode();

        while (current.type == TokenType::STAR || current.type == TokenType::SLASH) {
            TokenType op = current.type;
            eat(op);
            parseFactorForCode();

            emitPopRbx();
            emitPopRax();
            if (op == TokenType::STAR) {
                emitMulRbx();
            } else {
                emitDivRbx();
            }
            emitPushRax();
        }
    }

    void parseFactorForCode() {
        if (current.type == TokenType::NUMBER) {
            long val = current.value;
            eat(TokenType::NUMBER);
            emitMovRaxImm(val);
            emitPushRax();
        } else if (current.type == TokenType::FLOAT) {
            throw std::runtime_error("Float not supported in code generation");
        } else if (current.type == TokenType::IDENTIFIER) {
            std::string name = current.lexeme;
            eat(TokenType::IDENTIFIER);
            if (variables.count(name) == 0) throw std::runtime_error("Undefined variable: " + name);
            int offset = variables[name];
            emitMovRaxRbpOffset(offset);
            emitPushRax();
        } else if (current.type == TokenType::LPAREN) {
            eat(TokenType::LPAREN);
            parseExpressionForCode();
            eat(TokenType::RPAREN);
        } else {
            throw std::runtime_error("Expected factor");
        }
    }
};
