# Derivation of the Refracted Ray (Snell's Law in Vector Form)

![Refraction Diagram](refraction-diagram.svg)

## Initial Situation

An incident ray $\vec r$ hits a surface with normal vector $\vec n$. Both vectors are normalized:

$$
\|\vec r\| = \|\vec n\| = 1
$$

Convention: $\vec r$ points **towards** the surface (towards the point of impact), $\vec n$ points **away** from the point of impact into the half-space from which $\vec r$ originates.

The ratio of the refractive indices is

$$
\eta = \frac{n_1}{n_2}.
$$

We are looking for the refracted ray $\vec t$ with $\|\vec t\| = 1$.

---

## 1. Decomposition of the Incident Ray

We decompose $\vec r$ into a component parallel to the normal and a component parallel to the tangent:

$$
\vec r = \vec r_\perp + \vec r_\parallel
$$

### Normal Component ($\vec r_\perp$)

The projection of $\vec r$ onto $\vec n$ is

$$
\vec r_\perp = (\vec r \cdot \vec n)\,\vec n.
$$

### Angle of Incidence $\alpha$

The angle between the **backward** ray $-\vec r$ and the normal $\vec n$ is the angle of incidence $\alpha = i$:

$$
\cos\alpha \;=\; (-\vec r) \cdot \vec n \;=\; -\vec r \cdot \vec n \;=\; cosi.
$$

### Tangential Component ($\vec r_\parallel$)

$$
\vec r_\parallel = \vec r - \vec r_\perp = \vec r - (\vec r \cdot \vec n)\,\vec n.
$$

From $\|\vec r\|=1$ it follows for the magnitude of the tangential component:

$$
\|\vec r_\parallel\| = \sin\alpha.
$$

---

## 2. Snell's Law of Refraction

Snell's law of refraction is

$$
n_1 \sin\alpha = n_2 \sin\beta
\quad\Longleftrightarrow\quad
\eta\,\sin\alpha = \sin\beta.
$$

Thus, for the magnitudes of the tangential components:

$$
\|\vec t_\parallel\| = \sin\beta = \eta \sin\alpha = \eta \|\vec r_\parallel\|.
$$

Since $\vec t_\parallel$ points in the same direction as $\vec r_\parallel$, it follows directly that:

$$
\vec t_\parallel = \eta \vec r_\parallel.
$$

Substituting for $\vec r_\parallel$:

$$
\vec t_\parallel = \eta \,\vec r_\parallel = \eta\,\vec r - \eta\,(\vec r \cdot \vec n)\,\vec n.
$$

With $cosi = -\vec r \cdot \vec n$ this becomes

$$
\boxed{\; \vec t_\parallel = \eta\,\vec r + \eta cosi\,\vec n \;}
$$


---

## 3. Normal Component of $\vec t$

Since $\|\vec t\| = 1$ should hold, the magnitude of the normal component results from

$$
\cos\beta = \sqrt{1 - \sin^2\beta} = \sqrt{1 - \eta^2 \sin^2\alpha}.
$$

Because $\sin^2\alpha = 1 - \cos^2\alpha = 1 - cosi^2$, we can also write this as

$$
\cos\beta = \sqrt{1 - \eta^2\bigl(1 - cosi^2\bigr)} \;=:\; k.
$$

The normal component of $\vec t$ points **into** the surface, i.e., opposite to $\vec n$:

$$
\boxed{\; \vec t_\perp = -k\,\vec n \;}
$$

---

## 4. Assembling the Refracted Ray

$$
\vec t \;=\; \vec t_\parallel + \vec t_\perp
\;=\; \eta\,\vec r + \eta cosi\,\vec n - k\,\vec n
$$

$$
\boxed{\; \vec t \;=\; \eta\,\vec r + \bigl(\eta cosi - k\bigr)\,\vec n \;}
$$

with

$$
cosi = -\vec r \cdot \vec n,
\qquad
k = \sqrt{1 - \eta^2\bigl(1 - cosi^2\bigr)}.
$$

---

## Note: Total Internal Reflection

If the radicand becomes negative,

$$
1 - \eta^2\bigl(1 - cosi^2\bigr) < 0,
$$

no refracted ray exists — **total internal reflection** occurs. This can only happen if $\eta > 1$, i.e., during the transition from an optically denser to an optically thinner medium.
