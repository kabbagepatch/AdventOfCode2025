I knew my part one solution wasn't optimal since I really should've made the final ranges taking the overlaps into account but I also knew part two was likely gonna force me to do it anyway, so I held off until then.

So for part one, I chose the simple solution to go over all the ranges for each ingredient until I find one where it lands and call it fresh.

Part two, like I predicted, told me to find total count of all the fresh ingredients possible. Thus, I needed to take the overlap into account. I divided it into multiple scenarios during the range iteration. A new range's endpoints could
- not fall anywhere in the existing ranges, thus add it
- fall entirely within a range, thus ignore it
- have only the left/right endpoints fall in an existing range, thus extending that existing range
- have left and right endpoints in two different ranges, thus combining them both into one

With this I build a new set of ranges with no overlap. The only scenario I missed in my first try was a range entirely falling within a range that I haven't encountered yet, since I go top to bottom. For that, I just made another for loop to iterate over and remove

And that was the solution
