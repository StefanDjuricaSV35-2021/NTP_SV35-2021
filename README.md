# Projekat iz predmeta Napredne tehnike programiranja

## Tema: Paralelno računanje determinante matrice primenom Laplasovog razvoja

### Ocena za koju se radi: 10

---

### 1. Opis problema

Cilj ovog projektnog zadatka je implementacija i analiza algoritma za računanje determinante matrice korišćenjem Laplasovog razvoja. Determinanta je fundamentalna vrednost u linearnoj algebri koja se može izračunati samo za kvadratne matrice i pruža važne informacije o matrici, kao što je njena invertibilnost.

Laplasov razvoj je rekurzivni metod za računanje determinante. Prema ovom metodu, determinanta matrice $A$ veličine $n \times n$ može se izračunati razvojem po bilo kojoj vrsti ili koloni. Razvoj po prvoj vrsti dat je formulom:

$$ \det(A) = \sum_{j=1}^{n} (-1)^{1+j} a_{1j} M_{1j} $$

gde je $a_{1j}$ element u prvoj vrsti i j-toj koloni, a $M_{1j}$ je minor matrice koji odgovara tom elementu, odnosno determinanta podmatrice koja se dobija uklanjanjem prve vrste i j-te kolone.

Računska složenost ovog algoritma je $O(n!)$, što ga čini izuzetno neefikasnim za matrice većih dimenzija. Upravo zbog toga, ovaj problem je idealan kandidat za paralelizaciju, jer se računanje minora $M_{1j}$ može obavljati nezavisno za svako $j$.

### 2. Metode za rešavanje problema

Rešenje će obuhvatiti sekvencijalnu i paralelnu implementaciju, detaljnu analizu performansi i vizualizaciju dobijenih rezultata.

#### Sekvencijalna implementacija (za ocenu 7)

Prvi korak je implementacija sekvencijalnog rešenja. Biće kreirana rekurzivna funkcija koja kao argument prima matricu i vraća njenu determinantu. Bazni slučaj rekurzije je matrica dimenzija $2 \times 2$, za koju se determinanta računa direktno po formuli: 

$$
 \det \begin{pmatrix} a & b \\ c & d \end{pmatrix} = ad - bc
$$

Za sve veće matrice, funkcija će primenjivati formulu Laplasovog razvoja.

#### Paralelna implementacija (za ocenu 7)

Paralelna verzija će iskoristiti činjenicu da su proračuni minora ($M_{1j}$) u formuli međusobno nezavisni. Za računanje svakog minora u razvoju po prvoj vrsti biće kreirana posebna nit (thread). Glavna nit će delegirati ove zadatke radnim nitima, sačekati njihov završetak i na kraju sumirati rezultate kako bi dobila konačnu vrednost determinante. Za upravljanje nitima koristiće se standardna biblioteka Rust-a (`std::thread`).

#### Analiza performansi i skaliranja (za ocenu 9)

Nakon implementacije obe verzije, biće sprovedeni eksperimenti **jakog** i **slabog skaliranja** kako bi se uporedilo ubrzanje paralelne verzije u odnosu na sekvencijalnu.

1.  **Hardverska i softverska specifikacija**: U izveštaju će biti navedene detaljne specifikacije sistema na kojem se vrši testiranje (model procesora, broj jezgara, količina RAM memorije, operativni sistem, verzija Rust kompajlera).
2.  **Teorijsko ubrzanje**: Analiziraće se kod kako bi se odredio procenat sekvencijalnog i paralelnog dela, a zatim izračunalo teorijsko maksimalno ubrzanje prema **Amdalovom** i **Gustafsonovom zakonu**.
3.  **Prikupljanje podataka**: Za svaku kombinaciju parametara (broj jezgara, veličina matrice), program će biti izvršen dovoljan broj puta (npr. 30) kako bi se osigurala statistička relevantnost rezultata. Podaci o vremenima izvršavanja biće sačuvani za kasniju vizualizaciju.

#### Vizualizacija rešenja (za ocenu 10)

Poslednji korak projekta je vizualizacija rezultata analize performansi, koja će biti realizovana u potpunosti unutar Rust okruženja. U tu svrhu koristiće se grafička biblioteka **Plotters**.

Na osnovu podataka prikupljenih tokom faze testiranja, Rust aplikacija će generisati sledeće grafike u vidu `.png` ili `.svg` datoteka:
1.  **Grafik jakog skaliranja**: Prikazaće ostvareno ubrzanje u odnosu na broj procesorskih jezgara za fiksnu veličinu problema. Na grafiku će biti iscrtana i linija idealnog ubrzanja prema Amdalovom zakonu.
2.  **Grafik slabog skaliranja**: Prikazaće skaliranje efikasnosti za problem čija veličina raste proporcionalno broju jezgara. Na grafiku će biti iscrtana i linija idealnog ubrzanja prema Gustafsonovom zakonu.

Ovaj pristup omogućava da ceo proces – od izvršavanja, preko analize, do vizualizacije – bude automatizovan i sadržan unutar jednog projekta.

### 3. Struktura projekta

Projekat će biti organizovan na sledeći način:
.

├── src/

│   ├── main.rs         # Glavna logika, parsiranje argumenata i pokretanje eksperimenata

│   ├── sequential.rs   # Modul sa sekvencijalnom implementacijom

│   ├── parallel.rs     # Modul sa paralelnom implementacijom

│   └── visualization.rs # Modul za generisanje grafika na osnovu rezultata

├── charts/             # Direktorijum gde će biti sačuvani generisani grafici

├── report/

│   └── report.pdf      # Izveštaj sa analizom skaliranja i rezultatima

├── Cargo.toml
 
└── README.md
