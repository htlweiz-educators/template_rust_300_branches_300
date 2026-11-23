# Rust-Programmierübungen

Dieses Repository enthält eine Reihe von Einführungsübungen, die sich auf grundlegende E/A, Datentypen, Kontrollfluss und einfache Geometrieberechnungen in Rust konzentrieren.

## Übungen

### 1. Generationenidentifikation (`program00`)
Implementieren Sie ein Programm, das den Benutzer nach seinem Geburtsjahr fragt und bestimmt, zu welcher Generation er gehört.

**So reproduzieren Sie es:**
```bash
echo "1990" | cargo run --bin program00
```

**Beispielhafte Ausgabe:**
```text
Please enter your year of birth: 1990
Congratulations, you are a Millenial.
```

### 2. Rechteckpassform (`program01`)
Erstellen Sie ein Programm, das die Abmessungen von zwei Rechtecken liest und prüft, ob das zweite Rechteck in das erste passt.

**So reproduzieren Sie es:**
```bash
echo -e "100\n50\n40\n30" | cargo run --bin program01
```

**Beispielhafte Ausgabe:**
```text
Please input parameters of the first rectangle.
Side a in mm: 100
Side b in mm: 50
Side a in mm: 40
Side b in mm: 30
The rectangle 40 mm x 30 mm fits in the rectangle 100 mm x 50 mm.
The size differs by 3400 mm².
```

### 3. Kugel in einem Würfel (`program02`)
Implementieren Sie ein Programm, das prüft, ob eine Kugel in einen Würfel passt.

**So reproduzieren Sie es:**
```bash
echo -e "100\n40" | cargo run --bin program02
```

**Beispielhafte Ausgabe:**
```text
Please enter the side of a cube shaped box in mm: 100
Please enter the radius of a sphere in mm: 40
The sphere with an radius of 40 mm does fit in a cube shaped box with length of 100 mm.
In the box 731917.419 mm³ of air remain.
```

### 4. Kugel in einer Schachtel (`program03`)
Erweitern Sie die vorherige Übung, um zu prüfen, ob eine Kugel in eine rechteckige Schachtel passt.

**So reproduzieren Sie es:**
```bash
echo -e "100\n100\n50\n20" | cargo run --bin program03
```

**Beispielhafte Ausgabe:**
```text
Please enter side a of a box in mm: 100
Please enter side a of b box in mm: 100
Please enter side a of c box in mm: 50
Please enter the radius of a sphere in mm: 20
The sphere with an radius of 20 mm does fit in a box of 100x100x50 mm.
In the box 466489.677 mm³ of air remain.
```

### 5. Schachtel in einer Schachtel (`program04`)
Implementieren Sie ein Programm, das prüft, ob eine rechteckige Schachtel in eine andere passt.

**So reproduzieren Sie es:**
```bash
echo -e "100\n100\n100\n50\n50\n50" | cargo run --bin program04
```

**Beispielhafte Ausgabe:**
```text
Please input parameters of the first box.
a in mm: 100
b in mm: 100
c in mm: 100
Please input parameters of the second box.
a in mm: 50
b in mm: 50
c in mm: 50
The box of 50x50x50 mm does fit in the box of 100x100x100 mm.
In the box 875000 mm³ of air remains.
```

### 6. Flächenberechner (`program05`)
Implementieren Sie ein Programm, das die Fläche verschiedener geometrischer Formen basierend auf der Benutzerauswahl berechnet.

**So reproduzieren Sie es:**
```bash
echo -e "1\n50" | cargo run --bin program05
```

**Beispielhafte Ausgabe:**
```text
Choose a shape to calculate the area:
1) Circle
2) Rectangle
3) Triangle
Selection: 1
Enter the radius in mm: 50
The area of the circle is 7853.981 mm².
```

### 7. Reisekostenrechner (`program06`)
Implementieren Sie ein Preissystem für eine Reise basierend auf der Anzahl der Personen.

**So reproduzieren Sie es:**
```bash
echo "25" | cargo run --bin program06
```

**Beispielhafte Ausgabe:**
```text
Please enter the number of persons traveling: 25
The journey with 25 people costs you 6.50 EUR each. Plus a 5.00 EUR per bus for 2 busses. In total 172.50 EUR.
```

## Ausführung der Programme

Jede Übung ist als separates Binary in `Cargo.toml` definiert. Sie können sie mit dem folgenden Befehl ausführen:

```bash
cargo run --bin programXX
```

*(Ersetzen Sie `XX` durch die Programmnummer, z. B. `program00`, `program01`, usw.)*

## Testen

Das Projekt enthält ein automatisiertes Testsetup. Sie können die Tests mit folgendem Befehl ausführen:

```bash
cargo test
```
