# 🛡️ RustShield v2

> **RustShield** est un système de détection d'intrusion (IDS) et un gestionnaire d'informations et d'événements de sécurité (SIEM) développé en Rust.
> Cette version 2 introduit une architecture agent-serveur robuste pour une surveillance distribuée, une détection comportementale avancée et une réponse automatisée aux menaces.

---

## ✨ Points clés

* 🔌 **Architecture agent-serveur distribuée** : collecte de logs sur plusieurs hôtes avec un serveur centralisé pour l’analyse.
* 📊 **Analyse avancée des logs** : détection de comportements anormaux via parsing et normalisation des journaux système / réseau.
* ⚡ **Réaction automatisée** : détection des menaces en temps réel avec des règles dynamiques, pouvant déclencher des actions prédéfinies.
* 🔧 **Système modulaire en Rust** : conçu pour être extensible, hautement performant et sécurisé.

---

## 🧰 Technologies utilisées

* 🦀 **Rust** : performance, sécurité mémoire et concurrence native.
* 📁 **JSON** : configuration et règles extensibles.
* 🔍 **File watcher** : détection des modifications en temps réel.
* 🧠 **Matching engine** : règles personnalisables pour identifier des patterns d’intrusion.

---

## 🚀 Mise en place

### 1. Cloner le dépôt

```bash
git clone git@github.com:thomas-bsn/RustShield.git
```

### 2. Configuration

Dans `src/libs/configs/config.json`, modifiez le champ `server_ip` :

* `localhost` pour un test local (agent + serveur sur la même machine)
* Une **IP privée** si test en réseau local
* Une **IP publique** (ouvrir le port `7331` en TCP côté serveur)

⚠️ Assurez-vous de modifier **ce fichier dans l’agent et le serveur**.

### 3. (Optionnel) Activer le test unitaire agent → serveur

Dans `src/main.rs` de l’**agent**, passez la variable suivante à `true` :

```rust
let test = true;
```

Cela enverra automatiquement un log normalisé au serveur pour valider le bon fonctionnement du flux.

### 4. Lancer les composants

Dans chaque répertoire (`agent` ou `server`), exécutez :

```bash
cargo run
```

### 5. Tester en conditions réelles

1. Lancez d’abord le **serveur**, puis l’**agent**.
2. Modifiez ou ajoutez un fichier dans `tests/logs/` côté agent.
3. L’agent détectera le changement, parsera la log, la normalisera et l’enverra.
4. Le serveur appliquera les règles de détection et affichera le résultat.

---

## 📁 Arborescence

```
.
├── agent/
│   └── src/
│       ├── main.rs
│       └── libs/
│           └── configs/
│               └── config.json
├── server/
│   └── src/
│       ├── main.rs
│       └── libs/
│           └── configs/
│               └── config.json
└── tests/
    └── logs/
```

---

## 👥 Auteurs

* **Thomas Boisson**
* **Axel Cochepin**

---

## 📌 TODO / Améliorations futures

* Intégration d’un dashboard CLI / web pour visualiser les logs
* Ajout de règles dynamiques via interface JSON/API
* Export vers formats standards : Syslog, CSV, ELK
* Dockerisation et orchestration : Docker Compose, Kubernetes
