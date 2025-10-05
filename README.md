# Projekat iz predmeta Napredne tehnike programiranja

## Tema: Paralelno računanje determinante matrice primenom Laplasovog razvoja

### Ocena za koju se radi: 10

---

### 1. Opis problema

Cilj ovog projektnog zadatka je implementacija i analiza algoritma za računanje determinante matrice korišćenjem Laplasovog razvoja. Determinanta je fundamentalna vrednost u linearnoj algebri koja se može izračunati samo za kvadratne matrice i pruža važne informacije o matrici, kao što je njena invertibilnost.

Laplasov razvoj je rekurzivni metod za računanje determinante. Prema ovom metodu, determinanta matrice $A$ veličine $n \times n$ može se izračunati razvojem po bilo kojoj vrsti ili koloni. Razvoj po prvoj vrsti dat je formulom:

$$
\det(A) = \sum_{j=1}^{n} (-1)^{1+j} a_{1j} M_{1j}
$$

gde je $a_{1j}$ element u prvoj vrsti i j-toj koloni, a $M_{1j}$ je minor matrice koji odgovara tom elementu, odnosno determinanta podmatrice koja se dobija uklanjanjem prve vrste i j-te kolone.

Računska složenost ovog algoritma je $O(n!)$, što ga čini izuzetno neefikasnim za matrice većih dimenzija. Upravo zbog toga, ovaj problem je idealan kandidat za paralelizaciju, jer se računanje minora $M_{1j}$ može obavljati nezavisno za svako $j$.

### 2. Metode za rešavanje problema

Kako bi se izvršila sveobuhvatna analiza performansi, rešenje će biti implementirano u programskim jezicima Python i Rust. Pristup omogućava direktno poređenje prednosti i mana svakog ekosistema za rešavanje problema iz oblasti računarstva visokih performansi.

#### Implementacija u programskom jeziku Python

* **Sekvencijalna implementacija**: Biće kreirana rekurzivna funkcija u Python-u koja implementira Laplasov razvoj. Kao i u Rust verziji, bazni slučaj rekurzije biće matrica dimenzija $2 \times 2$.
* **Paralelna implementacija**: Za paralelizaciju će se koristiti Python-ova `multiprocessing` biblioteka. Glavni proces će distribuirati zadatke računanja minora radnim procesima (`workers`), sačekaće njihov završetak i sabrati rezultate.

#### Implementacija u programskom jeziku Rust

* **Sekvencijalna implementacija**: Biće kreirana rekurzivna funkcija koja kao argument prima matricu i vraća njenu determinantu. Bazni slučaj rekurzije je matrica dimenzija $2 \times 2$, za koju se determinanta računa direktno po formuli:
    $$
    \det \begin{pmatrix} a & b \\ c & d \end{pmatrix} = ad - bc
    $$
* **Paralelna implementacija**: Paralelna verzija će iskoristiti činjenicu da su proračuni minora ($M_{1j}$) u formuli međusobno nezavisni. Za računanje svakog minora biće kreirana posebna nit (thread) korišćenjem standardne biblioteke Rust-a (`std::thread`).

#### Uporedna analiza performansi i skaliranja

Nakon implementacije sve četiri verzije (sekvencijalna i paralelna za Python i Rust), biće sprovedeni eksperimenti jakog i slabog skaliranja kako bi se uporedilo njihovo ubrzanje i efikasnost.

1.  **Hardverska i softverska specifikacija**: U izveštaju će biti navedene detaljne specifikacije sistema na kojem se vrši testiranje.
2.  **Teorijsko ubrzanje**: Analiziraće se kod kako bi se odredio procenat sekvencijalnog i paralelnog dela, a zatim izračunalo teorijsko maksimalno ubrzanje prema **Amdalovom** i **Gustafsonovom zakonu**.
3.  **Prikupljanje podataka**: Za svaku kombinaciju parametara (broj jezgara, veličina matrice), **svaka od četiri implementacije** biće izvršena dovoljan broj puta (npr. 30) kako bi se osigurala statistička relevantnost rezultata.

#### Vizualizacija rešenja

Poslednji korak projekta je vizualizacija rezultata uporedne analize, koja će biti realizovana u potpunosti unutar Rust okruženja korišćenjem grafičke biblioteke **Plotters**.

Na osnovu podataka prikupljenih tokom faze testiranja, Rust aplikacija će generisati sledeće grafike u vidu `.png` ili `.svg` datoteka, **gde će na svakom grafiku biti uporedno prikazani rezultati za Python i Rust**:

1.  **Grafik jakog skaliranja**: Prikazaće ostvareno ubrzanje u odnosu na broj procesorskih jezgara za fiksnu veličinu problema za obe paralelne implementacije (Python i Rust). Na grafiku će biti iscrtana i linija idealnog ubrzanja prema Amdalovom zakonu.
2.  **Grafik slabog skaliranja**: Prikazaće skaliranje efikasnosti za problem čija veličina raste proporcionalno broju jezgara za obe paralelne implementacije. Na grafiku će biti iscrtana i linija idealnog ubrzanja prema Gustafsonovom zakonu.

.

├── python/

│   ├── sequential.py   # Sekvencijalna Python implementacija

│   └── parallel.py     # Paralelna Python implementacija

├── src/

│   ├── main.rs         # Glavna logika, parsiranje argumenata i pokretanje eksperimenata

│   ├── sequential.rs   # Modul sa sekvencijalnom Rust implementacijom

│   ├── parallel.rs     # Modul sa paralelnom Rust implementacijom

│   └── visualization.rs # Modul za generisanje grafika na osnovu rezultata

├── charts/             # Direktorijum gde će biti sačuvani generisani grafici

├── report/

│   └── report.pdf      # Izveštaj sa analizom skaliranja i rezultatima

├── Cargo.toml

└── README.md

### 3. Struktura projekta

Projekat će biti organizovan na sledeći način kako bi se jasno odvojile Python i Rust implementacije:
