#include "./libs/math.cpp"
#include <fstream>
#include <sstream>

// 【主函数入口】
// argc: 命令行参数的个数 (Argument Count)
// argv: 命令行参数的字符串数组 (Argument Vector)
int main(int argc, char* argv[]) {

    // 【参数校验】
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <code or .ds file>\n";
        std::cerr << "Example: " << argv[0] << " \"let x = 5\"\n";
        std::cerr << "Or: " << argv[0] << " program.ds\n";
        return 1;
    }

    std::string code;
    std::string input = argv[1];

    if (input.find(".ds") != std::string::npos) {
        // 从 .ds 文件读取
        std::ifstream file(input);
        if (!file) {
            std::cerr << "Error: Cannot open file " << input << std::endl;
            return 1;
        }
        std::stringstream buffer;
        buffer << file.rdbuf();
        code = buffer.str();
    } else {
        // 直接作为代码字符串
        code = input;
    }

    try {
        // 【构建解析器】
        // code 是用户输入的代码字符串
        Parser parser(code);

        // 【生成汇编代码】
        // 调用 generateCode() 生成计算表达式的汇编代码
        auto code = parser.generateCode();

        // 【写入汇编代码到文件】
        std::ofstream outfile("output.s", std::ios::out);
        outfile << code;
        outfile.close();

        std::cout << "汇编代码已生成并写入 output.s" << std::endl;

    } catch (const std::exception& e) {
        // 【异常捕获】
        // 如果 Parser 内部遇到非法字符、括号不匹配或除零等错误，会 throw 异常。
        // 这里统一捕获并打印错误信息，防止程序直接崩溃（Core Dump）。
        std::cerr << "Error: " << e.what() << std::endl;
        return 1; // 出错时返回 1
    }

    return 0; // 正常结束返回 0
}
