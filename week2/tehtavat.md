Tehtävä 1
Luo constant-muuttuja string slice, arvolla "I want to be changed.".
Luo uusi string tuosta constantista käyttäen luomaasi funktiota create_default() joka luo String-muuttujan ja palauttaa sen.
Luo funktio nimeltä remove_latest_word() joka ottaa muokattavan muuttujan (mutable) referenssin stringiin, poistaa viimeisimmän sanan ja välilyönnin sanasta.

Looppaa käyttäjälle menua. Käyttäjä valitsee joko 1) Reset 2) Poista viimeisin sana 3) lisää sana 4) printtaa merkkijono ja 0) lopeta ohjelma. Valitessaan 1), ohjelma luo uuden oletussanan käyttäen luomaasi funktiota create_default. Valinta 2) poistaa sanan lauseesta käyttäen luomaasi funktiota. 3) lisää sanan merkkijonon loppuun käyttäjän syötteestä. 4) printtaa nykyisen merkkijonon. 0) lopettaa ohjelman. Muissa tapauksissa printtaa "Invalid input. Try again.".

Huom!
Eri sanat erotellaan välilyönnein - sanan poisto ja lisäys tapahtuu välilyöntien välein.
Esimerkkiajo:

| 1) Reset | 2) Remove a word | 3) Add a word | 4) Print string | 0) End program |
asd
Parsing error. Was the input a number?
| 1) Reset | 2) Remove a word | 3) Add a word | 4) Print string | 0) End program |
3
The new word:
Eka sana
| 1) Reset | 2) Remove a word | 3) Add a word | 4) Print string | 0) End program |
4
I want to be changed. Eka sana
| 1) Reset | 2) Remove a word | 3) Add a word | 4) Print string | 0) End program |
3
The new word:
lisäsana
| 1) Reset | 2) Remove a word | 3) Add a word | 4) Print string | 0) End program |
4
I want to be changed. Eka sana lisäsana
| 1) Reset | 2) Remove a word | 3) Add a word | 4) Print string | 0) End program |
2
| 1) Reset | 2) Remove a word | 3) Add a word | 4) Print string | 0) End program |
4
I want to be changed. Eka sana
| 1) Reset | 2) Remove a word | 3) Add a word | 4) Print string | 0) End program |
2
| 1) Reset | 2) Remove a word | 3) Add a word | 4) Print string | 0) End program |
4
I want to be changed. Eka
| 1) Reset | 2) Remove a word | 3) Add a word | 4) Print string | 0) End program |
1
| 1) Reset | 2) Remove a word | 3) Add a word | 4) Print string | 0) End program |
4
I want to be changed.
| 1) Reset | 2) Remove a word | 3) Add a word | 4) Print string | 0) End program |
0

Tehtävä 2
Tee laskinohjelma joka voi tehä muokkauksia numeroon. Numero alustetaan nollaksi. Käytä i32 arvoa numeron tallettamiseen.

Tee looppu joka käyttäjän syötteellä 1) alustaa numeron nollaksi, 2) lisää käyttäjän antaman numeron lukuun, 3) vähentää käyttäjän antaman numeron luvusta, 4) kertoo luvun käyttäjän antamalla luvulla, 5) jakaa luvun käyttäjän antamalla luvulla, 6) printtaa numeron ja 0) lopettaa ohjelman. Muissa tapauksissa, printtaa "Invalid input. Try again".".

Kun käyttäjä haluaa modifioida numeroa, käytä muutettavia referenssejä (mutable reference) funktioiden parametreissä kuten lisäys ja jakofunktio, ja muuttumattomia referenssejä kun printtaat luvun käyttäjän nähtäväksi.
Esimerkkiajo:

| 1) Reset | 2) Addition | 3) Retraction | 4) Multiplication | 5) Division | 6) Print | 0) End program |
6
Current number: 0
| 1) Reset | 2) Addition | 3) Retraction | 4) Multiplication | 5) Division | 6) Print | 0) End program |
2
What number?
5
| 1) Reset | 2) Addition | 3) Retraction | 4) Multiplication | 5) Division | 6) Print | 0) End program |
6
Current number: 5
| 1) Reset | 2) Addition | 3) Retraction | 4) Multiplication | 5) Division | 6) Print | 0) End program |
1
| 1) Reset | 2) Addition | 3) Retraction | 4) Multiplication | 5) Division | 6) Print | 0) End program |
6
Current number: 0
| 1) Reset | 2) Addition | 3) Retraction | 4) Multiplication | 5) Division | 6) Print | 0) End program |
2
What number?
1
| 1) Reset | 2) Addition | 3) Retraction | 4) Multiplication | 5) Division | 6) Print | 0) End program |
4
What number?
7
| 1) Reset | 2) Addition | 3) Retraction | 4) Multiplication | 5) Division | 6) Print | 0) End program |
6
Current number: 7
| 1) Reset | 2) Addition | 3) Retraction | 4) Multiplication | 5) Division | 6) Print | 0) End program |
5
What number?
2
| 1) Reset | 2) Addition | 3) Retraction | 4) Multiplication | 5) Division | 6) Print | 0) End program |
6
Current number: 3
| 1) Reset | 2) Addition | 3) Retraction | 4) Multiplication | 5) Division | 6) Print | 0) End program |
0
Ending the program.

Tehtävä 3
Tässä tehtävässä teet uuden julkisen moduulin julkisella funktiolla ja käytät sitä toisessa tiedostossa, päätiedostossa.
Tee src-kansioon uusi kansio nimeltä "new_module". Tee mod.rs-tiedosto ja new_file.rs. Tee mod.rs-tiedostosta new_file julkinen moduuli. Tee tiedostossa new_file.rs julkinen funktio nimeltä calling_from_far(), joka tulostaa "Hello! I am speaking to you from another file!" ja palauttaa 1.

Tuo/importtaa main.rs:n pääfunktioon "new_module" ja käytä new_module-tiedostosta new_file:a.
Kutsu funktiota calling_from_far tiedostossa main.rs.

Esimerkkiajo:
Hello! I am speaking to you from another file!

Tehtävä 4
Tässä tehtävät hyödynnät laajemmin luomasi ulkoisen moduulin funktioita tarpeitten mukaan.

Tee ohjelma, jossa käyttäjä voi syötteen antamalla liikuttaa x:ää merkkitaulukossa. X ei voi likkua taulukon ulkopuolelle.

Laita kaikki karttafunktiot ja x:n siirto-funktiot kansioon nimeltä "map" src-kansion sisällä. Laita (julkiset) funktiot arraymap.rs-tiedostoon. Tee mod.rs-tiedostossa arraymapista julkinen.

Tee funktio, joka tekee konsoliin taulukon näköisen rakenteen tulostusfunktioiden avulla tulostaaksesi 5 riviä tähtiä, 5 x 5 ruudukko. Tulosta ruudukon keskelle x-kirjain tähden (\*) sijaan.

Tee silmukka, joka siirtää x:ää käyttäjän syötteen perusteella. Syötä w) siirtää x:n ylöspäin ruudukossa, a) siirtää sitä vasemmalle, s) siirtää sitä alas, d) siirtää sitä oikealle. Jos x ei pääse liikkumaan ruudukon ulkopuolelle, älä siirrä x:ää ja tulosta "Can't move out of the map" nykyisen ruudukon tulostuksen lisäksi. e) lopettaa ohjelman.

Käytä merkkijono taulukkoa tietojen tallentamiseen.

Käytä funktiota nimeltä create_map() oletuskartan luomiseen.

Kutsu päätiedostossa funktiota create_map() oletuskartan luomiseen. Tulosta kartta.

Tee sitten silmukka yllä mainitulla tavalla, pyydä käyttäjältä syötteitä w, a s, tai d liikuttaaksesi x:ää tai e lopettaaksesi ohjelman. Kutsu asianmukaisia funktioita arraymap.rs-tiedostossa tarvittaessa. Käytä referenssejä, kun välität arraymap-muuttujan funktioille, jotka ovat muuttuvia tai muuttumattomia tarpeiden mukaan (mutable ja immutable).
Esimerkkiajo:

```bash
* * * * *
* * * * *
* * x * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
w
* * * * *
* * x * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
w
* * x * *
* * * * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
w
Can't move out of the map
* * x * *
* * * * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
a
* x * * *
* * * * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
a
x * * * *
* * * * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
a
Can't move out of the map
x * * * *
* * * * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
d
* x * * *
* * * * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
d
* * x * *
* * * * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
s
* * * * *
* * x * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
d
* * * * *
* * * x *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
d
* * * * *
* * * * x
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
d
Can't move out of the map
* * * * *
* * * * x
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
s
* * * * *
* * * * *
* * * * x
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
s
* * * * *
* * * * *
* * * * *
* * * * x
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
s
* * * * *
* * * * *
* * * * *
* * * * *
* * * * x
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
s
Can't move out of the map
* * * * *
* * * * *
* * * * *
* * * * *
* * * * x
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
a
* * * * *
* * * * *
* * * * *
* * * * *
* * * x *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
e
Ending the program.
```

Tehtävä 5
Tässä harjoituksessa teet jälleen uusia tiedostoja receipt-nimiseen moduuliin. Tee receipt-niminen kansio, jossa on tiedostot content.rs, product.rs ja mod.rs. Tee mod.rs:ssä kaksi muuta tiedosta julkiseksi. Tee product.rs:ssä rakenne nimeltä StoreProduct, ja funktio nimeltä create_products(), joka tekee vektorin, jonka tyyppi on StoreProduct ja tallentaa 3 StoreProduct-tyypin muuttujaa kuvassa/etukäteen annetuilla arvoilla ja palauttaa taulukon, jossa on kaikki 3 annetussa järjestyksessä. StoreProductilla on nimi ja kokonaishinta.

content.rs-tiedostossa on suurin osa koodista tässä tehtävässä.
Käytä StoreProductia product.rs-tiedostosta.

Tee constant-muuttujat kolmelle tuotteelle kuvan mukaisesti.
Tee structi nimeltä ReceiptContent, jolla on kaksi attribuuttia: "products", joka on StoreProduct-tyyppinen vektori, ja "store", jonka tyyppi on String.

Tee silmukka, joka pyytää käyttäjää joko 1) lisäämään ostoskoriin 2) poistamaan uusin tuote tai 3) ostamaan nykyiset tuotteet ostoskorissa. Jos käyttäjä päättää lisätä ostoskoriin, kysy käyttäjältä minkä tuotteen hän haluaa lisätä ja tulosta kaikki vaihtoehdot omille rivilleen käyttämällä tulostustoimintoa. Valitsemalla 1, 2 tai 3, käyttäjän syöte vastaa yllä olevan kuvan tuotteita.

Kun käyttäjä lisää tuotteen funktion avulla, lisää tuote muuttujaan, jonka tyyppi on ReceiptContent.

Kun käyttäjä haluaa poistaa viimeisimmän tuotteen, poista uusin tuote ReceiptContent-muuttujasta.

Kun käyttäjä haluaa ostaa kaikki valitsemansa tuotteet, laske tuotteiden kokonaishinta käyttämällä luomaasi funktiota nimeltä complete_purchase() jolla on palautustyyppinä tyhjä Result-tyyppi. Kuitti tulee kirjoittaa tekstitiedostoon nimeltä "receipt.txt".

Esimerkki:

Kun käyttäjä haluaa ostaa kaikki valitsemansa tuotteet, laske tuotteiden kokonaishinta käyttämällä luomaasi funktiota nimeltä complete_purchase(). Alla on esimerkkiajon lisäksi receipt-tulostus. Kirjoita tiedot kuittiin kuinka monta tuotetta ostettiin, kuinka paljon ne maksoivat ja lopullinen hinta, myymälän nimi.

Esimerkkiajo:
| 1) Add to cart | 2) Remove most recent product | 3) Purchase |
1
Which product would you like to add?

1. Zbox 720 | Price - 600
2. GPU - AND Random RT6600 | Price - 200
3. Potato | Price - 1
   1
   | 1) Add to cart | 2) Remove most recent product | 3) Purchase |
   1
   Which product would you like to add?
4. Zbox 720 | Price - 600
5. GPU - AND Random RT6600 | Price - 200
6. Potato | Price - 1
   2
   | 1) Add to cart | 2) Remove most recent product | 3) Purchase |
   1
   Which product would you like to add?
7. Zbox 720 | Price - 600
8. GPU - AND Random RT6600 | Price - 200
9. Potato | Price - 1
   3
   | 1) Add to cart | 2) Remove most recent product | 3) Purchase |
   1
   Which product would you like to add?
10. Zbox 720 | Price - 600
11. GPU - AND Random RT6600 | Price - 200
12. Potato | Price - 1
    1
    | 1) Add to cart | 2) Remove most recent product | 3) Purchase |
    2
    | 1) Add to cart | 2) Remove most recent product | 3) Purchase |
    3
    Thank you for your purchase!

Kuittitulostus esimerkki (tulosta vain ne tuotteet alla olevalla tavalla, joita on ostettu vähintään 1 kappale):

## Imaginary Town General Store

Zbox 720 (1) - 600€
GPU - AND Random RT6600 (1) - 200€
Potato (1) - 1€

---

## Final price: 801€

Tehtävä 6
Tässä harjoituksessa sinulle annetaan valmis ohjelma, jossa on virhe liittyen lifetime konseptiin. Ohjelma ei käänny, ja korjataksesi tilanteen, pitää sinun muokata ohjelmaa siirtämällä tai poistamalla osioita jotta ohjelma toimii. Tarkoituksenasi on muokata ohjelmaa niin, että se ajautuu. Kokeile ajaa ohjelma ensin omalla laitteellasi, lue kääntäjän antama virhe, korjaa ohjelma ja palauta korjattu versio sellaisenaan Codegradeen.

Ohjelman tulisi palauttaa viite siihen muuttujaan, jonka merkkijono on pidempi. Koska viitteiden elinaika riippuu niiden omistajan elinajasta, pitää tämä ottaa huomioon ohjelmaa tehdessä.

Ohjelma koostuu lib.rs ja main.rs tiedostoista, mitkä on annettu valmiiksi. Projektikansion nimi täytyy olla "ch2_pt6_lifetime". Muokkaa vain lib.rs tiedoston run_main() funktiota.

Ohjelma:

lib.rs

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
if s1.len() > s2.len() {
s1
} else {
s2
}
}

pub fn run_main() -> String {
let result;
{
let string1 = String::from("long string");
let string2 = "short";

        result = longest(&string1, string2);
    }
    result.to_string()

}

main.rs:

use ch2_pt6_lifetime::run_main;

fn main() {
let result = run_main();
println!("The longest string is: {}", result);
}

Projektin rakenne:
ch2_pt6_lifetime/
├── src/
│ ├── lib.rs
│ └── main.rs
├── Cargo.toml

Last modified: Thursday, 12 June 2025, 3:00 PM
