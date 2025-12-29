# 🌀 PyRand

**Rust-powered random generation for Python**
Blazing-fast RNG utilities with parallel computation and distribution support. ⚡🐍💨

---

## 🚀 Features

* 🎲 Generate random integers, floats, and arrays
* 🔀 Shuffle lists or sample subsets
* ⚡ Parallelized operations with Rayon
* 🧩 Multiple distributions: uniform, normal, etc.
* 🔧 Fully compatible with Python 3.13+ and PyO3 forward-compatibility
* ℹ️ Is fully typed.

---

## 💿 Installation

```powershell
# From dist/ folder
python -m pip install dist\pyrand-x.x.x-cp3xx-cp3xx-win_amd64.whl --force-reinstall
```

Or build from source:

```powershell
.\build.ps1
```

---

## 📝 Example Usage

```python
import pyrand

# Seed RNG
pyrand.set_seed(42)

# Single random number
x = pyrand.randfloat()
print("Random float:", x)

# Random integer
y = pyrand.randint(1, 100)
print("Random int:", y)

# Random array
arr = pyrand.rand_array(5, 1.0, 10.0)
print("Random array:", arr)

# Shuffle list
lst = [1, 2, 3, 4, 5]
pyrand.shuffle(lst)
print("Shuffled:", lst)
```

---

## 📦 Available Functions

| Function                   | Description                       |
| -------------------------- | --------------------------------- |
| `set_seed(seed)`           | Set RNG seed for reproducibility  |
| `randfloat()`                 | Random float [0, 1)               |
| `randint(a, b)`            | Random integer between a and b    |
| `rand_array(n, low, high)` | Generate array of n random floats |
| `uniform(low, high)`       | Single uniform random float       |
| `choice(seq)`              | Pick a random element             |
| `sample(seq, k)`           | Random sample of k elements       |
| `shuffle(seq)`             | Shuffle in-place                  |

---

## 🏗️ Requirements

* Python 3.10+
* Rust toolchain
* Maturin
* PyO3

---

## 🔧 Build & Development

```powershell
.\build.ps1
```

* Wheel gets copied to `dist/`
* Installed automatically to your current Python

---

## 💡 Notes

* Parallel random generation is backed by [Rayon](https://github.com/rayon-rs/rayon)
* Designed to replace slow Python loops in heavy random computations

---

## ⚖️ License

MIT © Soumalya Das
