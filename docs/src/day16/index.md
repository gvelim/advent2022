# Day 16: Proboscidea Volcanium

Day 16 involves finding the optimal sequence for opening valves to release maximum pressure in a cave system.

## Problem Overview

You're trying to escape a volcano through a network of tunnels with pressure-release valves. Your goal is to maximize the pressure released before time runs out. Key aspects include:

1. Valves have different flow rates, and many have a flow rate of zero
2. Moving between valves takes 1 minute, and opening a valve takes 1 minute
3. For Part 1, you have 30 minutes to release as much pressure as possible
4. For Part 2, you work with an elephant for 26 minutes to release maximum pressure

This problem is essentially a pathfinding optimization problem where the goal is to find the sequence of valve openings that maximizes the total pressure released.

## Navigation

- [Problem Description](./problem.md): Detailed description of the day's challenge
- [Solution Explanation](./solution.md): Walkthrough of the approach used
- [Code](./code.md): The complete implementation with comments