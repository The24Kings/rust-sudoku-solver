# Rust based Sudoku Solving algoritm
Stack based solving algorithm with wave function collapse.

## Algorithm Breakdown *for my future reference*
- Start with a Vector with a size of 81 (our board)
- Each index will contain a 'State' which can be any number from 1-9
- Each 'State' will also have the active number
- I will utilize 'Wave Funtion Collapse' to propagate through each state<br/>
and solve the board, adding each "solution" to the Stack, if we run into an invlaid state<br/> 
*No Solution Possible* Pop items off the Stack until we find another cell with multiple<br/> 
possible states
- Cell-by-cell; Collapse->check->propagate->iterate->continue
- If check fails, pop until new solution is found.
