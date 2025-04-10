# Day 11: Monkey in the Middle

Day 11 involves simulating monkeys playing a game of keep-away with items of various worry levels.

## Problem Overview

You need to model a group of monkeys passing items between them according to specific rules. Each monkey:

1. Has a list of items with worry levels
2. Inspects each item, applying an operation to update its worry level
3. Tests the worry level to decide which monkey to throw the item to
4. Keeps track of how many items it inspects

The challenge is to determine the level of "monkey business" (product of inspection counts of the most active monkeys) after a number of rounds.

## Navigation

- [Problem Description](./problem.md): Detailed description of the day's challenge
- [Solution Explanation](./solution.md): Walkthrough of the approach used
- [Code](./code.md): The complete implementation with comments