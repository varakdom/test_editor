
# Git workflow et organisation

Le GitFlow est une approche de gestion de branches pour le développement de logiciels qui offre une structure organisée et un flux de travail clair. Il facilite la collaboration entre les développeurs et simplifie la gestion des versions de notre projet.

Voici l'organisation générale des branches dans notre dépôt Github :

    Branche main (main) : Il s'agit de la branche principale où le code source est considéré comme étant stable. Cette branche contient le code le plus récent et le plus stable, et elle est utilisée pour déployer la version en production.

    Branche production (production) : Cette branche est un reflet de la branche main et contient le code qui est actuellement déployé en production. Elle sert de base pour les mises à jour et les correctifs.

    Branche développement (development) : Il s'agit de la branche de développement principal où les nouvelles fonctionnalités et les correctifs sont ajoutés. Les développeurs travaillent sur cette branche pour préparer le prochain cycle de publication.

Chaque fois qu'une nouvelle fonctionnalité (feature) ou un correctif (bugfix) est développé, il doit être créé comme une sous-branche de la branche développement :

    Branche de fonctionnalité (feature) : Ces branches sont créées à partir de la branche développement et sont utilisées pour développer de nouvelles fonctionnalités. Une fois la fonctionnalité terminée, la branche est fusionnée (merge) dans la branche développement. Le nom de ces branches doit être "feature/nouvelle-fonctionnalite".

    Branche de correctif (bugfix) : Ces branches sont également créées à partir de la branche développement et sont utilisées pour résoudre les problèmes identifiés. Une fois le problème résolu, la branche est fusionnée dans la branche développement. Le nom de ces branches doit être "bugfix/correctif-probleme".

Voici un exemple de flux de travail en utilisant GitFlow :

    Un développeur crée une nouvelle branche de fonctionnalité à partir de la branche développement.

    Le développeur travaille sur la nouvelle fonctionnalité et effectue des commits régulièrement.

    Une fois la fonctionnalité terminée, le développeur soumet une demande de fusion (pull request) pour fusionner la branche de fonctionnalité dans la branche développement.

    Le chef de groupe examine les modifications apportées et les valide.

    La branche de fonctionnalité est fusionnée dans la branche développement et est supprimée.

    Lorsqu'un ensemble de fonctionnalités est prêt à être déployé en production, la branche développement est fusionnée dans la branche main.

    La branche main est ensuite fusionnée dans la branche production, et le code est déployé en production.

    Les mises à jour et les correctifs sont gérés de la même manière que les nouvelles fonctionnalités, en créant des branches spécifiques à partir de la branche développement.
