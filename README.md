# AIVD 2018 challenge 2
A quick and dirty prototype solver for puzzle 2 of the Dutch AIVD Christmas
puzzle challenge. [» Puzzle][puzzle]

## Output
Running the application in release mode solves the puzzle in about 3 seconds:
```bash
$ cargo run --release
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/aivd-sudoku`
Possibility:
KET|...|IS.
.SR|..T|OE.
.AN|SE.|RK.
---+---+---
.NK|IRE|STO
ET.|...|NRK
R.S|T.K|.A.
---+---+---
.KO|E.S|A.R
...|.IN|.OE
...|.AR|KNS

Possibility (filled):
KET|ROA|ISN
ISR|NKT|OEA
OAN|SEI|RKT
---+---+---
ANK|IRE|STO
ETI|ASO|NRK
ROS|TNK|EAI
---+---+---
NKO|ETS|AIR
SRA|KIN|TOE
TIE|OAR|KNS

Done
```

## License
This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information. 

[puzzle]: https://www.aivd.nl/onderwerpen/aivd-kerstpuzzel/documenten/publicaties/2018/12/11/aivd-kerstpuzzel-2018
