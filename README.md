# NgPortUnlocker

<img alt="Un magnifique logo fait par ChatGPT" src="ngportunlocker.png" title="NgPortUnlocker" width="100"/>

Ce projet permet de "demander" au pare-feu de NationsGlory de débloquer l'accès aux serveurs java.
En effet, depuis le 13 novembre 2024, les mesures de sécurité de NationsGlory ont été renforcées et il n'est désormais
plus possible de communiquer directement avec les serveurs java. Cela a été réalisé afin de limiter les attaques DDoS.

> ⚠️ **Attention :** Il vous revient de sécuriser l'accès à l'api exposée. Si celle-ci est utilisé à des fins malveillantes, vous pourriez avoir quelques problèmes...

## Comment ça marche ?

Ce projet émule le fonctionnement du client NationsGlory afin d'obtenir les accès.
POur cela, il se connecte au socket qui permet d'ouvrir les ports.
Il est donc nécessaire de s'authentifier avec un compte valide.

## Configuration

Afin de faire fonctionner le projet, il est obligatoire de renseigner les informations de connexion.
Pour cela, créer un fichier `config.toml` au même niveau que l'exécutable.

```toml
auth_string = "<token>" # Votre token d'authentification
address = "127.0.0.1" # L'adresse sur laquelle doit être ouvert le serveur
port = 8080 # Le port sur lequel doit être ouvert le serveur
```

Concernant le token d'authentification, il est composé de 3 parties :
`<pseudo>##<session_token>##<hash>`
- `<pseudo>` : Votre pseudo
- `<session_token>` : Récupérable depuis le fichier `%AppData%/NationsGlory/config.json`
- `<hash>` : Il est calculé automatiquement par le jeu sur la base de différents paramètres. Mais doit être le même que celui utilisé lors de la connexion.

Au vu de la difficulté de récupération du hash, je conseille personnellement d'utiliser un intercepteur de packet comme wireshark afin de l'obtenir. \
Pour cela, lancer votre intercepteur au lancement du jeu et chercher le bon packet.
Pour vous aider, vous pouvez utiliser le filtre suivant et le packet sera parmi les premiers : 
```bash
ip.dst == 185.152.25.81 && tcp.port == 59001
```

## Utilisation
Une fois le serveur lancé, afin de débloquer le port du serveur **pour l'ip courante**, il suffit de faire une requête GET sur le serveur.

```bash
curl http://<address>:<port>/connect/<server>
```

A l'heure actuelle, voici les serveurs recensés : "blue", "orange", "yellow", "white", "black", "cyan", "lime",
"coral", "pink", "purple", "green", "red", "accueil", "hub",
"freebuild1", "freebuild2", "freebuild3"

Voici les ports associés (L'ip étant toujours, à l'heure actuelle : **185.152.25.81**) :
- blue: 26012
- orange: 26008
- yellow: 26011
- white: 26009
- black: 26000
- cyan: 26004
- lime: 26005
- coral: 26002
- pink: 26001
- purple: 26003
- green: 26006
- red: 26007
- accueil (serveur tutoriel): 12333
- hub: 12555
- freebuild1: 17123
- freebuild2: 17124
- freebuild3: 17126