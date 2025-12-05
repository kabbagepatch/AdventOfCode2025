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

int find_ingredient_range_index(vector<pair<long, long>> &ranges, long ingredient) {
  for (int i = 0; i < ranges.size(); i += 1) {
    pair<long, long> range = ranges[i];
    if (ingredient >= range.first && ingredient <= range.second) {
      return i;
    }
  }
  return -1;
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

  vector<pair<long, long>> ranges;
  for (string line : lines) {
    if (line == "") {
      break;
    }
    vector<string> endpoints = split(line, "-");
    long left = stol(endpoints[0]);
    long right = stol(endpoints[1]);
    int l = find_ingredient_range_index(ranges, left);
    int r = find_ingredient_range_index(ranges, right);
    if (l == -1 && r == -1) { // new range
      ranges.push_back({left, right});
    } else if (l == r) { // range fully contained within another
      continue;
    } else if (l == -1) { // only right side in an existing range
      ranges[r] = { left, ranges[r].second };
    } else if (r == -1) { // only left side in an existing range
      ranges[l] = { ranges[l].first, right };
    } else { // left and right are within different ranges
      ranges.push_back({ranges[l].first, ranges[r].second});
      if (l < r) {
        ranges.erase(ranges.begin() + r);
        ranges.erase(ranges.begin() + l);
      } else {
        ranges.erase(ranges.begin() + l);
        ranges.erase(ranges.begin() + r);
      }
    }
  }

  for (int i = 0; i < ranges.size(); i += 1) {
    pair<long, long> range = ranges[i];
    for (pair<long, long> range2 : ranges) {
      if (range.first > range2.first && range.second < range2.second) {
        ranges.erase(ranges.begin() + i);
        i -= 1;
      }
    }
  }

  long n_fresh = 0;
  for (pair<long, long> range : ranges) {
    n_fresh += range.second - range.first + 1;
  }
  print(n_fresh);
}
