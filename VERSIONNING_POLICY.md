
# Politique de versionnement

Le versionnement sémantique (SemVer) est un système de numérotation de versions de logiciels qui attribue des numéros de version significatifs en fonction du type de modifications apportées. Il est basé sur un ensemble de règles et de conventions pour faciliter la communication et la gestion des dépendances entre les développeurs et les utilisateurs. SemVer utilise une chaîne de trois éléments séparés par des points pour représenter les numéros de version : MAJEUR.MINOR.CORRECTIF (par exemple, 2.5.0).

Voici une description détaillée de chaque élément:

    MAJEUR: Cet élément représente les changements incompatibles avec les versions précédentes. L'augmentation du numéro de version majeure signifie qu'il y a des modifications significatives dans le logiciel qui peuvent entraîner des problèmes de compatibilité avec les versions antérieures. Cela peut inclure des changements dans les interfaces de programmation d'applications (API), des fonctionnalités supprimées ou des modifications dans le comportement du logiciel. Lorsque vous augmentez le numéro de version majeure, les numéros mineur et correctif sont réinitialisés à zéro.

    MINOR: Cet élément représente l'ajout de nouvelles fonctionnalités qui sont compatibles avec les versions précédentes. Les modifications mineures n'entraînent pas de problèmes de compatibilité et peuvent être utilisées sans modifier le code existant. L'augmentation du numéro de version mineure signifie que de nouvelles fonctionnalités ont été ajoutées, mais que le logiciel reste compatible avec les versions précédentes. Lorsque vous augmentez le numéro de version mineure, le numéro de correctif est réinitialisé à zéro.

    CORRECTIF: Cet élément représente les corrections de bugs et autres modifications mineures qui n'affectent pas la compatibilité avec les versions précédentes. Les correctifs sont des changements qui résolvent des problèmes, améliorent la performance ou corrigent des erreurs sans ajouter de nouvelles fonctionnalités ni modifier le comportement du logiciel. L'augmentation du numéro de correctif indique que des problèmes ont été résolus, mais que le logiciel reste compatible avec les versions précédentes.

# Changelog

Le Changelog est un fichier de documentation (CHANGELOG.md) qui contient un journal des modifications apportées à un projet logiciel au fil du temps. Il permet aux développeurs et aux utilisateurs de suivre les nouvelles fonctionnalités, les corrections de bugs et les modifications mineures qui ont été effectuées au cours de l'évolution du logiciel. Un Changelog bien entretenu aide à communiquer clairement les changements entre les versions et facilite la gestion des dépendances pour les utilisateurs et les développeurs.
Structurez le contenu : Organisez le Changelog avec des sections claires pour chaque version, en commençant par la version la plus récente en haut. Incluez les informations suivantes pour chaque version :

    Numéro de version : Utilisez le modèle de versionnement sémantique (SemVer) pour indiquer clairement les modifications apportées à chaque version (MAJEUR.MINOR.CORRECTIF).

    Date de publication : Indiquez la date de publication de chaque version pour aider les utilisateurs à comprendre l'historique des modifications.

    Liste des modifications : Fournissez une liste des modifications apportées dans chaque version, en incluant les nouvelles fonctionnalités, les corrections de bugs et les modifications mineures.

    Utilisez un format lisible : Présentez les informations de manière claire et lisible, en utilisant des listes à puces, des en-têtes et des styles de texte pour faciliter la lecture et la compréhension.

    Soyez concis et informatif : Décrivez les modifications de manière concise et précise, en évitant les détails techniques inutiles. Les utilisateurs doivent pouvoir comprendre rapidement les changements apportés sans se référer au code source.

## Exemple de changelog

## [1.1.0] - 2023-04-30

### Ajouté

- Nouvelle fonctionnalité "Recherche avancée" permettant aux utilisateurs de filtrer et de trier les résultats de recherche.
- Bouton "Partager" pour partager le contenu sur les réseaux sociaux.

### Modifié

- Amélioration des performances de chargement des images sur la page d'accueil.
- Mise à jour de la bibliothèque de dépendance XYZ à la version 2.1.0

### Corrigé

- Correction d'un bug empêchant le formulaire de connexion de s'afficher correctement sur les appareils mobiles.
- Correction d'un problème de compatibilité avec le navigateur Google Chrome.

## [1.0.1] - 2023-04-15

### Corrigé

- Correction d'un bug provoquant un plantage de l'application lors de l'ouverture d'un fichier spécifique.
- Correction d'un problème d'affichage des caractères spéciaux dans les titres.

## [1.0.0] - 2023-04-01

### Ajouté

- Mise en place de la fonctionnalité de gestion des utilisateurs (inscription, connexion, déconnexion).
- Création de la page d'accueil avec une liste des articles disponibles.
- Implémentation de la fonctionnalité de recherche d'articles par mots-clés.
- Intégration de la fonctionnalité d'ajout et de suppression d'articles pour les administrateurs.

### Modifié

- Refonte de l'interface utilisateur pour améliorer l'ergonomie et le design.
- Optimisation des performances de chargement des pages.
