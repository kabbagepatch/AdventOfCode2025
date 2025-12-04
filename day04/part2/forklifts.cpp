#include <iostream>
#include <fstream>
#include <string>
#include <vector>
using namespace std;

struct coords {
  int r;
  int c;
  bool operator==(const coords& other) const {
    return r == other.r && c == other.c;
  }
};

int get_neighbor_count(vector<string>& grid, coords current) {
  int max_r = grid.size() - 1;
  int max_c = grid[0].length() - 1;
  int neighbors = 0;
  if (current.r > 0) {
    if (current.c > 0 && grid[current.r - 1][current.c - 1] != '.') neighbors += 1;
    if (grid[current.r - 1][current.c] != '.') neighbors += 1;
    if (current.c < max_c && grid[current.r - 1][current.c + 1] != '.') neighbors += 1;
  }
  if (current.c > 0 && grid[current.r][current.c - 1] != '.') neighbors += 1;
  if (current.c < max_c && grid[current.r][current.c + 1] != '.') neighbors += 1;
  if (current.r < max_r) {
    if (current.c > 0 && grid[current.r + 1][current.c - 1] != '.') neighbors += 1;
    if (grid[current.r + 1][current.c] != '.') neighbors += 1;
    if (current.c < max_c && grid[current.r + 1][current.c + 1] != '.') neighbors += 1;
  }

  return neighbors;
}

void print(int stuff) {
  cout << stuff << endl;
}

void print(string stuff) {
  cout << stuff << endl;
}

void print(coords stuff) {
  cout << stuff.c << ", " << stuff.r << endl;
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

  int removed_rolls = 0;
  do {
    vector<coords> can_remove;
    for (int i = 0; i < lines.size(); i += 1) {
      for (int j = 0; j < lines[0].length(); j += 1) {
        if (lines[i][j] == '@') {
          int neighbors = get_neighbor_count(lines, {i, j});
          if (neighbors < 4) {
            can_remove.push_back({i, j});
          }
        }
      }
    }

    if (can_remove.empty()) break;

    removed_rolls += can_remove.size();
    for (coords roll : can_remove) {
      lines[roll.r][roll.c] = '.';
    }
  } while (removed_rolls != 0);

  print(removed_rolls);
}
