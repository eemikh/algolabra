# Määrittelydokumentti

Opinto-ohjelma: tietojenkäsittelytieteen kandidaatti (TKT)

Ohjelma implementoi RSA-salauksen (salaus ja salauksen purkaminen) sekä RSA-avainten generoinnin. Nämä toiminnot on eroteltu, ja tarkemmin ne toimivat seuraavasti:

- Avaimen generointi: syötteenä ohjelmalle annetaan kohdetiedostot ja avaimen haluttu koko, ja ohjelma kirjoittaa yhteen tiedostoon generoidun julkisen avaimen ja toiseen tiedostoon yksityisen avaimen.
- Tekstin salaaminen: syötteenä ohjelmalle annetaan julkisen avaimen tiedosto sekä salattava teksti (tavuja), ohjelma palauttaa salatun tekstin
- Salatun tekstin purkaminen: syötteenä ohjelmalle annetaan yksityisen avaimen tiedosto sekä salattu teksti (tavuja), ohjelma palauttaa alkuperäisen salaamattoman tekstin

RSA-avainten generointi toteutetaan generoimalla satunnainen oikeankokoinen luku ja iteroimalla tästä parittomia lukuja, kunnes löytyy jokin luku, jolle pienillä alkuluvuilla (Eratostheneen seulalla löydettyjä) jako ei anna kokonaislukua ja jokin määrä Miller-Rabin-testejä menee läpi.

Miller-Rabin-testin aikavaativuus on O(k n^3), missä n on luvun numeroiden määrä ja k on iteraatioiden määrä (https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test#Complexity), mutta aikavaativuutta muille RSA:han liittyville algoritmeille on hankala määrittää.

Ohjelmointikielenä käytetään Rustia ja voin vertaisarvioida C:llä, Pythonilla ja Haskellilla tehtyjä projekteja. Dokumentaatio on suomeksi.

Lähteitä:

- https://www.cis.upenn.edu/~jean/RSA-primality-testing.pdf (etenkin Miller-Rabin)
