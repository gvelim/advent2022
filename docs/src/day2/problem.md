# Day 2: Problem Description

## Rock Paper Scissors

The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an **encrypted strategy guide** (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your **total score** is the sum of your scores for each round. The score for a single round is the score for the **shape you selected** (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the **outcome of the round** (0 if you lost, 3 if the round was a draw, and 6 if you won).

For example, suppose you were given the following strategy guide:

```
A Y
B X
C Z
```

This strategy guide predicts and recommends the following:

- In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 for choosing Paper + 6 for winning).
- In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 for choosing Rock + 0 for losing).
- In the third round, your opponent will choose Scissors (C), and you should choose Scissors (Z). This ends in a draw with a score of 6 (3 for choosing Scissors + 3 for drawing).

So, in this example, if you were to follow the strategy guide, you would get a total score of **15** (8 + 1 + 6).

## Part 1

**What would your total score be if everything goes exactly according to your strategy guide?**

## Part 2

The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

Now, you need to figure out what shape to choose so the round ends as indicated. The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated.

For example, suppose you were given the same strategy guide:

```
A Y
B X
C Z
```

This strategy guide now predicts and recommends the following:

- In the first round, your opponent will choose Rock (A), and you need to end the round in a draw (Y), so you also choose Rock. This gives you a score of 4 (1 + 3).
- In the second round, your opponent will choose Paper (B), and you need to lose (X), so you choose Rock. This gives you a score of 1 (1 + 0).
- In the third round, your opponent will choose Scissors (C), and you need to win (Z), so you choose Rock. This gives you a score of 7 (1 + 6).

Following this new interpretation of the strategy guide, you would get a total score of **12** (4 + 1 + 7).

**Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?**