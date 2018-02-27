# Phasor.js

[![Build Status](https://travis-ci.org/brunocodutra/phasor.js.svg?branch=master)](https://travis-ci.org/brunocodutra/phasor.js)
[![Coverage Status](https://coveralls.io/repos/github/brunocodutra/phasor.js/badge.svg?branch=master)](https://coveralls.io/github/brunocodutra/phasor.js?branch=master)
[![Greenkeeper badge](https://badges.greenkeeper.io/brunocodutra/phasor.js.svg)](https://greenkeeper.io/)

Phasor.js is a tiny implementation of complex numbers that strives to
yield correct results.

## Example

#### Node

```.js
import {i, rect, add, exp} from 'phasor.js';

add(exp(i(Math.PI)), rect(1)); // approximately rect(0);
```

#### Browser

```.html
<script src="http://unpkg.com/phasor.js"></script>

...

<script>
  add(exp(i(Math.PI)), rect(1)); // approximately rect(0);
</script>
```

## Motivation

Phasor.js was originally developed as part of [Steady] to provide the core
complex algebra primitives required in the implementation of algorithms
that solve electrical circuits.

While it wasn't [Steady]'s goal to implement complex numbers from scratch,
circumstances proved it necessary given the fact that, as of the time of
this writing (Feb 2018), no alternative library available on NPM was able to
correctly handle edge cases such as complex numbers with infinite magnitude.

For example, all four alternatives tested<sup>[[a]][[b]][[c]][[d]]</sup>
compute `∞i × -∞i` to `NaN` instead of `∞` and three of them also generally
fail to prevent overflow/underflow even in trivial expressions such as
`1E200i / 1E200i` and `1E-200i / 1E-200i`.

Phasor.js on the other hand successfully passes all of the following
assertions.

```.js
equal(mul(i(Infinity), i(-Infinity)), rect(Infinity));
equal(div(i(1E200), i(1E200)), rect(1));
equal(div(i(1E-200), i(1E-200)), rect(1));
```

## API Reference

#### rect(re, im = 0)

Constructs a complex number given it's real and imaginary parts.

#### polar(mag, ang = 0)

Constructs a complex number given it's magnitude and angle.

#### i(im = 0)

Constructs a purely imaginary number.

> Example:
```{.js}
equal(i(42), rect(0, 42));
```

#### real(c)

Extracts the real part of a complex number.

> Example:
```{.js}
real(rect(3, 4)) === 3;
```

#### imag(c)

Extracts the imaginary part of a complex number.

> Example:
```{.js}
imag(rect(3, 4)) === 4;
```

#### norm(c)

Extracts the magnitude of a complex number.

> Example:
```{.js}
norm(rect(3, 4)) === 5;
```

#### angle(c)

Extracts the angle of a complex number.

> Example:
```{.js}
angle(i(42)) === Math.PI / 2;
```

#### equal(c1, c2, e = 0)

Compares two complex numbers for equality, optionally taking a positive
residue for approximate comparisons.

#### add(c1, c2)

Computes the addition of two complex numbers.

> Example:
```{.js}
equal(add(rect(3), rect(0, 4)), rect(3, 4));
```

#### sub(c1, c2)

Computes the subtraction of two complex numbers.

> Example:
```{.js}
equal(sub(rect(3), rect(0, 4)), rect(3, -4));
```

#### mul(c1, c2)

Computes the multiplication of two complex numbers.

> Example:
```{.js}
equal(mul(rect(3), rect(0, 4)), i(12));
```

#### div(c1, c2)

Computes the division of two complex numbers.

> Example:
```{.js}
equal(div(rect(3), rect(0, 4)), i(-0.75));
```

#### neg(c)

Computes the opposite of a complex number.

> Example:
```{.js}
equal(neg(rect(3, 4)), rect(-3, -4));
```

#### conj(c)

Computes the conjugate of a complex number.

> Example:
```{.js}
equal(conj(rect(3, 4)), rect(3, -4));
```

#### exp(c)

Computes the exponential of a complex number.

> Example:
```{.js}
equal(exp(rect(3, 4)), polar(Math.exp(3), 4));
```

#### log(c)

Computes the principal logarithm of a complex number.

> Example:
```{.js}
equal(log(rect(3, 4)), rect(Math.log(5), Math.atan(4 / 3)));
```

#### sinh(c)

Computes the hyperbolic sine of a complex number.

> Example:
```{.js}
const c = rect(3, 4);
equal(sinh(c), div(sub(exp(c), exp(neg(c))), rect(2)));
```

#### cosh(c)

Computes the hyperbolic cosine of a complex number.

> Example:
```{.js}
const c = rect(3, 4);
equal(cosh(c), div(add(exp(c), exp(neg(c))), rect(2)));
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
cos(Math.tan(ϕ)) ≡ 1 / sqrt(1 + ϕ^2)
sin(Math.tan(ϕ)) ≡ sign(ϕ) / sqrt(1 + 1 / ϕ^2)

```

These identities are particularly convenient, because those square roots in
the denominators can take advantage of the [hypot] algorithm, which efficiently avoids unnecessary over/underflows.

[Steady]: https://brunocodutra.github.io/steady/
[a]:      https://www.npmjs.com/package/complexjs
[b]:      https://www.npmjs.com/package/complex-numbers
[c]:      https://www.npmjs.com/package/complex
[d]:      https://www.npmjs.com/package/complex-js
[hypot]:  https://en.wikipedia.org/wiki/Hypot