Tehtävä 1

Tee rocket-frameworkin avulla (funktio nimeltä hello) get-päätepiste kohtaan "/hello/<parametri>", joka palauttaa "Hei, {parametri}!".

Huomaa, että konsoliin tuleva tieto Rocket-frameworkistä ei ole tällä viikolla yhtä relevantti kuin endpointtien (esim. get, post jne.) lopputulos.

Esimerkkiajon lopputulos:

Esimerkki komentokehote tulostus:

Punaisella on korostettu kohta, jota klikkaamalla tai kopiomalla ja liittämällä pitäisi päästä katsomaa ylläolevan kuvan lailla ohjelman lopputulosta. Huomaa myös lisänä tekemäsi routen osoite, esim. "/hello/testi".

Configured for debug.

> > address: 127.0.0.1
> > port: 8000
> > workers: 16
> > max blocking threads: 512
> > ident: Rocket
> > IP header: X-Real-IP
> > limits: bytes = 8KiB, data-form = 2MiB, file = 1MiB, form = 32KiB, json = 1MiB, msgpack = 1MiB, string = 8KiB
> > temp dir: C:\Users\veikk\AppData\Local\Temp\
> > http/2: true
> > keep-alive: 5s
> > tls: disabled
> > shutdown: ctrlc = true, force = true, grace = 2s, mercy = 3s
> > log level: normal
> > cli colors: true
> > secret key: [generated]
> > Warning: secrets enabled without a stable `secret_key`
> > disable `secrets` feature or configure a `secret_key`
> > this becomes an error in non-debug profiles
> > Routes:
> > (hello) GET /hello/<someone>
> > Fairings:
> > Shield (liftoff, response, singleton)
> > Shield:
> > X-Content-Type-Options: nosniff
> > Permissions-Policy: interest-cohort=()
> > X-Frame-Options: SAMEORIGIN
> > Rocket has launched from http://127.0.0.1:8000
> > GET / text/html:
> > No matching routes for GET / text/html.
> > No 404 catcher registered. Using Rocket default.
> > Response succeeded.
> > GET /favicon.ico image/avif:
> > No matching routes for GET /favicon.ico image/avif.
> > No 404 catcher registered. Using Rocket default.
> > Response succeeded.
> > GET / text/html:
> > No matching routes for GET / text/html.
> > No 404 catcher registered. Using Rocket default.
> > Response succeeded.
> > GET /hello/testi text/html:
> > Matched: (hello) GET /hello/<someone>
> > Note: Using `String` as a parameter type is inefficient. Use `&str` instead.
> > `String` is used a parameter guard in src/main.rs:5.
> > Outcome: Success(200 OK)
> > Response succeeded.
> > GET /hello/testi text/html:
> > Matched: (hello) GET /hello/<someone>
> > Note: Using `String` as a parameter type is inefficient. Use `&str` instead.
> > `String` is used a parameter guard in src/main.rs:5.
> > Outcome: Success(200 OK)
> > Response succeeded.
> > GET /hello/testi text/html:
> > Matched: (hello) GET /hello/<someone>
> > Note: Using `String` as a parameter type is inefficient. Use `&str` instead.
> > `String` is used a parameter guard in src/main.rs:5.
> > Outcome: Success(200 OK)
> > Response succeeded.
> > GET /favicon.ico image/avif:
> > No matching routes for GET /favicon.ico image/avif.
> > No 404 catcher registered. Using Rocket default.
> > Response succeeded.

Tehtävä 2

Tee funktio nimeltä render_page, joka renderöi sivun käyttämällä RawHtml:ää Rocket-frameworkistä. renderöi sivu päätepisteestä "/<sivun_nimi>". Liitä page_name kohtaan "/page" rootin "/" sijaan.

Sivuilla tulee olla div, h1 ja p. Quote ja Bob -sivuilla on alla olevien kuvien kaltaiset tekstit - Otsikko h1-elementissä ja tekstit p-elementissä, sekä molemmat elementit div-elementin sisällä.

Jos käyttäjä yrittää päästä esimerkiksi "/newpage"en:, virhesivu, jossa on teksti "Sorry, the page is not available." tulee esittää käyttäjälle p-elementissä.

Esimerkkiajo "/page/bob":

Esimerkkiajo "/page/quote":

Esimerkkiajo "/page/testi": (testin sijalla voi olla mitä vain, paitsi bob tai quote):

Tehtävä 3

Tee POSt-request osoitteeseen "/", mounttaa POST-requesti "/receive" ja jos viestipyynnön käsittelijän vastaanottama data on "I eat yellow snow!", palauta merkkijono: "Don't do that!".

Muussa tapauksessa vastaa: "Response received: {variable_data}

”.

Voit käyttää jotain työkalua, esim. Postman, POST-routen testaamiseen.

Esimerkkiajot:

Yllä näemme routen osoitteen, keskellä olemme valinneet raw-muodon ja pistäneet tekstin body-osioon, ja alla näemme routen antaman vastauksen.

Tehtävä 4

Tee Rocket-kehyksen avulla HTML-lomake, jossa on kaksi valintaruutua, jossa id on good ja bad, arvo on good ja bad sekä nimetään vastaavasti good ja bad. Lisää myös otsikkoteksti h1-elementtiin tekstillä "How are you?".

Palauta käyttäjän syötteen perusteella h2 tekstin oikealla puolella:
good: "Hey, I am glad to hear that. Keep on rocking'! :)”

bad: "I'm sorry to hear that. I hope things get better for you. :("

good ja bad: Can you really be having both a good and a bad day at he same time?

ja jos vastaus on tyhjä: "You did not share your feelings. :("

Kun käyttäjä lähettää valintaruudun tiedot submit-napilla, ohjaa käyttäjä uudelleen "/answer"-reitille ja käsittele käyttäjän syöte siellä, renderöiden oikea lopputulos div:n sisällä, jonka id on "response", h2-elementissä.

Esimerkkikuvia:

Tehtävä 5

Tässä tehtävässä haet tiedoston sisällön get-requestilla ja lisäät tiedostoon sisältöä post-requestin kautta.

Tee get-request osoitteeseen "/message", joka palauttaa RawHtml<String>. Jos tiedostoa ei ole vielä olemassa, luo se nimellä data.txt. Muussa tapauksessa lue tiedoston sisältö, lisää se p-elementin sisään ja palauta se.

Tee post-reitti kohtaan "/message", joka ottaa Form-objektin, jonka tyyppi on Post, kirjoittaa annetun sisällön tiedoston riville ja palauttaa “Message received.”.

Esimerkkiajoja (taustan värillä ei merkitystä):
Post-route:

Get-route:
