

import numpy as np
from pulp import *
from more_itertools import one

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
    #   g,kcal, Pr,      Ca,   P,   Cu,  Zn, Jod, A,   D,   E
    603, 1019.87, 52.21, 2110.5, 1608, 3.015, 30.15, 442.2, 1708.5, 201, 40.2
])
values_upper_bound_factors = np.array([
    # g,kcal,  Pr,Ca, P,   Cu,  Zn, Jod, A,   D,   E
    2.0, 1.0, 2.5, 2, 2, 999., 10., 2.0, 7.0, 10.0, 3.0
])
assert values_upper_bound_factors.shape == values_targets.shape

ingredients_names = [
    "Rind_Muskel_fettfrei",
    "Rind_Beinscheibe",
    "Rind_Herz",
    "Rind_Kopffleisch",
    "Rind_Pansen_grün",
    "Rind_Blättermagen",
    "Kalb_Niere",
    "Huhn_Fleisch",
    "Hühnerhälse",
    "Apfel",
    "Banane",
    "Möhre",
    "Leinöl",
    "Lachsöl",
    "Kartoffel_gekocht",
    "Dinkelvollkornmehl",
    "Reis",
    "Kalbsleber",
    "Kalbsknochen",
    "Knochenmehl",
    "Dorschlebertran",
    "Weizenkeimöl",
    "Seealgenmehl",
    "Eierschale",
    "Zink_Tabletten",
    "Kupfer_Tabletten",
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
    [1.,   0,  0,    0,    0, 0.00, 1000.0,   0,   0,  0.0,    0],
    [1.,   0,  0,    0,    0, 1000.00, 0.0,   0,   0,  0.0,    0],
], dtype=np.float64)

#
# Input normalization
#

# Convert to per gram
ingredients_values[:, :] /= ingredients_values[:, :1]
# print(ingredients_values)

# Normalize to be relative to targets
ingredients_values_normalized = ingredients_values / values_targets
n_ingredients, n_values = ingredients_values_normalized.shape

model = LpProblem("BARF Problem", LpMinimize)
ingredient_g_vars = LpVariable.matrix("g", ingredients_names, cat="Continuous", lowBound=0)

def get_index(name):
    return ingredients_names.index(name)

def get_variable(name):
    return one(v for v in model.variables() if v.name == "g_" + name)

#
#
#

# Barfers Wellfood "Komplett-Mix Rind"
# https://www.barfers-wellfood.de/barf-shop/fertigbarf/barfers-complete/komplett-mix-gewolft

# g = np.zeros((n_ingredients,))
# g_day = values_targets[0]

# g[get_index("Rind_Muskel_fettfrei")] = 0.551 * g_day
# g[get_index("Rinderbrustbein")] = 0.118 * g_day
# g[get_index("Süßkartoffel")] = 0.111 * g_day
# g[get_index("Rinderleber")] = 0.047 * g_day
# g[get_index("Spinat")] = 0.037 * g_day
# g[get_index("Heidelbeere")] = 0.020 * g_day
# g[get_index("Rinderherz")] = 0.018 * g_day
# g[get_index("Rinderlunge")] = 0.018 * g_day
# g[get_index("Rindermilz")] = 0.018 * g_day
# g[get_index("Rinderniere")] = 0.018 * g_day
# g[get_index("Apfel")] = 0.015 * g_day
# g[get_index("Birne")] = 0.015 * g_day
# g[get_index("Leinöl")] = 0.004 * g_day
# g[get_index("Dorschlebertran")] = 0.003 * g_day
# g[get_index("Algenkalk")] = 0.003 * g_day
# g[get_index("Lachsöl")] = 0.003 * g_day
# g[get_index("Kokosraspeln")] = 0.003 * g_day
# g[get_index("Seealgenmehl")] = 0.001 * g_day


# Simply minimize sum of g
objective = lpSum(ingredient_g_vars)
model += objective

#
# Constraints
#

for expr, ub in zip(ingredients_values_normalized.T @ ingredient_g_vars,
                    values_upper_bound_factors, strict=True):
    model += expr >= 1.0
    model += expr <= ub

# Additional constraints

# model += get_variable("Rind_Muskel_fettfrei") >= 100.
# model += get_variable("Dinkelvollkornmehl") == 0.
model += get_variable("Huhn_Fleisch") == 0.
model += get_variable("Hühnerhälse") == 0.
# model += get_variable("Apfel") >= 50.

model += get_variable("Kalbsknochen") >= 0.02 * values_targets[0]
model += get_variable("Kalbsleber") >= 0.05 * values_targets[0]
model += get_variable("Dorschlebertran") >= 0.006 * values_targets[0]
model += get_variable("Seealgenmehl") >= 0.001 * values_targets[0]

#
# Optimize
#

print(model)
model.writeLP("barf.lp")

model.solve()
status = LpStatus[model.status]

#
# Output optimization results
#

print("Status", status)
print("Objective Value:", model.objective.value())

variables = [
    one(v for v in model.variables() if v.name == "g_" + name)
    for name in ingredients_names
]

for v, row in zip(variables, ingredients_values):
    if v.value() == 0.0:
        continue
    print(f"{v.value() * row[0]:8.4f} {v.name}")

rx = np.array([v.value() for v in variables])

r_absolute = (rx.reshape((1, -1)) @ ingredients_values).ravel()
r_normalized = (rx.reshape((1, -1)) @ ingredients_values_normalized).ravel()

print()
for name, absolute, normalized in zip(values_names, r_absolute, r_normalized, strict=True):
    print(f"{name:20s} {absolute:7.2f} ({normalized*100:6.2f}%)")
print()

