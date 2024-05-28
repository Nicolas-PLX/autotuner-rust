# Création d'un logiciel d'autotune
## À quel problème répond-il ?  

Ce projet vise à résoudre le problème de correction de la hauteur des notes dans des enregistrements audio, permettant à l'utilisateur d'améliorer leur performances musicales.
L'utilisateur fournit un enregistrement de sa voix en acapella, et le logiciel produit une version de l'enregistrement en corrigeant la justesse du chant de l'utilisateur, pour une tonalité précise (telle qu'une gamme définie à l'avance).

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

## Testabilité
Le projet peut être testé en utilisant les protocoles suivants :
- Donner des audios dont on connait déjà les notes (sous forme de midi par exemple) et voir si nos algorithmes de pitch tracking arrivent à précisémment retrouver ces notes
- Utiliser nos algorithmes de pitch tracking sur l'audio produit par le logiciel pour s'assurer que les notes produites sont bien les notes voulues

 On peut automatiser ce genre de testes pour CI/CD.

- On pourrait également réaliser des tests manuels sur les audios produits pour s'assurer que l'audio reste compréhensible et similaire a l'enregistrement de base.

## Originalité du Projet

Ce projet se distingue par sa création complète d'un logiciel Auto-Tune, développant des algorithmes personnalisés plutôt que de s'appuyer sur des API existantes. L'originalité réside dans la conception et l'implémentation d'un traitement vocal avancé, adapté aux besoins spécifiques du projet.

## Pourquoi ce n’est pas qu’un collage d’API ?

Contrairement à l'utilisation d'API existantes, ce projet vise à offrir une solution sur mesure en développant des algorithmes spécifiques de correction de hauteur. Cela permet un contrôle plus fin et une adaptation aux exigences particulières de l'application.

## Calendrier/Jalons

1. **Décembre :** Création du modèle de données
  
2. **Janvier :** Réalisation de la partie pitch tracking

3. **Février :** Réalisation de la partie pitch correction

4. **Mars :** Création de l'interface utilisateur et mise en place des tests.

5. **Avril :** Optimisation des performances et exploration des objectifs supplémentaires.
