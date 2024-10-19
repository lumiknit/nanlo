// vitest

import { test, expect } from 'vitest';
import { add } from '../dist/seongnyang.esm';

test('add', () => {
	expect(add(BigInt(1), BigInt(2))).toBe(BigInt(3));
});
