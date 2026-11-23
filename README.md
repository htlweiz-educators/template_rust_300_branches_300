# Rust Programming Exercises

This repository contains a series of introductory exercises focusing on basic I/O, data types, control flow, and simple geometry calculations in Rust.

## Exercises

### 1. Generation Identification (`program00`)
Implement a program that prompts the user for their birth year and determines which generation they belong to.

**How to reproduce:**
```bash
echo "1990" | cargo run --bin program00
```

**Sample Output:**
```text
Please enter your year of birth: 1990
Congratulations, you are a Millenial.
```

### 2. Rectangle Fitting (`program01`)
Create a program that reads the dimensions of two rectangles and checks if the second rectangle fits inside the first.

**How to reproduce:**
```bash
echo -e "100\n50\n40\n30" | cargo run --bin program01
```

**Sample Output:**
```text
Please input parameters of the first rectangle.
Side a in mm: 100
Side b in mm: 50
Side a in mm: 40
Side b in mm: 30
The rectangle 40 mm x 30 mm fits in the rectangle 100 mm x 50 mm.
The size differs by 3400 mm².
```

### 3. Sphere in a Cube (`program02`)
Implement a program that checks if a sphere fits inside a cube.

**How to reproduce:**
```bash
echo -e "100\n40" | cargo run --bin program02
```

**Sample Output:**
```text
Please enter the side of a cube shaped box in mm: 100
Please enter the radius of a sphere in mm: 40
The sphere with an radius of 40 mm does fit in a cube shaped box with length of 100 mm.
In the box 731917.419 mm³ of air remain.
```

### 4. Sphere in a Box (`program03`)
Extend the previous exercise to check if a sphere fits inside a rectangular box.

**How to reproduce:**
```bash
echo -e "100\n100\n50\n20" | cargo run --bin program03
```

**Sample Output:**
```text
Please enter side a of a box in mm: 100
Please enter side a of b box in mm: 100
Please enter side a of c box in mm: 50
Please enter the radius of a sphere in mm: 20
The sphere with an radius of 20 mm does fit in a box of 100x100x50 mm.
In the box 466489.677 mm³ of air remain.
```

### 5. Box in a Box (`program04`)
Implement a program that checks if one rectangular box can fit inside another.

**How to reproduce:**
```bash
echo -e "100\n100\n100\n50\n50\n50" | cargo run --bin program04
```

**Sample Output:**
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

### 6. Area Calculator (`program05`)
Implement a program that calculates the area of different geometric shapes based on user selection.

**How to reproduce:**
```bash
echo -e "1\n50" | cargo run --bin program05
```

**Sample Output:**
```text
Choose a shape to calculate the area:
1) Circle
2) Rectangle
3) Triangle
Selection: 1
Enter the radius in mm: 50
The area of the circle is 7853.981 mm².
```

### 7. Travel Cost Calculator (`program06`)
Implement a pricing system for a travel journey based on the number of people.

**How to reproduce:**
```bash
echo "25" | cargo run --bin program06
```

**Sample Output:**
```text
Please enter the number of persons traveling: 25
The journey with 25 people costs you 6.50 EUR each. Plus a 5.00 EUR per bus for 2 busses. In total 172.50 EUR.
```

## Running the Programs

Each exercise is defined as a separate binary in `Cargo.toml`. You can run them using the following command:

```bash
cargo run --bin programXX
```
*(Replace `XX` with the program number, e.g., `program00`, `program01`, etc.)*

## Testing

The project includes an automated testing setup. You can run the tests using:

```bash
cargo test
```
