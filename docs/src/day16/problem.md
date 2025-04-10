# Day 16: Problem Description

## Proboscidea Volcanium

The sensors have led you to the origin of the distress signal at the top of a large mountain. The mountain is made up of hot springs and waterfalls, but the terrain is otherwise treacherous and difficult to navigate.

As the expedition team begins climbing the mountain, you notice a trail of **steam** that ends at the entrance to a large cave. As you begin to make your way there, wolves with glowing red eyes begin circling you.

Just then, a sudden gust of freezing wind blows a small locket with a picture of you in front of you onto the ground. As you pick it up, you begin to hear echoing all around you — a distress message from the Elves about danger in the underground cave. You consider the wolves and begin broadcasting your own danger message on a frequency the wolves can't hear.

The distress message includes information about how the cave currently works in its present non-volcanic state. If you can calculate the potential pressure releases just in time, you might have a chance to stop the volcano from erupting.

You scan the cave for potential pressure release valves. Through your scan, you detect a network of pipes and valves. There's a pressure-release **valve** at each junction of pipes, a component that can be remotely operated over radio (your puzzle input). Each one of these valves has a flow rate: the number of pressure units it can release **per minute** (from 0, for a valve that can't be opened, to a reasonably large number).  You calculate how long you and an elephant could work together to move around the cave system, open valves, and release pressure.

To save time, you open the valves with non-zero flow rates. The rules for each of you moving and opening valves are:

- You start at valve `AA`.
- It takes you **1 minute** to move between valves.
- It takes you **1 minute** to open a valve.
- Moving and opening valves occurs in whole-number integer minutes.

To better plan your route, you note the flow rate of each valve from your scan. You're going to spend **30 minutes** opening valves to release as much pressure as possible.

For example, suppose you have the following scan output:

```
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
```

All of the valves begin **closed**. You start at valve `AA`, but it must be damaged or jammed or something: its flow rate is `0`, so there's no point in opening it. However, you could spend one minute moving to valve `BB` and another minute opening it; doing so would release pressure during the remaining **28 minutes** at a flow rate of `13`, a total eventual pressure release of `28 * 13 = 364`. Then, you could spend your remaining 26 minutes moving to and opening the remaining valves with positive flow rates (`CC`, `DD`, `EE`, `HH`, and `JJ`) to maximize pressure released (which would be 1707).

However, there's a more efficient approach. One way to maximize pressure is:

```
== Minute 1 ==
You open valve DD.
The elephant waits.

== Minute 2 ==
You move to valve CC.
The elephant moves to valve JJ.

== Minute 3 ==
You open valve CC.
The elephant opens valve JJ.

== Minute 4 ==
You move to valve BB.
The elephant waits.

== Minute 5 ==
You open valve BB.
The elephant moves to valve II.

== Minute 6 ==
You move to valve AA.
The elephant moves to valve AA.

== Minute 7 ==
You move to valve II.
The elephant moves to valve DD.

== Minute 8 ==
You move to valve JJ.
The elephant opens valve DD.

== Minute 9 ==
You open valve JJ.
The elephant moves to valve EE.

...
```

## Part 1

Work out the steps to release the most pressure in 30 minutes. **What is the most pressure you can release?**

## Part 2

You're worried that even with an optimal approach, the pressure released won't be enough. What if you got one of the elephants to help you?

It would take you 4 minutes to teach an elephant how to open the right valves in the right order, leaving you with only **26 minutes** to actually execute your plan. Would having two of you working together be better, even if it means having less time? (Assume the elephant is just as capable as you are at moving and opening valves.)

In the example above, you could teach the elephant your plan, which would take 4 minutes:

```
== Minute 1 ==
You move to valve II.
The elephant moves to valve DD.

== Minute 2 ==
You move to valve JJ.
The elephant opens valve DD.

== Minute 3 ==
You open valve JJ.
The elephant moves to valve EE.

== Minute 4 ==
You wait.
The elephant opens valve EE.

...
```

With the elephant helping, after opening valves `BB`, `CC`, `DD`, `EE`, `HH`, and `JJ`, you could achieve a flow rate of **`81`**.

However, you and the elephant need to be careful not to interfere with each other. As a result, you need to meticulously coordinate your actions to make sure that you and the elephant are never both trying to open the same valve, or move to the same valve.

With both you and the elephant working together for 26 minutes, **what is the most pressure you could release?**