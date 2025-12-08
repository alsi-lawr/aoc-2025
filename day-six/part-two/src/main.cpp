// aoc6pt2.cpp
// C++23 port of the given Rust solution.

#include <algorithm>
#include <cctype>
#include <cstdint>
#include <exception>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

using namespace std;

enum class Operation {
  Add,
  Multiply,
  Unknown,
};

Operation operation_from_string(string_view s) {
  auto is_space = [](unsigned char c) { return isspace(c) != 0; };

  auto begin = find_if_not(s.begin(), s.end(), is_space);
  auto rbegin = find_if_not(s.rbegin(), s.rend(), is_space);
  auto end = rbegin.base();

  if (begin >= end) {
    return Operation::Unknown;
  }

  if (distance(begin, end) == 1) {
    char c = *begin;
    if (c == '+')
      return Operation::Add;
    if (c == '*')
      return Operation::Multiply;
  }

  return Operation::Unknown;
}

Operation parse_operation_from_line(const vector<char> &op_line, size_t start,
                                    size_t end) {
  if (start >= end || start >= op_line.size()) {
    return Operation::Unknown;
  }

  end = min(end, op_line.size());
  string s;
  s.reserve(end - start);
  for (size_t i = start; i < end; ++i) {
    s.push_back(op_line[i]);
  }

  return operation_from_string(s);
}

struct Matrix2D {
  vector<vector<char>> data;

  Matrix2D transpose() const {
    if (data.empty()) {
      return {};
    }

    size_t rows = data.size();
    size_t cols = data[0].size();

    Matrix2D result;
    result.data.assign(cols, vector<char>(rows));

    for (size_t r = 0; r < rows; ++r) {
      for (size_t c = 0; c < cols; ++c) {
        result.data[c][r] = data[r][c];
      }
    }

    return result;
  }

  static Matrix2D from_strings(const vector<string> &lines) {
    Matrix2D m;
    m.data.reserve(lines.size());
    for (const auto &line : lines) {
      m.data.emplace_back(line.begin(), line.end());
    }
    return m;
  }

  bool column_contains_only(size_t column, char c) const {
    if (column >= data.size()) {
      return true;
    }
    for (char v : data[column]) {
      if (v != c)
        return false;
    }
    return true;
  }

  string column_string(size_t column, size_t rows) const {
    string s;
    if (column >= data.size())
      return s;
    const auto &col = data[column];
    rows = min(rows, col.size());
    s.reserve(rows);
    for (size_t r = 0; r < rows; ++r) {
      s.push_back(col[r]);
    }
    return s;
  }
};

struct Operand {
  uint64_t value{};

  static Operand parse_from_grid(const Matrix2D &grid, size_t num_rows,
                                 size_t col) {
    string s = grid.column_string(col, num_rows);

    auto is_space = [](unsigned char c) { return isspace(c) != 0; };
    auto begin = find_if_not(s.begin(), s.end(), is_space);
    auto rbegin = find_if_not(s.rbegin(), s.rend(), is_space);
    auto end = rbegin.base();

    if (begin >= end) {
      throw runtime_error("Empty operand column");
    }

    string trimmed(begin, end);
    uint64_t v = 0;
    try {
      v = stoull(trimmed);
    } catch (const exception &) {
      throw runtime_error("Failed to parse operand: '" + trimmed + "'");
    }

    return Operand{v};
  }
};

struct Operands {
  vector<Operand> values;

  static Operands parse_from_grid(const Matrix2D &grid, size_t num_rows,
                                  size_t start, size_t end) {
    Operands result;
    // columns [start, end) in reverse order
    for (size_t col = end; col-- > start;) {
      result.values.push_back(Operand::parse_from_grid(grid, num_rows, col));
    }
    return result;
  }

  uint64_t sum() const {
    uint64_t s = 0;
    for (const auto &o : values) {
      s += o.value;
    }
    return s;
  }

  uint64_t product() const {
    uint64_t p = 1;
    for (const auto &o : values) {
      p *= o.value;
    }
    return p;
  }
};

struct Problem {
  Operands operands;
  Operation op{Operation::Unknown};

  uint64_t answer() const {
    switch (op) {
    case Operation::Add:
      return operands.sum();
    case Operation::Multiply:
      return operands.product();
    case Operation::Unknown:
    default:
      return 0;
    }
  }
};

struct Problems {
  vector<Problem> items;

  uint64_t sum() const {
    uint64_t total = 0;
    for (const auto &p : items) {
      total += p.answer();
    }
    return total;
  }
};

Problems read_file(const string &file_path) {
  ifstream in(file_path);
  if (!in) {
    throw runtime_error("Failed to open file: " + file_path);
  }

  vector<string> lines;
  string line;
  while (getline(in, line)) {
    if (!line.empty()) {
      lines.push_back(line);
    }
  }

  if (lines.size() < 2) {
    throw runtime_error("Not enough lines in input");
  }

  size_t num_operands = lines.size() - 1;

  vector<string> operand_lines(
      lines.begin(), lines.begin() + static_cast<ptrdiff_t>(num_operands));
  Matrix2D grid = Matrix2D::from_strings(operand_lines).transpose();

  if (grid.data.empty()) {
    throw runtime_error("Empty grid");
  }

  size_t width = grid.data.size();

  vector<pair<size_t, size_t>> segments;
  bool in_seg = false;
  size_t seg_start = 0;

  for (size_t col = 0; col < width; ++col) {
    bool all_space = grid.column_contains_only(col, ' ');
    bool leaving_seg = in_seg && all_space;
    bool entering_seg = !in_seg && !all_space;

    if (leaving_seg) {
      in_seg = false;
      segments.emplace_back(seg_start, col);
    }
    if (entering_seg) {
      in_seg = true;
      seg_start = col;
    }
  }

  if (in_seg) {
    segments.emplace_back(seg_start, width);
  }

  vector<char> operator_row(lines[num_operands].begin(),
                            lines[num_operands].end());

  Problems problems;
  for (const auto &[start, end] : segments) {
    Operands ops = Operands::parse_from_grid(grid, num_operands, start, end);
    Operation op = parse_operation_from_line(operator_row, start, end);
    problems.items.push_back(Problem{move(ops), op});
  }

  return problems;
}

int main(int argc, char **argv) {
  if (argc < 2) {
    cerr << "usage: aoc6pt2 <input-file>\n";
    return 1;
  }

  try {
    Problems problems = read_file(argv[1]);
    uint64_t total = problems.sum();
    cout << "final = " << total << '\n';
    return 0;
  } catch (const exception &ex) {
    cerr << "Error: " << ex.what() << '\n';
    return 1;
  }
}
