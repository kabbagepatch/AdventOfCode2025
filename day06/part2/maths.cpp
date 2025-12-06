#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>
using namespace std;

void print(long stuff) {
  cout << stuff << endl;
}

void print(string stuff) {
  cout << stuff << endl;
}

void print(vector<string> stuff) {
  for (string t : stuff) {
    cout << t << endl;
  }
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

  int n_numbers = lines.size() - 1;
  long total = 0;
  string operator_line = lines[n_numbers];
  char cur_operator;
  vector<string> number_strings;
  for (int i = 0; i < operator_line.size(); i += 1) {
    if (operator_line[i] != ' ') {
      cur_operator = operator_line[i];
    }
    string number_string;
    for (int l = 0; l < n_numbers; l += 1) {
      if (lines[l][i] != ' ') {
        number_string.push_back(lines[l][i]);
      }
    }
    if (!number_string.empty()) {
      number_strings.push_back(number_string);
    }
    if (i + 1 >= operator_line.size() || number_string.empty()) {
      long result = cur_operator == '+' ? 0 : 1;
      for (int l = 0; l < number_strings.size(); l += 1) {
        if (cur_operator == '+') {
          result += stol(number_strings[l]);
        } else {
          result *= stol(number_strings[l]);
        }
      }
      total += result;
      number_strings.clear();
    }
  }

  print(total);
}
