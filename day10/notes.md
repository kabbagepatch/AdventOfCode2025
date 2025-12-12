It took me a few minutes of thinking but I was able to formulate part 1 as a BFS problem, with the states being the different light configurations and the paths being the four buttons you can press. It was fun implementing a non-standard BFS in Rust and then optimizing it further by using binary notaion for the lights and using bitmasks for the buttons. The only gotcha during this was my own fault for forgetting the "visited" check for the same state

Techinically the second problem is similar, except the state is the counters. But also turns out it was taking wayyyy too long to even get through one (got through the test input just fine, obviously). Too many possible states for a larger counter

After more thinking, I recognized part 2 was actually just a linear algebra problem. For example, for test input line one:

(3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

If A, B, C... F were the final button presses needed, we can formulate the problem as matrix multiplication
```
| A |   | 0 0 0 0 1 1 |   | 3 |
| B |   | 0 1 0 0 0 1 |   | 5 |
| C | x | 0 0 1 1 1 0 | = | 4 |
| D |   | 1 1 0 1 0 0 |   | 7 |
| E |
| F |  
```

That's about as far as I got. Initially I thought a matrix inverse could give me the answer but then I remembered only square matrices have inverses. But at least I have a system at the moment

The other possibilities could be a greedy algorithm or some sort of A* search that doesn't explore ALL paths like a BFS

-----

I finally solved Part 2. Mostly. Thanks to [this](https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/) explanation on reddit, I came up with a new algorithm for part 2. It involved parities and finding all the button combos and more recursion and caching. It got a bit more complicated than I wanted. It worked for about 99% of the inputs. Sadly, not a 100%. And after spending so many hours on it, I didn't want to spend anymore. 

I tried looking for solutons online to help debug mine and compare. Somehow, even the solution of the guy who wrote the explanation didn't work for me. In fact, his code (not_mine.py) gave me more wrong answers. Perhaps he just got lucky with his input. So I found another (not_mine.rs) one and finally got my final answer.
