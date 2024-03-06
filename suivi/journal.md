## 17/11/2023

TODO
- Sketch the main ideas behind the creation of texts for songs
- Sketch the main ideas behind generation of text that makes sense
- Define when two or more words "are coherent", i.e. make sense

## 24/11/2023, Degorre

Gros document rédigé listant les idées envisagées. Problèmes :

- beaucoup trop d'aspects différents à traiter
- la plupart des pistes sont assez floues ("on pourrait faire... " ... mais on ne voit pas vraiment comment)
- certaines pistes moins floues mais fournissant déjà du travail pour un projet entier à elles seules (notamment traitement du signal)
- certaines pistes consistent juste à configurer un réseau de neurones

-> il faut délimiter à une tâche principale nécessitant un travail substantiel mais réalisable.

Parmi les pistes discutées en séance : un sujet de traitement du signal pourrait convenir (par exemple : générateur de partitions avec nuances à partir de signal audio).

-> à faire : rédiger projet bien *délimité* comportant un objectif principal non trivial clair, RdV la semaine prochaine.

## 1/12/2023

Scheiner absent (malade)

- projet du README2.md : OK, mais très ambitieux -> supprimer l'autre README.md et renommer le 2.
- choisir une tâche principale à prioritiser entre tracking et shifting (pour l'instant, d'après la timeline, c'est le tracking qui est prioritaire)
- il faut distinguer le shifting (aveugle) et l'autotune (qui utilise le tracking)
- il faut une UI (en ligne de commande) dès le départ car c'est ce qui permet de tester l'intégration
- mentionner les tests unitaires
- compléter la méthodologie de test pour le shifting
- préciser le langage de programmation (C++ ?)

## 14/2/2024

### Fait

- priorité au pitch tracking -> on essaye de créer des fichiers midi
- FFT par Cooley-Tukey (mais à tester)
- Algorithme
- Affichage graphique primaire
- Lecture fichier audio

### TODO

- [] algo de tracking : coder un/des algorithme/s de tracking
- [] outillage de test
  - [] pour FFT: comparaison avec autre implémentation, FFT inverse
  - [] déterminer méthodologies de test pour tracking
  - [] notion de distance entre échantillons sonores -> coder cette distance
  - [] notion de distance entre fichiers midi -> coder cette distance
  - [] écrire jeux de test

## 6/3/2024

### Fait

- reverse FFT
- peut-être du travail sur le shifting? (Scheiner absent, ne peut pas en parler)

### À faire

- continuer objectifs donnés le 14/2
- préparer une démo
- réviser calendrier
- supprimer fichiers de projet obsolètes
