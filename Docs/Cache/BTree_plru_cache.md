Requisiti:

Prellocare lo spazio per i nodi in uno slab di memoria

Utilizzare un B-Tree sotto un RwLock

Utilizzare gestire la rimozione degli elementi dalla cache tramite Pseudo-LRU implementato nei nodi dell'albero binario:
- https://en.wikipedia.org/wiki/Pseudo-LRU#Tree-PLRU
- https://en.wikipedia.org/wiki/Cache_replacement_policies#Pseudo-LRU_(PLRU)

Decidere se implementare i bit/bool per settare il destra o sinistra del LRU tramite atomics o no.

Problemi:
- 1 Da implementare tutto a mano!
- 2 Una volta riempita la cache, le inserzioni diventano 2logN

Indagare la fattibilità di:
- Usare LIRS con i path per misurare la località(?) https://en.wikipedia.org/wiki/LIRS_caching_algorithm
- Usare l'approssimazione di LIRS usata da NetBSD, CLOCK-PRO:
 https://en.wikipedia.org/wiki/Cache_replacement_policies#CLOCK-Pro
https://www.usenix.org/legacy/events/usenix05/tech/general/jiang.html
https://www.usenix.org/legacy/events/usenix05/tech/general/full_papers/jiang/jiang.pdf