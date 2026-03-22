#include "./libs/math.cpp"

// 【主函数入口】
// argc: 命令行参数的个数 (Argument Count)
// argv: 命令行参数的字符串数组 (Argument Vector)
int main(int argc, char* argv[]) {

    // 【参数校验】
    // 我们只需要 1 个额外参数（即数学表达式字符串）。
    // 加上程序名本身，argc 应该等于 2。
    // 如果不等于 2，说明用户没给表达式，或者给多了。
    if (argc != 2) {
        // cerr 是标准错误输出流，通常显示为红色，区别于 cout
        std::cerr << "Usage: " << argv[0] << " \"<expression>\"\n";
        std::cerr << "Example: " << argv[0] << " \"1 + 2 * 3\"\n";
        return 1; // 返回非 0 值表示程序异常退出
    }

    try {
        // 【构建解析器】
        // argv[1] 就是用户在命令行输入的那个字符串，比如 "12+3*(4-5)"
        // 构造函数内部会自动进行词法分析和语法分析
        Parser parser(argv[1]);

        // 【执行计算】
        // 调用 evaluate() 开始递归下降分析并返回最终结果
        long result = parser.evaluate();

        // 【输出结果】
        std::cout << result << std::endl;

    } catch (const std::exception& e) {
        // 【异常捕获】
        // 如果 Parser 内部遇到非法字符、括号不匹配或除零等错误，会 throw 异常。
        // 这里统一捕获并打印错误信息，防止程序直接崩溃（Core Dump）。
        std::cerr << "Error: " << e.what() << std::endl;
        return 1; // 出错时返回 1
    }

    return 0; // 正常结束返回 0
}
