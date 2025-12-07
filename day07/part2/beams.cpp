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

void print(vector<vector<int>> stuff) {
  for (vector<int> t : stuff) {
    for (int a : t) {
      if (a == 0) {
        cout << " ";
      } else {
        cout << a;
      }
    }
    cout << endl;
  }
  cout << endl;
}


long get_n_paths(vector<string> &lines, int r, int c, vector<vector<long>> &path_counts) {
  if (path_counts[r][c] != -1) {
    return path_counts[r][c];
  }
  lines[r][c] = '|';

  if (r >= lines.size() - 1) {
    path_counts[r][c] = 1;
    return 1;
  }

  if (lines[r + 1][c] == '^') {
    path_counts[r][c] = get_n_paths(lines, r + 1, c - 1, path_counts) + get_n_paths(lines, r + 1, c + 1, path_counts);
    return path_counts[r][c];
  }

  path_counts[r][c] = get_n_paths(lines, r + 1, c, path_counts);
  return path_counts[r][c];
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

  int start_col = lines[0].find('S');
  vector<vector<long>> path_counts(lines.size(), vector<long>(lines[0].size(), -1));

  long n_paths = get_n_paths(lines, 0, start_col, path_counts);
  print(n_paths);
}
