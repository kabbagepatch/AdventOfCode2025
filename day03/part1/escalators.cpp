#include <iostream>
#include <fstream>
#include <string>
#include <vector>
using namespace std;

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

  int joltage = 0;
  for (string bank : lines) {
    int largest = -1;
    int largest_ind = -1;
    for (int i = 0; i < bank.length() - 1; i += 1) {
      int battery = bank[i] - '0';
      if (largest < battery) {
        largest = battery;
        largest_ind = i;
      }
    }
    int second_largest = -1;
    for (int i = largest_ind + 1; i < bank.length(); i += 1) {
      int battery = bank[i] - '0';
      if (second_largest < battery) {
        second_largest = battery;
      }
    }
    joltage += largest * 10 + second_largest;
  }

  print(joltage);
}
