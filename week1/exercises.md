# TASK 1

In the created project, print a greeting to the user in the main.rs file.

Esimerkkiajo:
Hey, Rust!

---

# TASK 2

Create two immutable strings that store the values ​​"Rust" and "No".

Ask the user for input. If the user's input is Rust, the program prints Example run 1 statement.

If the input is "No", the program prints according to Example run 2.

Otherwise, print the user's value in the print statement, according to example run 3.

Use a formatted print statement with variables to print the values ​​of the variables.

Example runs:

Example run - Rust:

So you appreciate Rust? That's great! Thank you!

Example run - No:
So you like nothing? Alright then... :)

Example run your_own_value:

It seems that you like your_own_value.

---

# TASK 3

Using loops, asking the user for input and using if statements, make a program that adds numbers to the i16 size value according to the user's input until the number is larger than i16 can store. Use i32 for the calculation, but when the value exceeds the maximum value that i16 could hold, stop the program. If the user input is 0, exit the program. If the given value is lower than 0, give an error message to the user and continue the program.

Bolded and italicized sections are inputs given by the user.
Example run:
By how much do you want to increment the number?
Current: 0. Increment by:
9999
Current: 9999. Increment by:
9999
Current: 19998. Increment by:
9999
Current: 29997. Increment by:
9999
Enough incrementations.

---

# TASK 4

Make a function called receive_random() that generates random numbers 1-10.

Make a function called measure_luck() that takes the value of i32 as a parameter and returns a string. If the value of the parameter is greater than 3, return a String telling the user that he was unlucky. Otherwise, tell him he was lucky.

In the main function, make a loop, at the beginning of which add 1 to the counter variable. Generate a random number through the function you made.

If the generated value is between 1 and 3, print "Low..."

If the value is between 4 and 6, print "Middle!".

If the value is between 7 and 9, print High!!".

If the value is 10, print "Jackpot!!!" and end the loop.

Otherwise, print "Uncovered".

Add 1 to its counter variable every round.

After the loop, test your luck with the measure_luck() function, using the counter variable as a parameter.

Example run:

Middle!
High!!
Middle!
Low...
High!!
Low...
High!!
High!!
Jackpot!!!
You were UNLUCKY!

---

# TASK 5

Tee looppi, jossa joko hyökkäät pääviholliseen, saat uuden puolustus-monikerran tai käytät potionin (juoman).
Kunnes sinä - pelaaja, tai päävihollinen on tuhottu, looppi jatkuu.

Jos hyökkäät pääviholliseen, käytä luomaasi funktiota receive_player_attack_dmg.
Jos puolustat itseäsi, käytä luomaasi funktiota receive_defense_multiplier joka arpoo sinulle uuden puolustuskertoimen.
Jos käytät potionin ja parannat elämäpisteitäsi, käytä tähänkin omaa funktiotasi. Lisää 25 pistettä pelaajalle jos potioneita on vielä jäljellä.

Tee vuorollasi vain 1 ylläolevista vaihtoehdoista. Jos päävihollinen on vuorosi jälkeen vielä elossa, se hyökkää sinuun ja hyökkäyksen vahinkomäärä riippuu puolustuskertoimestasi.
Ennen taistelua (ohjelmasi päälooppia) alusta potionimääräsi kolmeen, pelaajan elämäpisteet 100:n, päävihollisen elämäpisteet 150:neen. Käytä float-numeroita elämäpisteille.

Tee funktio nimeltä receive_player_attack_dmg joka palauttaa satunnaisen luvun 12.5 ja 20 väliltä.
Tee funktio nimeltä receive_defense_multiplier joka generoi satunnaisluvun 2 ja 4 väliltä, ja palauttaa arvon joka on 1 / generoitu_satunnaisluku - eli palautettu luku on alle 1.
Tee funktio nimeltä receive_boss_attack_dmg joka palauttaa satunnaisen luvun väliltä 5 ja 25.

Satunnaislukujen generoimiseen käytä rand-cratea. Importtaa se Cargo.toml tiedostossa.

Esimerkkiajo käyttäjän syötteiden kanssa:
| Your HP - 100 | Boss HP - 150 |
| 1) Attack | 2) Defend | 3) Heal |
1
Your attack deals 14.957747 amount of damage.
You take 19.060558 damage.
| Your HP - 80.93944 | Boss HP - 135.04225 |
| 1) Attack | 2) Defend | 3) Heal |
2
Defense activated!
You take 4.502411 damage.
| Your HP - 76.43703 | Boss HP - 135.04225 |
| 1) Attack | 2) Defend | 3) Heal |
1
Your attack deals 16.456348 amount of damage.
You take 17.453207 damage.
| Your HP - 58.983818 | Boss HP - 118.58591 |
| 1) Attack | 2) Defend | 3) Heal |
3
You consume a potion.
You take 9.422489 damage.
| Your HP - 74.561325 | Boss HP - 118.58591 |
| 1) Attack | 2) Defend | 3) Heal |
1
Your attack deals 17.22312 amount of damage.
You take 23.554003 damage.
| Your HP - 51.007324 | Boss HP - 101.362785 |
| 1) Attack | 2) Defend | 3) Heal |
1
Your attack deals 18.885159 amount of damage.
You take 16.806934 damage.
| Your HP - 34.20039 | Boss HP - 82.47763 |
| 1) Attack | 2) Defend | 3) Heal |
1
Your attack deals 18.075796 amount of damage.
You take 14.347532 damage.
| Your HP - 19.852858 | Boss HP - 64.40183 |
| 1) Attack | 2) Defend | 3) Heal |
3
You consume a potion.
You take 24.755264 damage.
| Your HP - 20.097595 | Boss HP - 64.40183 |
| 1) Attack | 2) Defend | 3) Heal |
1
Your attack deals 18.169785 amount of damage.
You take 5.479629 damage.
| Your HP - 14.617966 | Boss HP - 46.232048 |
| 1) Attack | 2) Defend | 3) Heal |
1
Your attack deals 17.021484 amount of damage.
You take 19.016312 damage.
You have been defeated!
