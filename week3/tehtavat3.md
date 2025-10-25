Tehtävä 1
Listaa kaikki yllä näkyvät mahdolliset maat, pyydä käyttäjää syöttämään tietoja ja tulosta maan tiedot. Tee moduuli nimeltä "game", jossa on Country.rs-tiedosto ja mod.rs-tiedosto. Tee Country.rs-tiedosto julkiseksi mod.rs:ssä. Luo Country.rs:ssä rakenne nimeltään Country, jolla on attribuutit nimi, väestö, armeijan_koko, valloitetut_maat merkkijonovektorina ja is_conquered Boolen tyyppi. Käytä numeroattribuuteille i64:ää, merkkijonoille String-tyyppiä, ja jonoille vektoria String-tyypillä.

Luo structiin metodit new, get_name(), get_population() ja get_army_size. get-funktiot palauttavat vastaavat attribuuttiarvonsa muuttumattomina viitteinä. new-funktio tekee näihin kenttiin Country:n instanssin annetuilla arvoilla, ja palauttaa tämän instanssin self-avulla.

Main-funktiossa importtaa Country-tyyppi ja luo käyttäjän syötteen perusteella uusi Country alla olevilla tiedoilla ja tulosta maa, väestö ja armeijan koko esimerkkiajon mukaisesti.

Esimerkkiajo Finland:
| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |
Choose your country:
1
Country: Finland
Population: 5600000
Army size: 900000

Esimerkkiajo Sweden:
| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |
Choose your country:
2
Country: Sweden
Population: 10000000
Army size: 200000

Tehtävä 2

Lisää Player.rs-tiedosto game-kansioon. Lisää "Player"-niminen rakenne ja toteuta Player-luokalle metodit new, inspect ja get_country. Player-rakenteessa on pelaajan valitsema Country, ja funktio new instantoi Player-objektin annetulla maalla. Tarkastustoiminto (inspect funktio) tulostaa samat tiedot kuin aiemmin, mutta lisättynä tulostusriveillä. Voit käyttää tulostusta esimerkiksi alla olevalla tavalla:
println!("Name: {}", self.country.get_name())

get_country-funktio palauttaa muuttuvan viittauksen pelaajan valitsemaan maahan.

Main-funktiossa pyydä käyttäjää valitsemaan maansa. Tee valinnan perusteella uusi Player-instanssi, jolla on valittu maa Country-luokan instanssina ja samat maatiedot kuin aiemmin.

Kysy käyttäjältä, haluaako hän tarkastaa kansakuntansa. Käytä Playerin tarkistustoimintoa, jos käyttäjä syöttää "y", ja jos käyttäjä syöttää "n", tulosta esimerkkiajon mukaisesti.

Esimerkkiajo Finland:
| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |
Choose your country:
1
| Inspection on your own nation? | y = yes | n = no |
y
An inspection has been completed..
Country information:
Name: Finland
Population: 5600000
Army size: 900000

Esimerkkiajo Norway, ei inspectionia:
| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |
Choose your country:
3
| Inspection on your own nation? | y = yes | n = no |
n
The leader is confident. No inspection needed.

Tehtävä 3

Tarkastusosan jälkeen käyttäjä voi vakoilla maata. Et voi vakoilla omaa kansakuntaasi. Jos vakoilet toista kansakuntaa, tulosta maan tiedot samalla tavalla kuin ennenkin. Vakoillessa tulostat vakoiltavan maan tiedot.

Luo vakoilutoiminto metodina Player.rs tiedoston Player-luokassa.

Muokkaa main.rs-tiedostoa loopiksi, kunnes Player katkaisee loopin syöttämällä 0 . Pyydä loopissa haluaako käyttäjä tehdä tarkastuksen (inspection) ja sitten kysy haluaako hän vakoilla. Anna vaihtoehto ohjelmasta poistumiseen vakoiluosassa.

Muokkaa alkuperäistä toteusta modulaarisemmaksi. Lisää GameMap.rs tiedosto game-kansioon. Tee tässä tiedostossa GameMap-niminen rakenne, jonka attribuutti on maat, jonka tyyppi Country-vektori. Toteuta GameMapiin funktiot new, list_countries ja get_country_by_index -funktiot. Funktiossa new luo kaikki neljä maata tietoineen Country-tyyppinä ja tallenna ne GameMap-structin country-attribuutille. list_countries -funktio listaa kaikki maat ja niiden indeksit:
"1) Finland" jne.

get_country_by_index -funktio palauttaa muuttuvan maatyypin annetulle maalle GameMap-määritteen maissa.

Implementoi main.rs:n alussa käyttöön GameMap-rakenne ja lisää siihen kaikki maat. Modulaarisuuden vuoksi käytä tätä toimintoa, kun on tarpeen tulostaa käyttäjälle maiden tiedot vastaavien indeksien kanssa.

Muista lisätä tarvittavat lisäykset mod.rs tiedostoon game-kansiossa.

Muista lisätä tarvittavat lisäykset pelikansion mod.rs-tiedostoon.

Esimerkkiajo:

| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |
Choose your country:
4
| Inspection on your own nation? | y = yes | n = no |
y
An inspection has been completed..
Country information:
Name: Denmark
Population: 6000000
Army size: 50000
| 1) Spy on a country | 0) Exit program |
1

1. Denmark
2. Finland
3. Norway
4. Sweden
   1
   You can't spy on your own nation!
   | Inspection on your own nation? | y = yes | n = no |
   y
   An inspection has been completed..
   Country information:
   Name: Denmark
   Population: 6000000
   Army size: 50000
   | 1) Spy on a country | 0) Exit program |
   1
5. Denmark
6. Finland
7. Norway
8. Sweden
   3
   Espionage successful.
   Country information:
   Name: Norway
   Population: 5500000
   Army size: 100000
   | Inspection on your own nation? | y = yes | n = no |
   n
   The leader is confident. No inspection needed.
   | 1) Spy on a country | 0) Exit program |
   9
   Invalid game input. Try again.
   | Inspection on your own nation? | y = yes | n = no |
   n
   The leader is confident. No inspection needed.
   | 1) Spy on a country | 0) Exit program |
   0

Tehtävä 4

Tätä tehtävää varten sinun on luotava uusia metodeja Country.rs:lle:
get_conquered_nations()

get_is_conquered()

set_population()

set_army_size()

set_conquered_nations()

set_is_conquered()

get_conquered_nations() funktio palauttaa maan (Country) valloitetut kansakunnat -luettelon. Muut get-funktiot palauttavat vastaavat attribuuttitiedot maasta.

set-funktiot ottavat muuttuvan referenssin ja vastaavan arvon. set-funktioissa muutat Country-attribuutin arvon parametrina annettuun arvoon.

Lisää toimintoja maan valloittamiseen. Lisää funktio nimeltä conquer_nation() Player.rs:ään. Se ottaa syötteenä muunnettavan self:n – Playerin, kohdemaan Country-tyyppinä ja oman maan nimen merkkijonona. Tarkista, onko maa jo valloitettu. Tarkista, onko kohdemaasi oma maasi.

Jos maasi armeijakoko on suurempi kuin kohdemaat, käytä get_conquered_nations- ja set_conquered_nations-funktioita merkataksesi kohdemaa valloitetuksi.

Kun maa on valloitettu, lisää maasi populaatioon kohdemaan populaatio käyttämällä set_populationia. Ja lisää armeijan koko vastaamaan maatasi + kohdemaata.

Printtaa, että olet valloittanut maan.

Jos armeijan koko on sama, ilmoita tämä esimerkkiajon tavalla.

Muussa tapauksessa tulosta esimerkkiajon mukaisesti.

Esimerkkiajo valloituksesta:
| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |
Choose your country:
4
| Inspection on your own nation? | y = yes | n = no |
y
An inspection has been completed..
Country information:
Name: Denmark
Population: 6000000
Army size: 50000
| 1) Spy on a country | 2) Invade a country | 0) Exit program |
2

1. Denmark
2. Finland
3. Norway
4. Sweden
   2
   You have lost your war against Finland. You have been conquered.
   | Inspection on your own nation? | y = yes | n = no |
   n
   The leader is confident. No inspection needed.
   | 1) Spy on a country | 2) Invade a country | 0) Exit program |
   0

Tehtävä 5

Lisää Country.rs:iin funktio, joka lisää armeijaan 50 000 henkilöä, ellei sotilasmäärä ylitä maan väkilukua – sotilashenkilöstöä ei voi olla enemmän kuin asukasmäärää. Lisää mahdollisuus laajentaa armeijaa pelin loopissa.

Lisää GameMap.rs:ssä get_countries- ja set_countries-funktiot. Lisää myös other_countries_turn, jossa jos maata ei ole vielä valloitettu eikä se ole sama kuin pelaajan maa, se laajentaa armeijaansa. Kutsu tämä toiminto pääloopin lopussa, joten jokaisella maalla on vuoronsa, kun pelaaja on tehnyt siirtonsa.

Esimerkkiajo:
| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |
Choose your country:
1
| Inspection on your own nation? | y = yes | n = no |
y
An inspection has been completed..
Country information:
Name: Finland
Population: 5600000
Army size: 900000
| 1) Spy on a country | 2) Invade a country | 3) Expand military |
3
| Inspection on your own nation? | y = yes | n = no |
y
An inspection has been completed..
Country information:
Name: Finland
Population: 5600000
Army size: 950000
| 1) Spy on a country | 2) Invade a country | 3) Expand military |
2

1. Denmark
2. Finland
3. Norway
4. Sweden
   1
   You have conquered Denmark
   | Inspection on your own nation? | y = yes | n = no |
   n
   The leader is confident. No inspection needed.
   | 1) Spy on a country | 2) Invade a country | 3) Expand military |
   2
5. Denmark
6. Finland
7. Norway
8. Sweden
   3
   You have conquered Norway
   | Inspection on your own nation? | y = yes | n = no |
   y
   An inspection has been completed..
   Country information:
   Name: Finland
   Population: 17100000
   Army size: 1250000
   | 1) Spy on a country | 2) Invade a country | 3) Expand military |
   1
9. Denmark
10. Finland
11. Norway
12. Sweden
    4
    Espionage successful.
    Country information:
    Name: Sweden
    Population: 10000000
    Army size: 350000
    | Inspection on your own nation? | y = yes | n = no |
    n
    The leader is confident. No inspection needed.
    | 1) Spy on a country | 2) Invade a country | 3) Expand military |
    2
13. Denmark
14. Finland
15. Norway
16. Sweden
    4
    You have conquered Sweden
    You have conquered all your targets. Good work, comrade!

Last modified: Thursday, 6 February 2025, 12:53 PM
You are logged in as Teemu Aitomeri (Log out)
Data retention summary
Policies
Get the mobile app
Copyright © LUT University
