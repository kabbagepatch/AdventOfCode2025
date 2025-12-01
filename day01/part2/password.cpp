#include <iostream>
#include <fstream>
#include <string>
#include <vector>
using namespace std;

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

  int dial = 50;
  int n_zero = 0;
  for (string line : lines) {
    char dir = line[0];
    int number = stoi(line.substr(1));
    n_zero += number / 100;
    number %= 100;
    int prev_dial = dial;
    if (dir == 'L') {
      dial -= number;
      if (dial < 0) {
        dial += 100;
        if (dial != 0 && prev_dial != 0) n_zero += 1;
      }
    } else {
      dial += number;
      if (dial > 99) {
        dial -= 100;
        if (dial != 0 && prev_dial != 0) n_zero += 1;
      }
    }
    if (dial == 0) n_zero += 1;
  }

  cout << n_zero << endl;
}
