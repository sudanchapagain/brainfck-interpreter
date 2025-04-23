#include <cstdint>
#include <fstream>
#include <iostream>
#include <iterator>
#include <stack>
#include <stdexcept>
#include <vector>

class Interpreter {
  /*
    SIZE is to emulate the memory model with cells.
    program vector holds the code.
    loop_stack to handle [ and ] loops.
    prog_ptr tracks current position in code
    data_ptr tracks current position in memory
  */
  static const size_t SIZE = 30000;
  std::vector<uint8_t> cells;
  std::vector<uint8_t> program;
  std::stack<size_t> loop_stack;
  size_t prog_ptr;
  size_t data_ptr;

 public:
  Interpreter(const std::vector<uint8_t>& program_code)
      : cells(SIZE, 0), program(program_code), prog_ptr(0), data_ptr(0) {}

  void run() {
    // https://gist.github.com/maxcountryman/1699708
    // https://gist.github.com/maxcountryman/1714336
    while (prog_ptr < program.size()) {
      char cmd = program[prog_ptr];
      switch (cmd) {
        case '>':
          ++data_ptr;
          break;
        case '<':
          --data_ptr;
          break;
        case '+':
          ++cells[data_ptr];
          break;
        case '-':
          --cells[data_ptr];
          break;
        case '.':
          std::cout << static_cast<char>(cells[data_ptr]);
          break;
        case ',':
          cells[data_ptr] = static_cast<uint8_t>(std::cin.get());
          break;
        case '[':
          if (cells[data_ptr] == 0) {
            int balance = 1;
            while (balance != 0 && ++prog_ptr < program.size()) {
              if (program[prog_ptr] == '[')
                ++balance;
              else if (program[prog_ptr] == ']')
                --balance;
            }
          } else {
            loop_stack.push(prog_ptr);
          }
          break;
        case ']':
          if (cells[data_ptr] != 0) {
            if (loop_stack.empty()) throw std::runtime_error("Unmatched ']'");
            prog_ptr = loop_stack.top();
          } else {
            if (!loop_stack.empty()) loop_stack.pop();
          }
          break;
        default:
          break;
      }
      ++prog_ptr;
    }
  }
};

std::vector<uint8_t> read_program(const std::string& filename) {
  std::ifstream file(filename.c_str(), std::ios::binary);
  if (!file) throw std::runtime_error("Could not open file: " + filename);
  return std::vector<uint8_t>(std::istreambuf_iterator<char>(file), {});
}

int main(int argc, char* argv[]) {
  if (argc != 2) {
    std::cerr << "Usage: bf <file>\n";
    return 1;
  }

  try {
    std::vector<uint8_t> program = read_program(argv[1]);
    Interpreter interpreter(program);
    interpreter.run();
  } catch (const std::exception& ex) {
    std::cerr << "Error: " << ex.what() << "\n";
    return 1;
  }

  return 0;
}
