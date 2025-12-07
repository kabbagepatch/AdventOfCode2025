#include <iostream>
#include <fstream>
#include <string>
#include <vector>
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

void print(int stuff) {
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

  int n_splits = 0;
  for (int i = 1; i < lines.size(); i += 1) {
    for (int j = 0; j < lines[0].size(); j += 1) {
      char cur = lines[i][j];
      char above = lines[i - 1][j];
      if (cur == '.') {
        if (above == 'S' || above == '|') {
          lines[i][j] = '|';
        }
      } else if (cur == '^') {
        if (above == '|') {
          n_splits += 1;
          lines[i][j - 1] = '|';
          lines[i][j + 1] = '|';
        }
      }
    }
  }
  print(n_splits);
}
