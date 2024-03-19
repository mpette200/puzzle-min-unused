# Puzzle
## Smallest Value Not In The List
Find the smallest non-negative integer that is not in the list.

Constraints:
 * The list will only contain positive integers or zero.
 * The list could be in any order.

Examples:
 * [0, 1, 2, 3, 5] -> 4
 * [0, 1, 3, 4, 5] -> 2
 * [2, 1, 0] -> 3
 * [20, 10, 30] -> 0

Challenges:
 * Find a solution that runs in linear time.
 * Use functional programming techniques.
 * Measure computation time to confirm whether it is linear time.

## Linear Time Solution
Find groups of consecutive numbers, so in the example [0, 1, 2, 3, 5]
there are consecutive numbers from 0-3 and 5.

Can iterate through the numbers in any order and for each number store
the lower bound and upper bound of the current consecutive range.

In each iteration, the two adjacent numbers need to be checked, and
relevant lower and upper bounds need to be updated.

For example, iterating through the list in this order [3, 1, 2, 0, 5]:
* Insert 3
  * 3 -> (3,3)
* Insert 1
  * 1 -> (1,1)
  * 3 -> (3,3)
* Insert 2
  * 1 -> (1,3)
  * 2 -> midrange
  * 3 -> (1,3)
* Insert 0
  * 0 -> (0,3)
  * 1 -> midrange
  * 2 -> midrange
  * 3 -> (0,3)
* Insert 5
  * 0 -> (0,3)
  * 1 -> midrange
  * 2 -> midrange
  * 3 -> (0,3)
  * 5 -> (5,5)

Problems:
* Memory allocation - If using hashmap need to allocate memory larger than size of original list.
* Seems to be much slower than just sorting the list.
