# Day 14: Problem Description

## Regolith Reservoir

The distress signal leads you to a giant waterfall! Actually, hang on - the signal seems like it's coming from the waterfall itself, and that doesn't make any sense. However, you do notice a little path that leads **behind** the waterfall.

Mineral formations of various kinds are dripping into small pools. The distress signal must be coming from somewhere in the cave behind the waterfall.

Your handheld device downloads a scan of the cave; this scan shows the shape of the cave walls. Your device reports that there's a kind of sand that slowly drops out of thin air and settles in the cave. When you see it in person, you confirm this - the source of the sand seems to be a point above the cave.

This scan is useful for detecting whether more sand will fall. Using your scan, simulate the falling sand. How many units of sand come to rest before sand starts flowing into the abyss below?

Sand is pouring into the cave from point `500,0`.

Drawing rock as `#`, air as `.`, and the source of the sand as `+`, this example looks like this:

```
  4     5  5
  9     0  0
  4     0  3
0 ......+...
1 ..........
2 ..........
3 ..........
4 ....#...##
5 ....#...#.
6 ..###...#.
7 ........#.
8 ........#.
9 #########.
```

Sand is produced **one unit at a time**, and the next unit of sand is not produced until the previous unit of sand comes to rest. A unit of sand is large enough to fill one tile of air in your scan.

A unit of sand always falls **down one step** if possible. If the tile immediately below is blocked (by rock or sand), the unit of sand attempts to instead move diagonally **one step down and to the left**. If that tile is blocked, the unit of sand attempts to instead move diagonally **one step down and to the right**. Sand keeps moving as long as it is able to do so, at each step trying to move down, then down-left, then down-right. If all three possible destinations are blocked, the unit of sand **comes to rest** and no longer moves, at which point the next unit of sand is created back at the source.

Using your scan, simulate the falling sand. **How many units of sand come to rest before sand starts flowing into the abyss below?**

## Example

In this example, the first unit of sand falls downward until it lands on the rock path at the bottom:

```
......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
......o.#.
#########.
```

The second unit of sand follows a slightly different path, falling to the right and then coming to rest:

```
......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
.....oo.#.
#########.
```

After a total of 5 units of sand come to rest, they form this pattern:

```
......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
......o.#.
....oooo#.
#########.
```

After a total of 22 units of sand fall:

```
......+...
..........
......o...
.....ooo..
....#ooo##
....#ooo#.
..###ooo#.
....oooo#.
...ooooo#.
#########.
```

After a total of 24 units of sand fall:

```
......+...
..........
......o...
.....ooo..
....#ooo##
...o#ooo#.
..###ooo#.
....oooo#.
.o.ooooo#.
#########.
```

Finally, using your scan, once a total of **24** units of sand come to rest, all further sand flows out the bottom, falling into the endless void. Just for fun, the path any new sand takes before falling forever is shown here with `~`:

```
.......+...
.......~...
......~o...
.....~ooo..
....~#ooo##
...~o#ooo#.
..~###ooo#.
..~..oooo#.
.~o.ooooo#.
~#########.
~..........
~..........
~..........
```

For Part 1, once all 24 units of sand shown above come to rest, all further sand flows out the bottom, falling into the endless void.

## Part 2

You realize you misunderstood the scan. There isn't an endless void at the bottom of the scan - there's floor, and you're standing on it!

You don't have time to scan the floor, so just assume the floor is an infinite horizontal line with a y coordinate equal to **two plus the highest y coordinate** of any point in your scan.

In the example above, the highest y coordinate of any point is 9, and so the floor is at y=11. (This is as if your scan contained one extra rock path from `0,11` to `500,11`.)

With the added floor, the sand spreads to the left and right, reaching a position of rest if it encounters a higher sand unit. Because of this, more sand is able to come to rest before the source is blocked.

Using your scan and assuming the floor is an infinite horizontal line with a y coordinate equal to two plus the highest y coordinate of any point in your scan, **how many units of sand come to rest?**

In the example from Part 1, after a total of 93 units of sand fall and come to rest, no more sand can come to rest. The source becomes blocked when sand unit 94 is produced:

```
............o............
...........ooo...........
..........ooooo..........
.........ooooooo.........
........oo#ooo##o........
.......ooo#ooo#ooo.......
......oo###ooo#oooo......
.....oooo.oooo#ooooo.....
....oooooooooo#oooooo....
...ooo#########ooooooo...
..ooooo.......ooooooooo..
#########################
```

For Part 2, using your scan, **how many units of sand come to rest** before the source of the sand becomes blocked?