What a problem. There's always one that takes hours. Especially one that has a deceptively easy part 1.

Part 1 was just looping through all the pairs of the points and finding the one that makes the biggest area

Part 2 was a lot more complicated. I needed to find the rectangle that's fully inside whatever shape the coordinates made

My first approach was just checking if all four corners, two from the input, two that form the rectangle, fall inside the polygon. I even came up with a lovely ray casting algorithm that checked that. But turns out that's the wrong approach because just because the corners fall in the polygon, that doesn't mean the whole rectangle will.

Then I had a few more approaches that involved checking line segment intersections with the rectangle, or checking if we ever go inside the rectangle with subsequent points. Had some very complex code to handle that but it still wasn't quite complete without a hundred edge cases

The final approach was a bit simpler. For each line segment, I just checked if exactly one of the edges lands inside the rectangle (implying an intersection since the other point is outside) or if the midpoint of the line segment falls inside (implying the line segment is partially inside but might have the ends on the edges)
