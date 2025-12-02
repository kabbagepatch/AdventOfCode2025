#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <cmath>
#include <algorithm>
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
  for (string range : ranges) {
    vector<string> rangeList = split(range, "-");
    string lower = rangeList[0];
    int lower_digit_count = lower.length();
    string upper = rangeList[1];
    int upper_digit_count = upper.length();

    if (lower_digit_count == upper_digit_count && lower_digit_count % 2 == 1) { // odd number of digits
      continue;
    }
    if (lower_digit_count != upper_digit_count) {
      if (lower_digit_count % 2 == 1) {
        string repeated_zeros(lower_digit_count, '0');
        lower = "1" + repeated_zeros;
        lower_digit_count += 1;
      } else if (upper_digit_count % 2 == 1) {
        upper_digit_count -= 1;
        string repeated_nines(upper_digit_count, '9');
        upper = repeated_nines;
      }
    }
    int lower_left_half = stoi(lower.substr(0, lower.length() / 2));
    int lower_right_half = stoi(lower.substr(lower.length() / 2));
    int upper_left_half = stoi(upper.substr(0, upper.length() / 2));
    int upper_right_half = stoi(upper.substr(upper.length() / 2));
    
    for (int i = lower_left_half; i <= upper_left_half; i += 1) {
      if (i == lower_left_half && lower_left_half < lower_right_half) {
        continue;
      }
      if (i == upper_left_half && upper_left_half > upper_right_half) {
        continue;
      }
      n_invalid += 1;
      invalid_sum += i * pow(10, (lower_digit_count / 2)) + i;
    }
  }

  cout << n_invalid << endl;
  cout << invalid_sum << endl;
}
