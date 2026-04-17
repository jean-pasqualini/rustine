Très bonne question — c’est **le détail qui fait tout comprendre** 👇

---

## 🧠 Pourquoi on doit inverser (reverse) ?

Parce que :

> **on récupère les chiffres du plus petit au plus grand**

---

## 🔍 Exemple avec 13

On fait :

```text
13 % 2 = 1   ← bit le plus à droite
6  % 2 = 0
3  % 2 = 1
1  % 2 = 1   ← bit le plus à gauche
```

👉 dans l’ordre où tu les trouves :

```text
1 0 1 1
```

Mais ça correspond à :

```text
← gauche        droite →
  MSB           LSB
  1 1 0 1
```

Donc il faut inverser pour avoir :

```text
1101
```

---

## 🧩 Intuition simple

* `% 2` donne **le dernier bit** (LSB = rightmost)
* mais un nombre s’écrit **de gauche à droite** (MSB → LSB)

👉 donc tu lis **à l’envers**

---

## 📦 Analogie (base 10)

Imagine :

```text
345
```

Si tu fais :

* `% 10` → 5
* `% 10` → 4
* `% 10` → 3

Tu obtiens :

```text
5 4 3
```

👉 mais le vrai nombre est :

```text
3 4 5
```

Donc → reverse

---

## ⚙️ Version “physique”

Tu es en train de :

> démonter le nombre **par la droite**

Mais pour l’écrire correctement :

> il faut reconstruire **de gauche à droite**

---

## ⚡ Résumé

* `% base` → donne le **chiffre de droite**
* on les récupère dans le mauvais ordre
* donc → on **reverse à la fin**

---

## 💡 Astuce

Tu peux éviter le `reverse` en construisant à l’envers :

```rust
result.insert(0, bit);
```

Mais c’est plus lent.

---

Si tu veux, je peux te montrer une version **sans reverse et optimisée** (utilisée en bas niveau / alloc-free).
