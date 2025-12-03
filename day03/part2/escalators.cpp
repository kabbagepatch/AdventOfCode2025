#include <iostream>
#include <fstream>
#include <string>
#include <vector>
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

  int joltage_length = 12;
  long total_joltage = 0;
  for (string bank : lines) {
    string joltage = "";
    int largest_so_far = -1;
    int largest_ind = -1;
    int prev_largest_ind = -1;
    for (int l = 0; l < joltage_length; l += 1) {
      for (int i = prev_largest_ind + 1; i < bank.length() - joltage_length + joltage.length() + 1; i += 1) {
        int battery = bank[i] - '0';
        if (largest_so_far < battery) {
          largest_so_far = battery;
          largest_ind = i;
        }
      }
      joltage.append(to_string(largest_so_far));
      prev_largest_ind = largest_ind;
      largest_so_far = -1;
      largest_ind = -1;
    }
    total_joltage += stol(joltage);
  }

  print(total_joltage);
}
