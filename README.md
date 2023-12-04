Projekta struktūra:
```
|-server - nodejs serveris, kas saņem animāciju pieprasījumus un kontrolē animāciju palaišanu
|-visualiser - web lapa, kas saņem datus no servera un darbina lampiņu 3D vizualizāciju as Three.js. Tiek iekļauts kopējā web lapā.
|-web - web lapa lampiņu kontrolēšanai
|-generator - rust programma, kas saņem parametrus par animācijām, ģenerē animācijas un atgriež tās pa kadriem serverim
```