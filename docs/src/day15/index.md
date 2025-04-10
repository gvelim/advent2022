# Day 15: Beacon Exclusion Zone

Day 15 involves analyzing sensor coverage to find positions where beacons cannot be present.

## Problem Overview

You need to help locate a distress beacon in a cave system. There are several sensors that can detect the nearest beacon, and you need to use this information to:

1. Determine positions where a beacon cannot possibly be located
2. Find the one position in a specific area where the distress beacon must be located

The key aspects of this problem are:
- Each sensor reports its position and the position of the nearest beacon
- The distance between a sensor and its nearest beacon is calculated using Manhattan distance
- For Part 1, you need to count positions that cannot contain a beacon in a specific row
- For Part 2, you need to find the only possible position for the distress beacon in a large area

This problem tests your ability to work with ranges and coordinate systems efficiently.

## Navigation

- [Problem Description](./problem.md): Detailed description of the day's challenge
- [Solution Explanation](./solution.md): Walkthrough of the approach used
- [Code](./code.md): The complete implementation with comments