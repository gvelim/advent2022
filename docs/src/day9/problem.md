# Day 9: Problem Description

## Rope Bridge

This rope bridge creaks as you walk along it. You aren't sure how old it is, or whether it can even support your weight. It seems to support the Elves just fine, though.

The bridge is a series of planks connected by rope. It doesn't have any guardrails, which is a bit concerning given how many Elves have already fallen into the river. You decide to distract yourself by modeling how the ropes move as you cross the bridge.

The bridge is made entirely of rope, with the poses connected end-to-end. The first pose is secured to a large tree, and the last pose is holding a big bag of supplies. A single rope connects each pose in the bridge. The ropes form a physical constraint: every rope segment wants to stay straight, and if a rope is pulled taught then the next rope segment in the chain will also be pulled in that direction (unless it's constrained in some other way).

The Elves want to know where the bag of supplies might end up. To simulate the ropes, you will need to keep track of the **head** (the first pose) and the **tail** (the last pose). If the head is ever two steps directly up, down, left, or right from the tail, the tail must move one step in that direction so it remains close enough. Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up.

You'll need to keep track of positions the tail visited at least once. For now, you should model the positions of the knots after each step. Then, you can count the positions the **tail** visited at least once.

For example:

```
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
```

This series of motions moves the head **right** four steps, then **up** four steps, then **left** three steps, then **down** one step, and so on. After each step, you'll need to update the position of the tail if the head and tail aren't touching. Visually, these motions occur as follows (s marks the starting position as a reference point):

```
== Initial State ==

......
......
......
......
..H...  (H covers T, s)

== R 4 ==

......
......
......
......
..TH..  (T covers s)

......
......
......
......
..TH..  (T covers s)

......
......
......
......
...TH.  (T covers s)

......
......
......
......
....TH  (T covers s)

== U 4 ==

......
......
......
....H.
....T.  (T covers s)

......
......
....H.
....T.
......  (T covers s)

......
....H.
....T.
......
......  (T covers s)

....H.
....T.
......
......
......  (T covers s)

== L 3 ==

...H..
....T.
......
......
......  (T covers s)

..H...
...T..
......
......
......  (T covers s)

.H....
..T...
......
......
......  (T covers s)

== D 1 ==

..H...
..T...
......
......
......  (T covers s)

== R 4 ==

...H..
..T...
......
......
......  (T covers s)

....H.
...T..
......
......
......  (T covers s)

.....H
....T.
......
......
......  (T covers s)

......
.....H
....T.
......
......  (T covers s)

== D 1 ==

......
......
.....H
....T.
......  (T covers s)

== L 5 ==

......
......
....H.
....T.
......  (T covers s)

......
......
...H..
....T.
......  (T covers s)

......
......
..H...
...T..
......  (T covers s)

......
......
.H....
..T...
......  (T covers s)

......
......
H.....
.T....
......  (T covers s)

== R 2 ==

......
......
.H....
.T....
......  (T covers s)

......
......
..H...
.T....
......  (T covers s)
```

After simulating the rope, you can count up all of the positions the **tail** visited at least once. In this diagram, s again marks the starting position (which the tail also visited) and # marks other positions the tail visited:

```
..##..
...##.
.####.
....#.
...s#.
```

So, there are **13** positions the tail visited at least once.

## Part 1

Simulate your complete hypothetical series of motions. **How many positions does the tail of the rope visit at least once?**

## Part 2

A rope snaps! Suddenly, the river is getting a lot closer than you remember. The bridge is still there, but some of the ropes that broke are now whipping toward you as you fall through the air!

The ropes are moving too quickly to grab; you only have a few seconds to choose how to arch your body to avoid being hit. Fortunately, your simulation can be extended to support longer ropes.

Rather than two knots, you now must simulate a rope consisting of **ten** knots. One knot is still the head of the rope and moves according to the series of motions. Each knot further down the rope follows the knot in front of it using the same rules as before.

Using the same series of motions as the above example, but with the knots marked H, 1, 2, ..., 9, the motions now occur as follows:

```
== Initial State ==

......
......
......
......
H.....  (H covers 1, 2, 3, 4, 5, 6, 7, 8, 9, s)

== R 4 ==

......
......
......
......
H123..  (9 covers s)

......
......
......
......
.H123.  (9 covers s)

......
......
......
......
..H123  (9 covers s)

......
......
......
......
...H123  (9 covers s)

== U 4 ==

......
......
......
....H..
....1..
....2..
....3..
....4..
....5..
....6..
....7..
....8..
....9..  (9 covers s)

== L 3 ==

// ... continued visualization omitted for brevity ...
```

Now, you need to keep track of the positions the new tail (knot 9) visits. In this example, the tail never moves far enough to leave a # in the visualized grid, but if you count the positions the tail visits at least once, you still get **1**. (You may want to try a different initial configuration to be sure.)

Let's try a larger example:

```
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
```

These motions cause the head of the rope to move around quite a bit. Here is an illustration of the positions of the head (H) and the tail (9) after each of the first few steps:

```
== Initial State ==
................
................
................
................
................
................
................
................
................
................
................
................
................
................
................
...............H  (H covers 1, 2, 3, 4, 5, 6, 7, 8, 9, s)

== R 5 ==
................
................
................
................
................
................
................
................
................
................
................
................
................
................
................
...............H  (H covers 1, 2, 3, 4, 5, 6, 7, 8, 9, s)

// ... continued visualization omitted for brevity ...
```

After simulating the rope, you can count up all of the positions the tail (knot 9) visited at least once. In this larger example, the tail visits **36** positions (including the position where it starts).

**Simulate your complete series of motions on a rope with ten knots. How many positions does the tail of the rope visit at least once?**