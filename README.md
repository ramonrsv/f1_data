f1-dpf
------

f1-dpf is a collection of Rust crates for accessing historical F1 event data, predicting future event results, and managing fantasy teams.

Fantasy Game Rules
------------------

The rules and scoring system for the fantasy game are based on [The Official Formula 1 Fantasy Game](https://fantasy.formula1.com/en/game-rules). These are used to calculate scores and optimize team selection and in-season actions. While they are meant to replicate the official rules and therefore assist with playing the official game, this crate is in no way associated with _Formula 1®_ and there are no assurances of rule parity between the rule sets herein and those of the official fantasy game. Note that the rules may differ per season, so a specific rule set must be selected to be used for the calculations. The supported rule sets are:

[2023](f1-fantasy/docs/rules/2023.md)
