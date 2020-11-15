# Phasor.js

[![Coverage Status](https://codecov.io/gh/brunocodutra/phasor.js/branch/master/graph/badge.svg)](https://codecov.io/gh/brunocodutra/phasor.js)

Phasor.js is a wasm (via Rust) implementation of complex numbers that strives to yield correct results.

## Example

```.js
import {i, rect} from 'phasor.js';

i(Math.PI).exp().add(rect(1)); // approximately rect(0);
```

## Motivation

Phasor.js was originally developed as part of [Steady] to provide the core
complex algebra primitives required in the implementation of algorithms
that solve electrical circuits.

While it wasn't [Steady]'s goal to implement complex numbers from scratch,
circumstances proved it necessary given the fact that, at the time (Feb 2018), 
no alternative library available on NPM was able to correctly handle edge 
cases such as complex numbers with infinite magnitude.

For example, all four alternatives tested<sup>[[a]][[b]][[c]][[d]]</sup>
compute `∞i × -∞i` to `NaN` instead of `∞` and three of them also generally
fail to prevent overflow/underflow even in trivial expressions such as
`1E200i / 1E200i` and `1E-200i / 1E-200i`.

Phasor.js on the other hand successfully passes all of the following
assertions.

```.js
i(Infinity).mul(i(-Infinity)).ulpsEq(rect(Infinity));
i(1E200).div(i(1E200)).ulpsEq(rect(1));
i(1E-200).div(i(1E-200)).ulpsEq(rect(1));
```

In 2020, Phasor.js was re-implemented from scratch in Rust.

## API Reference

#### rect(re, im = 0)

Constructs a complex number given it's real and imaginary parts.

#### polar(mag, ang = 0)

Constructs a complex number given it's magnitude and angle.

#### i(im = 0)

Constructs a purely imaginary number.

> Example:
```{.js}
i(42).ulpsEq(rect(0, 42));
```

#### p.real()

Extracts the real part of a complex number.

> Example:
```{.js}
rect(3).real() === 3;
rect(3, 4).real() === 3;
```

#### p.imag()

Extracts the imaginary part of a complex number.

> Example:
```{.js}
rect(3).real() === 0;
rect(3, 4).imag() === 4;
```

#### p.norm()

Extracts the magnitude of a complex number.

> Example:
```{.js}
rect(3, 4).norm() === 5;
```

#### p.angle()

Extracts the angle of a complex number.

> Example:
```{.js}
rect(3, 4).angle() === Math.atan2(4, 3);
```

#### absDiffEq(c1, c2, e = Number.EPSILON)

Compares two complex numbers for approximate equality, optionally taking
a positive residue.

#### relativeEq(c1, c2, e = Number.EPSILON, rel = Number.EPSILON)

Compares two complex numbers for approximate equality, optionally taking
a positive residue and a maximum relative distance.

#### ulpsEq(c1, c2, e = Number.EPSILON, ulps = 4)

Compares two complex numbers for approximate equality, optionally taking
a positive residue and the maximum distance of [Units in the Last Place][ulps].

#### p.add(q)

Computes the addition of two complex numbers.

> Example:
```{.js}
rect(3).add(rect(0, 4)).ulpsEq(rect(3, 4));
```

#### p.sub(q)

Computes the subtraction of two complex numbers.

> Example:
```{.js}
rect(3).sub(rect(0, 4)).ulpsEq(rect(3, -4));
```

#### p.mul(q)

Computes the multiplication of two complex numbers.

> Example:
```{.js}
rect(3).mul(rect(0, 4)).ulpsEq(i(12));
```

#### p.div(q)

Computes the division of two complex numbers.

> Example:
```{.js}
rect(3).div(rect(0, 4)).ulpsEq(i(-0.75));
```

#### p.neg()

Computes the opposite of a complex number.

> Example:
```{.js}
rect(3, 4).neg().ulpsEq(rect(-3, -4));
```

#### p.conj()

Computes the conjugate of a complex number.

> Example:
```{.js}
rect(3, 4).conj().ulpsEq(rect(3, -4));
```

#### p.recip()

Computes the reciprocal of a complex number.

> Example:
```{.js}
rect(3, 4).recip().ulpsEq(rect(3 / 25, -4 / 25));
```

#### p.exp()

Computes the exponential of a complex number.

> Example:
```{.js}
rect(3, 4).exp().ulpsEq(polar(Math.exp(3), 4));
```

#### p.ln()

Computes the principal natural logarithm of a complex number.

> Example:
```{.js}
rect(3, 4).ln().ulpsEq(rect(Math.log(5), Math.atan(4 / 3)));
```

#### p.log()

Computes the principal logarithm of a complex number to an arbitrary base.

> Example:
```{.js}
rect(3, 4).log(10).ulpsEq(rect(Math.log10(5), Math.atan(4 / 3) / Math.log(10)));
```

#### p.sinh()

Computes the hyperbolic sine of a complex number.

> Example:
```{.js}
const p = rect(3, 4);
const q = polar(
    -Math.hypot(Math.sinh(3), Math.sin(4)),
    Math.atan2(Math.tan(4), Math.tanh(3)),
);

p.sinh().ulpsEq(q);
```

#### p.cosh()

Computes the hyperbolic cosine of a complex number.

> Example:
```{.js}
const p = rect(3, 4);
const q = polar(
    -Math.hypot(Math.sinh(3), Math.cos(4)),
    Math.atan2(Math.tan(4), 1 / Math.tanh(3)),
);

p.cosh().ulpsEq(q);
```

#### p.isNaN()

Returns `true` if either the imaginary or real part (or both) of a complex number is `NaN`.

> Example:
```{.js}
console.assert(!i(0).isNaN());
console.assert(!i(1).isNaN());
console.assert(!i(1E-315).isNaN())
console.assert(!i(Infinity).isNaN());
console.assert(i(NaN).isNaN());
```

#### p.isInfinite()

Returns `true` if the magnitude of a complex number is `Infinity`.

> Example:
```{.js}
console.assert(!i(0).isInfinite());
console.assert(!i(1).isInfinite());
console.assert(!i(1E-315).isInfinite())
console.assert(i(Infinity).isInfinite());
console.assert(!i(NaN).isInfinite());
```

#### p.isFinite()

Returns `true` if a complex number is neither _NaN_ nor _infinite_.

> Example:
```{.js}
console.assert(i(0).isFinite());
console.assert(i(1).isFinite());
console.assert(i(1E-315).isFinite())
console.assert(!i(Infinity).isFinite());
console.assert(!i(NaN).isFinite());
```

#### p.isZero()

Returns `true` if the magnitude of a complex number is zero.

> Example:
```{.js}
console.assert(i(0).isZero());
console.assert(!i(1).isZero());
console.assert(!i(1E-315).isZero())
console.assert(!i(Infinity).isZero());
console.assert(!i(NaN).isZero());
```

#### p.isSubnormal()

Returns `true` if the magnitude of a complex number is [subnormal].

> Example:
```{.js}
console.assert(!i(0).isSubnormal());
console.assert(!i(1).isSubnormal());
console.assert(i(1E-315).isSubnormal())
console.assert(!i(Infinity).isSubnormal());
console.assert(!i(NaN).isSubnormal());
```

#### p.isNormal()

Returns `true` if a complex number is not _NaN_, _infinite_, _zero_, or _subnormal_.

> Example:
```{.js}
console.assert(!i(0).isNormal());
console.assert(i(1).isNormal());
console.assert(!i(1E-315).isNormal())
console.assert(!i(Infinity).isNormal());
console.assert(!i(NaN).isNormal());
```

#### p.isReal()

Returns `true` if a complex number is purely real.

> Example:
```{.js}
console.assert(rect(3).isReal());
console.assert(!rect(0, 4).isReal());
console.assert(!rect(3, 4).isReal());
```

#### p.isImaginary()

Returns `true` if a complex number is purely imaginary.

> Example:
```{.js}
console.assert(!rect(3).isImaginary());
console.assert(rect(0, 4).isImaginary());
console.assert(!rect(3, 4).isImaginary());
```

## Under the Hood

Complex numbers are represented under the hood by their magnitude and the
tangent of its angle, which might seem unusual, but it's one of the main
reasons why Phasor.js is able to yield numerically correct results.

The tangent of an angle has many advantages from the point of view of
numerical algorithms than angles expressed in radians, the most important
one being the fact that it makes it possible to implement most algorithms,
in particular the four fundamental operations (add, sub, mul and div),
without relying on any trigonometric function, which are highly non-linear
and a major source of numerical error.

Phasor.js essentially takes advantage of the following two
trigonometric identities that rely on the less non-linear square root
function:

```
cos(tan(α)) ≡ 1 / sqrt(1 + α^2)
sin(tan(α)) ≡ sign(α) / sqrt(1 + 1 / α^2)

```

These identities are particularly convenient, because those square roots in
the denominators can take advantage of the [hypot] algorithm, which efficiently avoids unnecessary over/underflows.

[Steady]:       https://brunocodutra.github.io/steady/
[a]:            https://www.npmjs.com/package/complexjs
[b]:            https://www.npmjs.com/package/complex-numbers
[c]:            https://www.npmjs.com/package/complex
[d]:            https://www.npmjs.com/package/complex-js
[hypot]:        https://en.wikipedia.org/wiki/Hypot
[subnormal]:    https://en.wikipedia.org/wiki/Denormal_number
[ulps]:         https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/  
