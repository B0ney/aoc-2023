#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <array>

const std::string SYMBOLS = "!\"$%^&*()_+-=#~'@;:/?>,<`¬`|\\[]{}";

class Digit {
  public:
    int x;
    int y;
    int val;
    size_t digits;
    bool adjacent = false;
};

bool is_digit(char c) { return c >= '0' && c <= '9'; }

void scan(std::vector<Digit>& digits, const std::string& input, int y) {
    int start;
    bool currently_digit = false;
    std::string buffer = "";
    
    for (int x = 0; x <= input.size(); x++) {
        if (is_digit(input[x])) {
            if (!currently_digit) {
                currently_digit = true;
                start = x;
            }

        } else if (currently_digit) {
            digits.push_back({start, y, std::stoi(buffer), buffer.length()});
            currently_digit = false;
            buffer.clear();
        }

        if (currently_digit) {
            buffer.push_back(input[x]);
        }
    }
}

bool has_symbol(const std::vector<std::string>& data, int digits, int x, int y) {
    if (y >= data.size()) {
        return false;
    }

    if (x >= data[y].size()) {
        return false;
    }

    return SYMBOLS.find(data[y][x]) != std::string::npos;
}

bool check_adjaceny(const std::vector<std::string>& data, Digit& digit) {
    int len = digit.digits;
    int x = digit.x;
    int y = digit.y;

    std::vector<bool> arg;

    for (int l = 0; l < len; l++) {
        arg.push_back(has_symbol(data, len, x + l, y - 1)); // bottom
        arg.push_back(has_symbol(data, len, x + l, y + 1)); // top
    }

    arg.push_back(has_symbol(data, len, x - 1  , y - 1)); // top left corner
    arg.push_back(has_symbol(data, len, x + len, y - 1)); // top right corner
    arg.push_back(has_symbol(data, len, x - 1  , y + 1)); // bottom left corner
    arg.push_back(has_symbol(data, len, x + len, y + 1)); // bottom right corner

    arg.push_back(has_symbol(data, len, x - 1  , y));// middle left 
    arg.push_back(has_symbol(data, len, x + len, y));// middle right


    for (auto c: arg) {
        if (c) {
            return true;
        }
    }

    return false;
}

int main() {
    std::ifstream file("../input.txt");

    if (!file.is_open()) {
        std::printf("can't open file\n");
        return -1;
    }

    std::vector<std::string> data;
    std::vector<Digit> digits;
    std::string line;
    int y = 0;
       
    while (std::getline(file, line)) {
        scan(digits, line, y);
        data.push_back(line);
        y++;
    }

    int sum = 0;
    for (auto digit: digits) {
        if (check_adjaceny(data, digit)) {
            sum += digit.val;
        }
    }
    std::cout << sum <<'\n';
}
