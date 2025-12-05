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

bool is_ingredient_fresh(vector<pair<string, string>> &ranges, string ingredient_string) {
  long ingredient = stol(ingredient_string);
  for (pair<string, string> range : ranges) {
    long left = stol(range.first);
    long right = stol(range.second);
    if (ingredient >= left && ingredient <= right) {
      return true;
    }
  }
  return false;
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

  vector<pair<string, string>> ranges;
  vector<string> ingredients;
  bool getting_ranges = true;
  for (string line : lines) {
    if (line == "") {
      getting_ranges = false;
      continue;
    }
    if (getting_ranges) {
      vector<string> range = split(line, "-");
      ranges.push_back({ range[0], range[1] });
    } else {
      ingredients.push_back(line);
    }
  }

  int n_fresh;
  for (string ingredient : ingredients) {
    if (is_ingredient_fresh(ranges, ingredient)) {
      n_fresh += 1;
    }
  }

  print(n_fresh);
}
