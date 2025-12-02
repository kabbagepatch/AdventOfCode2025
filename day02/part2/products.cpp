#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <set>
#include <cmath>
#include <algorithm>
#include <numeric>
using namespace std;

vector<string> split(string s, string delimiter) {
  vector<string> results;
  size_t pos = 0;
  string token;
  while ((pos = s.find(delimiter)) != string::npos) {
      token = s.substr(0, pos);
      results.push_back(token);
      s.erase(0, pos + delimiter.length());
  }
  results.push_back(s);
  return results;
}

bool isNumber(char c) {
  return c >= '0' && c <= '9';
}

void print(string stuff) {
  cout << stuff << endl;
}

void print(vector<string> stuff) {
  for (string t : stuff) {
    cout << t << endl;
  }
  cout << endl;
  cout << endl;
}

int main () {
  ifstream myfile;
  myfile.open ("../input");
  vector<string> lines;
  if (myfile.is_open()) {
    string line_string;
    while(getline(myfile, line_string)) {
      lines.push_back(line_string);
    }
  }
  myfile.close();

  int n_invalid = 0;
  long invalid_sum = 0;
  vector<string> ranges = split(lines[0], ",");
  vector<string> new_ranges;

  for (string range : ranges) {
    vector<string> range_list = split(range, "-");
    string lower = range_list[0];
    int lower_digit_count = lower.length();
    string upper = range_list[1];
    int upper_digit_count = upper.length();
    if (lower_digit_count == upper_digit_count) {
      new_ranges.push_back(range);
      continue;
    }

    string repeated_nines(lower_digit_count, '9');
    new_ranges.push_back(lower + "-" + repeated_nines);

    string repeated_zeros(upper_digit_count - 1, '0');
    new_ranges.push_back("1" + repeated_zeros + "-" + upper);
  }

  set<string> invalid_ids;
  for (string range : new_ranges) {
    vector<string> range_list = split(range, "-");
    string lower = range_list[0];
    string upper = range_list[1];
    int digit_count = lower.length();

    for (int repeat_count = 2; repeat_count <= digit_count; repeat_count += 1) {
      if (digit_count % repeat_count != 0) continue;

      int lower_components[repeat_count];
      int upper_components[repeat_count];
      int component_size = digit_count / repeat_count;
      for (int i = 0; i < repeat_count; i += 1) {
        lower_components[i] = stoi(lower.substr(i * component_size, component_size));
        upper_components[i] = stoi(upper.substr(i * component_size, component_size));
      }

      for (int c = lower_components[0]; c <= upper_components[0]; c += 1) {
        string id = to_string(c);
        bool valid = false;
        bool check_lower = true;
        bool check_upper = true;
        for (int i = 1; i < repeat_count; i++) {
          if (c == lower_components[0] && check_lower) {
            if (lower_components[i] > lower_components[0]) {
              valid = true;
            } else if (lower_components[i] < lower_components[0]) {
              check_lower = false;
            }
          }
          if (c == upper_components[0] && check_upper) {
            if (upper_components[i] < upper_components[0]) {
              valid = true;
            } else if (upper_components[i] > upper_components[0]) {
              check_upper = false;
            }
          }
          id += to_string(c);
        }
        if (valid) continue;

        invalid_ids.insert(id);
      }
    }
  }

  cout << invalid_ids.size() << endl;
  for (set<string>::iterator it = invalid_ids.begin(); it != invalid_ids.end(); ++it) {
    invalid_sum += stol(*it);
  }
  cout << invalid_sum << endl;
}
