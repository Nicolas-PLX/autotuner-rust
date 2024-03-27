# Génération automatique de paroles de chansons à l'aide d'algorithme d'intelligence artificielle  

## À quel problème répond-il ?  

Notre projet répond au problème de la création automatisée de paroles de chansons
dans différent style musicaux, en utilisant des algorithmes et de la modélisation
de données.

## Objectifs  

### Objectif Principal :  

Concevoir et implémenter un logiciel Auto-Tune capable de détecter et de corriger la hauteur des notes dans des enregistrements audio.

### Intermédiaires :
- Concevoir un modèle de données nous permettant de travailler avec des signaux. Nous avons considéré des algorithmes telles que le FTF (fast fourier transform), ou bien le constant Q transform
- Réaliser la partie pitch tracking, autrement dit la partie qui va nous permettre de savoir a quelle hauteur notre signal est au cours de l'audio. Nous avons prévu pour cela d'utiliser un algorithme comme le YIN ou le PYIN.
- Programmer la partie pitch correction. Dans cette partie, on veut pouvoir modifier la hauteur du signal sans altérer sa vitesse ou sa durée, on veut que la hauteur du chant change mais qu'il reste compréhensible. Pour cela, nous pouvons utiliser un algorithme tel que le PSOLA (Pitch Synchronous Overlap and Add)
- Créer une interface utilisateur simple permettant d"intéragir avec le code

### Supplémentaires :
- Permettre à l'utilisateur d'expérimenter avec différents réglages tels que la vitesse et la justesse de la correction pour avoir un aspect créatif
- Optimiser les performances pour une utilisation en temps réel

### Objectifs intermédiaires :  

Pour atteindre l'objectif principal, nous allons collecter une base de données de paroles de chansons. 
Nous prétraiterons ces données en les classifiants par des paramètres tels que style musical, 
la longueur etc... en les nettoyant, en extrayant des informations sur les rimes 
et les métriques, et en les structurant en ensembles de données adaptés à l'apprentissage 
des chaînes de Markov. Nous utiliserons la librairie "pandas" de python pour traiter la base
de données.

De plus, Nous pourrions explorer plusieurs autres idées (en plus des chaînes de Markov)
pour notre projet, déjà un modèles de génération de texte, mais aussi des réseaux de neurones 
récurrents, ou encore d'autres méthodes.

### Objectifs supplémentaires :  

Si le projet progresse correctement, nous pourrions déjà ajouter des paramètres supplémentaires 
pour la génération de paroles de chansons (en plus du style de musique), comme le style
de rimes, le nombre de vers, la taille des vers, le rythme. Nous pourrions éventuellement rajouter la 
personnalisation des paroles,en fonction des préférences que l'utilisateur fournira au programme. 
Nous pourrions rajouter aussi une fonctionnalité d'évaluation de la qualité des paroles 
générées par le programme, pour ensuite amélioré notre programme à l'aide de ces évaluations. 

## Aspect Technique :  

### Création de chaînes :  

Les chaînes de Markov seront créées en analysant la structure et la séquence des mots 
dans les paroles de chansons. Pour ce faire, nous allons diviser chaque chanson en 
séquences de mots, et observer les transitions entre ces mots. Les transitions 
seront utilisées pour former les probabilités de transition entre les mots, créant ainsi 
la matrice de transition de la chaîne de Markov. 

### Caractérisation de l'état de la chaîne :

Chaque état de la chaîne représentera un mot spécifique, et le passé sera caractérisé 
par la séquence des mots précédents menant à l'état actuel. Ainsi, à un moment donné, 
l'état actuel de la chaîne serait un mot, et le passé serait la séquence de mots qui a 
conduit à cet état. Ces informations seront utilisées pour prédire le mot suivant dans 
génération de paroles de chansons.  

### Stockage de la chaîne de Markov :

La chaîne de Markov pour une langue donnée sera stockée sous la forme d'une matrice de transition. 
Cette matrice représentera les probabilités de transition entre chaque paire de mots dans la langue spécifique.
En utilisant des structures de données appropriées, telles que des dictionnaires ou 
des tableaux multidimensionnels, nous pourrons accéder rapidement aux probabilités de 
transition lors de la génération des paroles. Nous irons stocké les chaînes préalablement généré
dans des fichiers, pour pouvoir les réutiliser ensuite.  

## Testabilités du projet :  

### Test de cohérence :  
    
Les paroles générées doivent être cohérentes, que ce soit au niveau de la structures,
de la rimes, du thème ... Nous pourrons tester la cohérence de ces rimes
automatiquement, en comparant le texte généré a des schémas de rimes par exemple, 
et de même pour la structure des vers. Nous pourrons aussi implémenter des protocoles 
d'analyse syntaxique et grammaticales qui s'assureront d'une bonne cohérence de 
nos générations.
    
### Test de créativité :  

Les paroles devront se montrer créative. Nous pourrions les comparer à des paroles
de chansons déjà existante, dans le même style de musique, pour pouvoir évalué 
leur originalité. On pourrait vérifier que les paroles de chanson possèdent un vocabulaire diversifié :
si les paroles de chansons que nous avons générés contiennent un grand éventail 
de vocabulaires, on estimera que ces paroles seront originales et créatif. Pour se faire,
nous devrons créer des ensembles de vocabulaires associés à chaque style de musiques,
et vérifier que les paroles n'ont pas que du vocabulaire très générique appartenant
à l'ensemble de vocabulaire du style de musique correspondant à celui des paroles.
Cependant, il faudra également s'assurer que les paroles de chansons sont bien générés
dans le bon style de musique, et qu'il n'empreinte pas le vocabulaire d'un autre
style.
    
### Test de similarité :  

Nous devrions mesurer la capacité du modèle à reproduire le plus fidèle au possible 
les paroles d'un style de musique. Nous lui fournirons un corpus d'entraînement pour qu'il
compare la similarité entre les paroles qu'il générera et le corpus, dans le même style. Nous pourrions 
fournir un corpus d'entraînement contenant des paroles classifié en fonction
du style de musique, puis vérifier que ce que nous avons généré appartient belle 
et bien à la classification auquel il est censé appartenir. Ces classifications 
correspondrait donc à des genres musicaux, et nous rangerions les paroles de chansons par 
rapport au style musicale auquel elles sont censés appartenir (ce qui reprend le point précédent).
    
    
### Lecture manuelle :  
    
Nous pourrons manuellement vérifier la qualité de génération des paroles après 
modification du code en plus de l'environnement CI, afin de s'assurer par nous 
même ce à quoi nous devons nous attendre.
    
### Intégration Continue :  

Intégration Continue (CI) : Pour garantir la qualité du projet, nous mettrons en place
un environnement de CI qui exécutera des tests automatisés à chaque modification du code.
Cela permettra de détecter rapidement l'impact de nos changements sur la qualité
de la génération  tout au long du développement.

## Originalité :  

Nous voulons que ce projet se distingue des autres dans ce domaine, notamment par l'utilisation
d'algorithme différents de ceux basés sur des réseaux de neurones : c'est pour cela que
nous voudrions utilisé un modèle basé sur des réseaux de Markov. Cette approche différente
des modèles traditionnels nous permettra d'avoir un projet à la fois riche, mais également
plus intéressant.

## Pourquoi ce n'est pas qu'un collage d'API ?  

Nous devrons assurer la conception et la mise en oeuvre de nos algorithmes nous même, sans passer
par l'utilisation d'API tier. En construisant notre propre solution, nous devrions
pouvoir ajuster le modèle en fonction de nos besoins.

## Calendrier  

### Jalon 1 - Fin Novembre ~ début Décembre : 
Collecte et prétraitement des données de paroles de chansons, mise en place de la base de données.
### Jalon 2 - Janvier : 
Exploration d'idée pour les algorithmes.
### Jalon 3 - Février ~ Mars : 
Implémentation des algorithmes pour la génération de paroles et tests.
### Jalon 4 - Avril ~ Mai : 
Entraînement du modèle, interface utilisateurs.
### Jalon 5 - Mai : 
Ajustements finaux, documentations.
