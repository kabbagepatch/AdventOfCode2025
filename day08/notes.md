I made this problem harder for me because I decided to use Rust entirely without touching C++. But I had a good idea for the solution from the start. Basically, I would first calculate all the distances and sort them so I have the closest boxes at hand (I used a 1D array with a key formed by using two indices to simplify the sort)

Then I would start with the circuits and circuit maps. The circuit maps for easy lookup for which circuit does a box belong to. And the circuits 2D vector as the vector of circuits. Initially all boxes are in their own circuits. As I iterate over the shortest distances, I combine the circuits together by moving from one circuit to the other and clearing the first (updating the circuit maps accordingly).

Then, for part 1, I sort the circuits by lengths and multiply the lengths of the first three

For part 2, instead of looping 1000 times, I let the loop go until one of the circuit contains all the boxes. I return the two boxes that were last to join and multiply their x coordinates
