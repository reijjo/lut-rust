Tehtävä 1
Tässä tehtävässä teet yksinkertaisen komentotulkin. Tee funktio nimeltä "read_file()", joka lukee tällä sivulla annetun "read.txt" -tiedoston merkkijonoon ja tulostaa sen tulosteen konsoliin. Tee funktio nimeltä "prank_user()", jossa tulostat "You have receved an email". Kumpikaan funktioista ei ota ulkoisia parametreja.

Tee silmukka pääfunktioon, joka toimii komentotulkkina ja ottaa käyttäjän syötteen. Jos käyttäjä syöttää "read", käytä read_file-toimintoa. Jos käyttäjä syöttää "prank", kutsu prank_user-funktiota. Muussa tapauksessa tulosta "Invalid command. Try again."

Esimerkkiajo:

Tehtävä 2
Tee ohjelma, joka annettujen komentoriviparametrien perusteella joko lukee tietyn tiedoston tai kirjoittaa tiettyyn tiedostoon. Parametrien tulee sisältää tiedoston nimi, suoritettavat toimet (kirjoitus/luku) ja mahdollisesti toinen tiedosto, jolla on ensimmäiseen tiedostoon kirjoitettava sisältö. Nimeä funktiot write_in_file ja read_file. Ne ottavat vastaavasti 2 ja 1 parametria.

Huomaa, että writeln!-komento luo ylimääräisen rivin kirjoitettuun tiedostoon. Esimerkiksi write!-komennolla saa käyttäjän syötteen sellaisenaan kirjoitettua tiedostoon.

Huomaa, että ohjelma pitää ajaa esimerkiksi cargo run komennolla rust-analyzer työkalun Run-napin sijasta, sillä sinun ohjelma käyttää komentoriviargumentteja.

Esimerkkiajo kirjoitus:

cargo run uusi_tiedosto.txt write "testi lisäys"

Esimerkkiajo luku:

cargo run uusi_tiedosto.txt read

The contents of the file:

testi lisäys

Esimerkkiajo ilman komentoja:

cargo run

No arguments were given.

Tehtävä 3
Tässä tehtävässä teemme funktion, joka käyttää säikeitä (threads). Tietyn ajan kuluttua käyttäjän on painettava näppäintä f lyhyen ajan sisällä. Jos käyttäjä painaa oikeaa näppäintä, kun vielä on aikaa, hän voittaa kaksintaistelun. Väärän näppäimen painaminen johtaa siihen, että käyttäjä jää väliin, ja ohjelman on odotettava kunnes vihollinen tekee vuoronsa. Jos ei ammuta ajoissa, vastustaja ampuu ensin. Kaksintaistelu alkaa 5 sekunnin päästä. Kun 3 sekuntia on kulunut, vastustaja voittaa pelin.

Tee funktio nimeltä start_duel(), joka käynnistää taistelun ja säikeet sekä muut toiminnot, joita tarvitset ohjelman tekemiseen.

Esimerkkiajo, käyttäjä ei tee mitään:
FIRE!!!
Opponent shoots first!

Esimerkkiajo, käyttäjä syöttää f kirjaimen oikeassa kohtaa:

FIRE!!!

f

You fire first!

Tehtävä 4
Jatketaan säikeillä. Käyttäjällä on joko 100 000 dollarin tai 1 000 000 dollarin arvosta rahaa, ja hänen on estettävä kaksi varkautta varastamasta. Ohjelma korostaa tiedonsiirtoa säikeiden välillä. Siinä on kolme säiettä, joista kaksi edustaa varkaita. Viimeinen on valmis vastaanottamaan syötteen käyttäjältä, jolloin hän voi pysäyttää rikolliset ennen kuin lisää rahaa menetetään. Lopputuloksena on, että varkaat pääsivät karkuun ottamansa rahan kanssa ennen kuin kirjoitit "catch" tai menetät kaikki rahasi ennen kuin ehdit kirjoittaa "catch".

Tee funktio nimeltä create_threads(), joka luo säikeen per varas, ja yksi säie, jossa käyttäjä saa kiinni varkaat ja joka looppaa kunnes käyttäjä saa kirjoitettua "catch" tai rahat ovat loppu. Funktio ottaa parametrina luvun, ei referenssiä.

Varkaiden säikeiden sleep-aika on 5 sekuntia varkaalle, joka vie 10 000 dollaria, ja 3 sekuntia toiselle varkaalle, joka vie sinulta 35 000 dollaria.

Esimerkkiajo, miljoona dollaria:

Do you have a million dollars? | y = yes, n = no

y

All right then, millionaire.

ALERT!!! Someone stole $35,000 from you!

Funds left: 965000

ALERT!!! Someone stole $10,000 from you!

Funds left: 955000

catch

The thieves have left.

Esimerkkiajo, ei miljoonaa dollaria:

Do you have a million dollars? | y = yes, n = no

n

Let's just assume you have $100,000 then.

ALERT!!! Someone stole $35,000 from you!

Funds left: 65000

ALERT!!! Someone stole $10,000 from you!

Funds left: 55000

ALERT!!! Someone stole $35,000 from you!

Funds left: 20000

catch

The thieves have left.

Tehtävä 5
Tämä harjoitus sisältää lähes kaikki tässä luvussa käsitellyt aiheet. Ohjelma sisältää komentotulkin, vaatii komentoriviargumentteja ja sisältää säikeitä. Argumentit määrittelevät mitä kirjainta (arg 1) on painettava mahdollisimman monta annetussa sekuntimäärässä (arg 2). Yksi säikeistä laskee kuinka aika kuluu. Toinen vastaanottaa syötteitä käyttäjältä. Pääsäie on tilassa, jossa se käsittelee näistä kahdesta säikeestä saatua dataa. Tulosta lopuksi, kuinka monta painallusta käyttäjä on tehnyt ja minkä näppäimen käyttäjä valitsi.
Huomaa, että ohjelma pitää ajaa esimerkiksi cargo run komennolla rust-analyzer työkalun Run-napin sijasta, sillä sinun ohjelma käyttää komentoriviargumentteja.

Esimerkkiajo:
cargo run f 10

Do you want to start or exit?
$ start
f
Presses: 1
f
Presses: 2
ff
f
Presses: 3
f
Presses: 4
f
Presses: 5
f
Presses: 6
f
Presses: 7
f
Presses: 8
f
Presses: 9
f
Presses: 10
f
Presses: 11
f
Presses: 12
f
Presses: 13
f
Presses: 14
f
Presses: 15
f
Presses: 16
f
Presses: 17
ff
f
ff
ff
f
Presses: 19
f
Presses: 20
f
Presses: 21
f
Presses: 22
f
Presses: 23
f
Presses: 24
f
Presses: 25
f
Presses: 26
f
Presses: 27
f
Presses: 28
f
Presses: 29
f
Presses: 30
f
Presses: 31
f
Presses: 32
f
Presses: 33
f
Presses: 34
fYou have managed to press 'f' 34 times.

Esimerkkiajo 2:

cargo run f 10

Do you want to start or exit?
$ exit
Last modified: Thursday, 13 February 2025, 12:46 PM
