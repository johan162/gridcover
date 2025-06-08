# Overview

This is a program to simulate the coverage of a moving circle across a grid. The aim of the program is to mark all squares in the grid that is completely covered by the circle as it moves across the grid at a given velocity and drection. 

## Rules

1. The radius of the circle can be specified by a command line argument. Default is 40 units
2. A square is either 100% covered by the circle or not. A partial covered square is not covered.
3. The grid size is given as N x M and specified by a command line option, default is 100 x 100 squares
4. The bottom left square is (0,0)
5. The size of each grid square is specified by a command line option, default is 10 units
6. The circle starts at coordinates for its center and given by command line option, default is (0,0)
7. The coordinate of the circle is interpretated as the center of the square at the same grid-coordinates
8. The circle moves at velocity v units/second given as command line option, defaults to 0.5 units/s
9. The direction of the circle movement is give as a vector (x,y) on the command line and defaults to (1,1)
10. Each iteration simulates a movement 0f 0.1 units of the circle
11. The simulation ends once the circle has moved across the grid
12. After ths simulation finishes the grid is then printed with a '-' for a non covered square and '*' for a covered square and the time it would have taken the circle to move across (given its velocity is also printed)

