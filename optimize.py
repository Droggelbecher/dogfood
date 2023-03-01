

import numpy as np
from scipy.optimize import minimize, LinearConstraint, linprog

values_names = [
    "Menge [g]",
    "Energie [kcal]",
    "Protein [g]",
    "Kalzium [mg]",
    "Phosphor [mg]",
    "Kupfer [mg]",
    "Zink [mg]",
    "Jod [ug]",
    "Vit. A [IE]",
    "Vit. D [IE]",
    "Vit. E [mg]"
]

values_targets = np.array([
    603, 1019.87, 52.21, 2110.5, 1608, 3.015, 30.15, 442.2, 1708.5, 201, 40.2
])
values_upper_bound_factors = np.array([
    1.5, 1.0, 2.5, 2, 2, 999., 10., 2.0, 7.0, 10.0, 3.0
])
assert values_upper_bound_factors.shape == values_targets.shape

ingredients_names = [
    "Rind, Muskel fettfrei",
    "Rind, Beinscheibe",
    "Rind, Herz",
    "Rind, Kopffleisch",
    "Rind, Pansen grün",
    "Rind, Blättermagen",
    "Kalb, Niere",
    "Huhn, Fleisch",
    "Hühnerhälse",
    "Apfel",
    "Banane",
    "Möhre",
    "Leinöl",
    "Lachsöl",
    "Kartoffel, gekocht",
    "Dinkelvollkornmehl",
    "Reis",
    "Kalbsleber",
    "Kalbsknochen",
    "Knochenmehl",
    "Dorsch-Lebertran",
    "Weizenkeimöl",
    "Seealgen-Mehl",
    "Eierschale",
]

ingredients_values = np.array([
    #   g,kcal, Pr,   Ca,   P,   Cu,  Zn, Jod, A,   D,   E
    [100., 125, 21,   10,  160,  0.2,   3,   3,  50,    0,    1],
    [100., 130, 21, 4842,  250,  0.1, 5.1,   0,   0,    0,    0],
    [100., 116, 17,    5,  210,  0.4, 2.0,   7,  30,    0,    1],
    [100., 290, 17,   10,  160,  0.2, 3.0,   3,  50,    0,    1],
    [100., 124, 20,  120,  130,  0.1, 1.5,   0,  30,    0,    1],
    [100., 104, 15,   90,   80,  0.2, 2.2,   0,   0,    0,    0],
    [100., 125, 17,  0.1,    2,  0.4, 2.4,   0, 700,    0,    0],
    [100., 117, 21,   15,  150,  0.2, 1.0,   3,  25,    0,    0],
    [100., 132, 17, 1726, 1099,  0.1, 3.8,   0,   0,    0,    0],
    [100.,  45,  0,    6,   11,  0.1, 0.1,   1,   0,    0,    1],
    [100.,  72,  1,    7,   23,  0.1, 0.2,   2,  28,    0,    0],
    [100.,  36,  1,   50,   35,  0.1, 0.4,   4,   0,    0,    2],
    [100., 805,  0,    0,    0,  0.0, 0.0,   0,   0,    0,   70],
    [100., 820,  0,    0,    0,  0.0, 0.0,   0,   0,    0,    0],
    [100.,  74,  2,   10,   60,  0.2, 0.3,   3,   3,    0,    0],
    [100., 282, 14,   24,  384,  0.5, 2.8,   0,   0,    0,    1],
    [100., 105,  2,    2,   38,  0.0, 0.4,   1,   0,    0,    0],

    #   g,kcal, Pr,   Ca,   P,   Cu,  Zn, Jod, A,   D,   E
    [1.,   1,  1,    0,    3, 0.05, 0.1,   0, 140,  0.6,    0],
    [1.,   2,  0,  138,   62, 0.01, 0.1,   0,   0,  0.0,    0],
    [1.,   0,  0,  300,  150, 0.00, 0.0,   0,   0,  0.0,    0],
    [1.,   9,  0,    0,    0, 0.00, 0.0,   0, 400, 40.0, 0.03],
    [1.,   8,  0,    0,    0, 0.00, 0.0,   0,   0,  0.0,    2],
    [1.,   3,  0,    0,    0, 0.00, 0.0, 400,   0,  0.0,    0],
    [1.,   0,  0,  330,    0, 0.00, 0.0,   0,   0,  0.0,    0],
], dtype=np.float64)

ingredients_values_normalized = ingredients_values / values_targets

n_ingredients, n_values = ingredients_values_normalized.shape

# TODO:
# Compute bounds & matrices for scipy.optimize.linprog
# ingredients_values_normalized @ x -> Lower bound of 1.0 (=negative upper bound)
# ingredients_values_normalized @ x -> Upper bound of some integer factors from the book
# equality bounds for fixing some parameters to zero if desired

#     Ax >= b
# <=> (-A)x <= -b

A_ub = np.concatenate([
    -ingredients_values_normalized,
    ingredients_values_normalized,
], axis=1)
b_ub = np.concatenate([
    -np.ones((n_values,)),
    values_upper_bound_factors
])

# "Cost" of ingredients, we want to minimize.
c = np.ones((n_ingredients * 2,))

# We receive the ingredients amounts 'x' twice
# (so we can apply both lower and upper bounds to it)
# This ensures these x's are the same
A_eq = np.concatenate([
    np.eye(n_ingredients),
    -np.eye(n_ingredients),
])
b_eq = np.zeros((n_ingredients,))

# ingredients_bounds = [
    # (0., None)
# ] * len(ingredients_names)

# def loss(amounts):
    # normalized_values = amounts.reshape((1, -1)) @ ingredients_values_normalized
    # # return np.sum((normalized_values - np.ones_like(normalized_values)) ** 2)

    # loss = np.sum(np.abs(normalized_values - np.ones_like(normalized_values)))
    # return loss

# x0 = np.array([0.0] * len(ingredients_names))

r = linprog(c, A_ub=A_ub.T, b_ub=b_ub)
print(r)

rxr = r.x.ravel()[:n_ingredients]

print()
for name, x, row in zip(ingredients_names, rx, ingredients_values, strict=True):
    amount = x * row[0]
    print(f"{amount:6.2f}g {name}")
print()

r_absolute = (rx.reshape((1, -1)) @ ingredients_values).ravel()
r_normalized = (rx.reshape((1, -1)) @ ingredients_values_normalized).ravel()

print()
for name, absolute, normalized in zip(values_names, r_absolute, r_normalized, strict=True):
    print(f"{name:20s} {absolute:7.2f} ({normalized*100:6.2f}%)")
print()

