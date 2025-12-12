from itertools import combinations
from typing import List, Set, Dict, Tuple

def make_bcombos(bsets: List[Set[int]], machine_count: int) -> Dict[Tuple[int], Tuple[int, int]]:
    # Note: because of the insertion order 
    #  can assume that outp[x][0] >= outp[y][0] iff x > y
    bcombos = {}
    # This includes pressing no buttons as a combination:
    #  easy way to halve the problem during iteration
    for clen in range(len(bsets) + 1):
        for bcombo in combinations(range(len(bsets)), clen):
            joltage = tuple(sum(int(i in bsets[bn]) for bn in bcombo) for i in range(machine_count))
            if joltage not in bcombos:
                bcombos[joltage] = (clen, bitmask(joltage))
    return bcombos

def bitmask(joltage: Tuple[int]) -> int:
    return sum((j % 2) << i for i, j in enumerate(joltage))

def solve_indicator(bcombos: Dict[Tuple[int], int], indicator: Tuple[int]) -> int:
    bm = bitmask(indicator)
    for (clen, bm_) in bcombos.values():
        if not bm ^ bm_: return clen
    raise ValueError("Cannot find valid button combination")

class JoltageSolver:
    def __init__(self, bcombos: Dict[Tuple[int], int], joltage: Tuple[int]) -> "JoltageSolver":
        self.machine_count = len(joltage)
        self.bcombos = bcombos
        self._cache = {-1: 0}
        self._cap = sum(joltage)
        self.score = self._solve(joltage)

    def _solve(self, joltage: Tuple[int]) -> int:
        if joltage in self._cache: return self._cache[joltage]
        if max(joltage) == 0: return 0
        if min(joltage) < 0: return self._cap
        ret = self._cap
        jmask = bitmask(joltage)

        for joltdiff, (cost, bmask) in self.bcombos.items():
            # Pressing the buttons will leave us with two problem halves
            #   but only if parities match
            # Insight: If we can't solve half a problem, we can't solve the problem
            if jmask ^ bmask:
                continue
            joltage_ = tuple((j - i) // 2 for i, j in zip(joltdiff, joltage))
            ret = min(ret, cost + (2 * self._solve(tuple(joltage_))))
        self._cache[joltage] = ret
        return ret

ans, ans2 = 0, 0
file_path = 'src/input'
try:
    with open(file_path, 'r') as f:
        inp = f.read() # Read the entire file content into a single string
    # print(inp)
except FileNotFoundError:
    print(f"Error: The file '{file_path}' was not found.")
rows = inp.strip().split("\n")

for idx, row in enumerate(rows):
    # step_time = utils.timer()
    i_in, *b_in_lst, j_in = row.split(" ")
    joltage = tuple(map(int, j_in[1:-1].split(",")))
    indicator = tuple(int(c == "#") for c in i_in[1: -1])

    bsets = []
    for b_in in b_in_lst:
        blst = list(map(int, b_in[1:-1].split(",")))
        bsets.append(set(blst))
    bcombos = make_bcombos(bsets, len(joltage))

    t = solve_indicator(bcombos, indicator)
    t2 = JoltageSolver(bcombos, joltage).score
    # print(f"{idx + 1} / {len(rows)} took {(utils.timer() - step_time) * 1000:.3f}ms, results {t}, {t2}")
    ans, ans2 = ans + t, ans2 + t2

print(ans, ans2)
