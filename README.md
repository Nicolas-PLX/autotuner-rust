# Génération automatique de paroles de chansons à l'aide d'algorithme d'intelligence artificielle  

## À quel problème répond-il ?  

Notre projet répond au problème de la création automatisée de paroles de chansons
dans différent style musicaux, en utilisant des algorithmes et de la modélisation
de données.

## Objectifs  

### Objectif Principal :  

L'objectif principal de notre projet est de développer un générateur automatique
de paroles de chansons étant capable de produire des paroles originales, cohérentes, et 
dans différents styles de musiques. Les paroles de chansons seront générés à partir
d'un type de musique  défini par l'utilisateur. Ce style de musique devrait
appartenir à une sélection prédéfini que notre programme sera capable de traiter. Nous
souhaitons utilisé des chaînes de Markov pour modéliser la structure et la séquence
des mots dans les paroles de chansons. Les chaînes de Markov permettent de prédire
un mot en fonction des mots précédents, à l'aide de probabilités. Nous souhaitons explorer
ceci pour les utiliser dans notre projet, mais nous nous réservons le droit de changer
d'algorithmes si le besoin s'en fait sentir.

Nous comptons réaliser ce projet en python, langage commun et souvent utilisé pour l'IA.


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


