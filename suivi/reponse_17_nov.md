# Réponses aux questions du journal du 17 Novembre.

## Rappel des questions :

TODO
- Sketch the main ideas behind the creation of texts for songs
- Sketch the main ideas behind generation of text that makes sense
- Define when two or more words "are coherent", i.e. make sense


### 1) Sketch the main ideas behind the creation of texts for songs


Créer un système de génération de paroles de chansons à partir de fichiers audio est intéressant
mais relativement complexe à faire. Pour se faire, on pourrait procéder de la façon suivante :

- On commence tout d'abord à récolter des données sur des fichiers audio pour déjà avoir quelques exemples et voir
ce qui va diffencier un son d'un autrE.
- On doit ensuite pouvoir analyser musicalement la musique, ce qui est un des points le plus important de ce projet.
Il faut comprendre comment fonctionne chaque fichier musicaux, illisible à l'oeil nu (par exemple, un fichier .wave 
possède toute ses informations dans son format (modèle générique de format Microsoft RIFF), avec les 12 premiers bytes
qui sont le format RIFF, les 24 bytes qui suivent permettent de stocker des informations sur les "particularités" de
la musique (ByteRate, SampleRate, Bits per sample ...) puis les bytes qui restent concernent la musique sous son
format brut). J'ai pris l'exemple de .wav, mais chaque format est différent, et il faudrait d'abord en premier temps
choisir un seul type de format pour travailler dessus, puis ensuite, permettre au programme de travailler sur d'autre
format.
En comprenant le fonctionnement du format de fichier, on pourrait par la suite en déduire les informations que nous souhaitons,
et récupérer les informations présentent dans les fichiers dont nous auront pour la suite.
- Après avoir récupérer les informations dans le soundfile, il faudrait pouvoir établir ce que c'est une mélodie, les récupérer
et en suite les traiter. Cela est possible, il y a des bibliothèques qui sont spécialisé dans le traitement
de fichier son, donc nous pourrions pouvoir faire une version tout à fait correct. 
- On pourrait aussi évalué les moment de "puissance" présente dans la musique, quand l'intensité sonore semble être la plus élevé.
Ceci couplé avec la mélodie pourrait nous aider à comprendre la structure harmonique du son. En général, il faudrait pouvoir
retrouver le tempo, la tonalité... toute ces informations qui peuvent definir une musique, et qui sont plus facile à comprendre
pour nous.

- Après avoir analysé la musique, il faut désormais pouvoir associer l'analyse musicale avec des thèmes, et à partir de ces thèmes nous 
devrons pouvoir générer des paroles correspondant au thème. Cette tâche est "relativement" facile pour un être humain, (si on écoute
un type beat avec une musique un peu molle, qui sonne triste, on peut se dire que le thème pourrait être la tristesse, la séparation...
mais un ordinateur ne peut pas le faire facilement). On pourrait essayer d'implémenter un réseau de neurones qui apprendrait des 
configurations d'analyse, qui les associerait avec des thèmes, ainsi de suite jusqu'à ce qu'il ait bon. 
- A partir de ces thèmes donc, il faudrait enfin pouvoir générer du texte. En se basant sur des travaux déjà existant dans ce domaine
cela devrait être possible de générer du texte en raccord avec le thème.
- Une bonne idée de test pour pouvoir évalué le programme, ce serait de pouvoir comparé des instrus de musiques (on retire les 
paroles de la chanson, mais on garde la musique instrumentale derrière), et on essaye de comparer ce que notre programme nous fournit
avec les paroles originales. Plus notre génération sera proche des paroles originales, on pourra estimer que l'approche est bonne. 
Bien sûr, il faudrait établir un système de scores pour savoir si notre génération est proche des paroles de bases, peut être
en regardant le thème des phrases ? les mots qui se retrouvent beaucoup entre les deux textes ?


### 2) Sketch the main ideas behind generation of text that makes sense

Pour générer des textes qui ont du sens, il faudrait déjà respecté les deux points
qui sont stipulés dans la question d'après. 

Le plus simple certainement, ce serait, en se basant sur 
des databases remplis de cas de phrases (projet Gutemberg par exemple), généré des séquences de mots
à partir de ses exemples, en fonction notamment du nombres de fois où on retrouve ses mots dans les même séquences
(Si un mot 'w' se retrouve beaucoup de fois avec le mot 'u', on peut estimé que ses deux mots donnent
du sens quand ils sont associés).

Sinon, sans utilisé de DB comme précédemment, cela nous semble relativement (très) compliqué, mais on peut se donner les
idées suivantes :

- Il faudrait premièrement récupérer tout les mots du dictionnaires dans une certaine langue, et les ranger par catégorie
grammaticale (déterminant, verbe, pronom personnel, nom commun ...)

- Ensuite, il faut établir toutes les règles syntaxique, grammaticale, sémantique ... de la langue en question.

- Enfin, il faut tenter des combinaisons de séquences de mots qui respecterait les règles précédemment stipulées, en utilisant les 
particularités des mots (les catégories précédemment créées). Il faut faire attention, car un mot peut avoir plusieurs fonctions.
On pourrait le voir sous forme de graphe, où on essaierait d'abord 2 branches, puis on tentera de les combiner avec
une autre paire de branches, jusqu'à arrivé à la profondeur souhaité. On récupererait les combinaisons de branches qui ont survécu
à la vérification des règles, et (en théorie) ces branches auraient du sens. Bien sûr, ce n'est pas parce que
la branche existe qu'elle aurait du sens pour nous, mais elle aurait du sens d'un point de vu réglementaire.

- On pourrait analysé ses branches, pour y récupérer les fréquences de mots qui se retrouvent souvent ensemble 
(dans une matrice d'occurence par exemple, qui augmenterait l'occurence entre deux mots à chaque fois qu'ils se retrouvent
dans la même branche). Ensuite, on pourrait déterminé un palier, en dessous de ce palier, on estimerait que ses mots
ne se retrouvent pas assez souvent ensemble pour qu'ils soient cohérents, et dans ce cas cela nous permettra d'élaguer
un peu plus de branches. On récuperera les branches qui se retrouvent avec un indice de cohérence relativement élevé, et
qui respecte les règles de la langue.

C'est un sujet compliqué, qui est encore en pleine expension à l'heure qu'il est, avec des technologies
comme GPT-3 ...

### 3) Define when two or more words "are coherent", i.e. make sense

On peut définir la cohérence de deux ou plusieurs mots par deux principaux aspects :

- Le respect des règles de la langue des mots :
Un mot appartient à un langage, et les langages sont définis par des règles (grammaticale,
orthographique ...). Pour que plusieurs mots soit cohérent, il faut donc tout d'abord que cette
suite de mot respecte toute les règles dont elle appartient.  Par exemple, "une bateau" ne peut pas
être cohérent, car elle ne respecte pas l'accord entre un déterminant masculin et un nom commun féminin.
Pour qu'une suite de mot soit cohérent, il faut déjà que les règles soient respectées. De même, tout
bêtement, si le mot n'appartient pas au langage, alors il peut difficilement être cohérent (sauf
si on autorise les emprunts de mots d'une langue à une autre).

- Le resultat de l'expression doit avoir du sens :
Dans ce contexte, le mot "sens" est difficile à définir, parce que comment peut-on dire si une phrase
à du sens ou non ? La définition la plus "simple" pour cela serait de voir si la suite de mots est
compréhensible pour le lecteur. L'être humain peut facilement dire si cela est compréhensible pour lui,
mais l'ordinateur n'est pas doté d'intelligence, il peut seulement analysé la suite de mots, voir s'il a
déjà rencontré auparavant un cas équivalent... En plus de l'aspect de règles que la suite de mots
doit respecté, il faudrait également que celle-ci soit remplis de mots qui sont liés "logiquement"
entre eux, pour que cela donne une expression compréhensible.

