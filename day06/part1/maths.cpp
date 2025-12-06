#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>
using namespace std;

vector<string> split(string s, string delimiter) {
  vector<string> results;
  size_t pos = 0;
  string token;
  while ((pos = s.find(delimiter)) != string::npos) {
      token = s.substr(0, pos);
      if (!token.empty()) results.push_back(token);
      s.erase(0, pos + delimiter.length());
  }
  if (!s.empty()) results.push_back(s);
  return results;
}

bool isNumber(char c) {
  return c >= '0' && c <= '9';
}

void print(long stuff) {
  cout << stuff << endl;
}

void print(string stuff) {
  cout << stuff << endl;
}

void print(vector<string> stuff) {
  for (string t : stuff) {
    if (t.empty()) cout << "<blank>";
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

  long total = 0;
  vector<vector<string>> numbers;
  for (string line : lines) {
    numbers.push_back(split(line, " "));
  }
  vector<string> operators = numbers[numbers.size() - 1];
  numbers.pop_back();
  for (int i = 0; i < operators.size(); i += 1) {
    bool addition = operators[i] == "+";
    long result = addition ? 0 : 1;
    if (addition) {
      for (int n = 0; n < numbers.size(); n += 1) {
        result += stoi(numbers[n][i]);
      }
    } else {
      for (int n = 0; n < numbers.size(); n += 1) {
        result *= stoi(numbers[n][i]);
      }
    }
    total += result;
  }

  print(total);
}
