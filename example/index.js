const assert = require('assert');
const {i, rect, add, exp, equal} = require('../dist');

assert.ok(equal(add(exp(i(Math.PI)), rect(1)), rect(0), 1E-9));
