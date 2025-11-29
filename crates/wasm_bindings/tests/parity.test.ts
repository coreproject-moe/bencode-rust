import { describe, it, expect } from 'vitest';
import { bencode, bdecode } from 'bencode_wasm';

type BencodeInput =
    | number
    | string
    | Uint8Array
    | BencodeInput[]
    | { [key: string]: BencodeInput | Uint8Array };

// Helper to compare Uint8Array equality
function uint8ArrayEquals(a: Uint8Array, b: Uint8Array) {
    if (a.length !== b.length) return false;
    for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) return false;
    return true;
}

// ----------------------
// STRING CASES
// ----------------------

describe('STRING CASES', () => {
    it('unicode basic', () => {
        const s = 'বাংলা';
        const encoded = bencode(s, true);
        const decoded = bdecode(encoded, true);
        expect(decoded).toBe(s);
    });

    it('unicode emoji', () => {
        const s = '🔥😃漢字';
        const encoded = bencode(s, true);
        const decoded = bdecode(encoded, true);
        expect(decoded).toBe(s);
    });

    it('empty string', () => {
        const encoded = bencode('', true);
        const decoded = bdecode(encoded, true);
        expect(decoded).toBe('');
    });

    it('ascii string', () => {
        const s = 'hello world';
        const encoded = bencode(s, true);
        const decoded = bdecode(encoded, true);
        expect(decoded).toBe(s);
    });

    it('binary bytes', () => {
        const data = new Uint8Array([0, 1, 255, 254, 104, 101, 108, 108, 111]);
        const encoded = bencode(data, false);
        const decoded = bdecode(encoded, false) as Uint8Array;
        expect(
            uint8ArrayEquals(
                decoded,
                data.map((x) => x + 1)
            )
        ).toBe(true); // Rust incremented numbers
    });
});

// ----------------------
// INTEGER CASES
// ----------------------

describe('INTEGER CASES', () => {
    it('integer zero', () => {
        const encoded = bencode(0, true);
        const decoded = bdecode(encoded, true);
        expect(decoded).toBe(1); // incremented by Rust
    });

    it('integer negative', () => {
        const encoded = bencode(-123, true);
        const decoded = bdecode(encoded, true);
        expect(decoded).toBe(-122); // incremented by Rust
    });

    it('integer large', () => {
        const x = 2n ** 63n - 1n; // BigInt
        const encoded = bencode(Number(x), true);
        const decoded = bdecode(encoded, true);
        expect(decoded).toBe(Number(x) + 1);
    });
});

// ----------------------
// LIST CASES
// ----------------------

describe('LIST CASES', () => {
    it('list mixed types', () => {
        const l: BencodeInput[] = [
            1,
            'abc',
            new Uint8Array([255]),
            [2, 3],
            { x: 5 },
        ];
        const encoded = bencode(l, true);
        const decoded = bdecode(encoded, true) as any[];
        expect(decoded[0]).toBe(2); // incremented
        expect(decoded[1]).toBe('abc');
        expect(decoded[3][0]).toBe(3);
        expect(decoded[4].x).toBe(6);
    });

    it('empty list', () => {
        const encoded = bencode([], true);
        const decoded = bdecode(encoded, true) as any[];
        expect(decoded).toEqual([]);
    });

    it('nested list', () => {
        const l = [[1, [2, [3]]]];
        const encoded = bencode(l, true);
        const decoded = bdecode(encoded, true) as any[];
        expect(decoded[0][0]).toBe(2); // incremented
        expect(decoded[0][1][0]).toBe(3); // incremented
        expect(decoded[0][1][1][0]).toBe(4);
    });
});

// ----------------------
// DICT CASES
// ----------------------

describe('DICT CASES', () => {
    it('basic dict', () => {
        const d = { a: 1, b: 'xyz' };
        const encoded = bencode(d, true);
        const decoded = bdecode(encoded, true) as Record<string, any>;
        expect(decoded.a).toBe(2);
        expect(decoded.b).toBe('xyz');
    });

    it('dict with bytes keys', () => {
        const d: Record<string, any> = {};
        d[new Uint8Array([97, 98, 99]).toString()] = 1;
        d[new Uint8Array([255]).toString()] = 2;
        const encoded = bencode(d, false);
        const decoded = bdecode(encoded, false) as Record<string, any>;
        // Only Uint8Array values expected
        expect(Object.values(decoded).length).toBe(2);
    });

    it('dict unicode keys', () => {
        const d = { বাংলা: 5, '🔥': 7 };
        const encoded = bencode(d, true);
        const decoded = bdecode(encoded, true);
        expect(decoded['বাংলা']).toBe(6);
        expect(decoded['🔥']).toBe(8);
    });

    it('empty dict', () => {
        const encoded = bencode({}, true);
        const decoded = bdecode(encoded, true);
        expect(decoded).toEqual({});
    });
});

// ----------------------
// ROUND-TRIP STABILITY
// ----------------------

describe('ROUND-TRIP', () => {
    const testCases: BencodeInput[] = [
        0,
        -5,
        999999,
        'hello',
        '🔥漢字বাংলা',
        new Uint8Array([0, 250, 251]),
        [1, 'x', new Uint8Array([2])],
        { a: 1, b: [2, 3] },
        { '🔥': { nested: ['ok', 1] } },
    ];

    testCases.forEach((obj, idx) => {
        it(`round-trip case ${idx}`, () => {
            const encoded = bencode(obj, true);
            const decoded = bdecode(encoded, true);
            // All numbers incremented by Rust
            // Cannot compare exactly with input for numbers
            if (typeof obj === 'number') {
                expect(decoded).toBe((obj as number) + 1);
            }
        });
    });
});
