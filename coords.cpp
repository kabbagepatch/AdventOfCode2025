#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <queue>
using namespace std;

struct coords {
  int r;
  int c;
  bool operator==(const coords& other) const {
    return r == other.r && c == other.c;
  }
};

void print(coords stuff) {
  cout << stuff.c << ", " << stuff.r << endl;
}

const vector<pair<int, int>> directions = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

bool is_valid_to_move(coords c, const vector<string> &maze) {
  return c.c >= 0 && c.r >= 0 && c.r < maze.size() && c.c < maze[0].size() && maze[c.r][c.c] == '.';
}

// BFS
vector<coords> get_shortest_path(vector<string> &maze, coords start, coords target) {
  int rows = maze.size();
  int cols = maze[0].size();
  vector<vector<int>> distances(rows, vector<int>(cols, INT16_MAX));
  vector<vector<coords>> parents(rows, vector<coords>(cols));

  queue<coords> q;
  q.push({start.r, start.c});
  distances[start.r][start.c] = 0;
  parents[start.r][start.c] = {-1, -1};

  while(!q.empty()) {
    coords current = q.front();
    q.pop();

    if (current == target) {
      break;
    }

    for (pair<int, int> d : directions) {
      coords neighbor = {current.c + d.first, current.r + d.second};

      if (is_valid_to_move(neighbor, maze) && distances[neighbor.r][neighbor.c] == INT16_MAX) {
        distances[neighbor.r][neighbor.c] = distances[current.r][current.c] + 1;
        parents[neighbor.r][neighbor.c] = current;
        q.push(neighbor);
      }
    }
  }
  
  vector<coords> shortest_path;

  if (distances[target.r][target.c] == INT16_MAX) {
    return shortest_path;
  }

  coords cur_parent = target;
  while (cur_parent.r != -1) {
    shortest_path.push_back(cur_parent);
    cur_parent = parents[cur_parent.r][cur_parent.c];
  }

  return shortest_path;
}

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

vector<coords> get_neighbors(vector<string>& grid, coords current) {
  int max_r = grid.size() - 1;
  int max_c = grid[0].length() - 1;
  vector<coords> neighbors;
  if (current.r > 0) {
    if (current.c - 1 > 0 && grid[current.r - 1][current.c - 1] != '.') neighbors.push_back({ current.r - 1, current.c - 1 });
    if (grid[current.r - 1][current.c] != '.') neighbors.push_back({ current.r - 1, current.c });
    if (current.c + 1 < max_c && grid[current.r - 1][current.c + 1] != '.') neighbors.push_back({ current.r - 1, current.c + 1 });
  }
  if (current.c - 1 > 0 && grid[current.r][current.c - 1] != '.') neighbors.push_back({ current.r, current.c - 1 });
  if (current.c + 1 < max_c && grid[current.r][current.c + 1] != '.') neighbors.push_back({ current.r, current.c + 1 });
  if (current.r < max_r) {
    if (current.c - 1 > 0 && grid[current.r + 1][current.c - 1] != '.') neighbors.push_back({ current.r + 1, current.c - 1 });
    if (grid[current.r + 1][current.c] != '.') neighbors.push_back({ current.r + 1, current.c });
    if (current.c + 1 < max_c && grid[current.r + 1][current.c + 1] != '.') neighbors.push_back({ current.r + 1, current.c + 1 });
  }

  return neighbors;
}
