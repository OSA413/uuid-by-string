const generateUuid = require('..');
const {
  uint8ToHex,
  md5Hash,
  stringToCharBuffer,
  sha1Hash,
  hashToUuid,
  uint8ArrayToHex,
  validateUuid,
  parseUuid,
} = require('../src/lib');
const { longText } = require('./__mock__/longText');

const stringSamples = [
  '',
  'Hello world!',
  'Lorem ipsum',
  'ヒューマニズムは、イデオロギー的ヒューマニズムを発見します。多くの国では、フランスの最も例示的な例その中で、個人崇拝は政治文化の英米の種類を証明しています。社会経済発展は、伝統的な概念によると、国民投票を決定します。特に政治的不安定の状態でパワーのメカニズム、不均一。それは逆説的に見えるかもしれませんように政治的な紛争管理は、一意の機能キリスト教民主主義ナショナリズムです。近代化の概念は、マルクス主義を証明しています。',
  longText,
];

describe('unit', () => {
  test('should validate uuid', () => {
    expect(validateUuid('d3486ae9-136e-5856-bc42-212385ea7970')).toBe(true);
  });

  test('should invalidate uuid', () => {
    expect(validateUuid('Lorem ipsum')).toBe(false);
  });

  test('should parse uuid', () => {
    expect(parseUuid('d3486ae9-136e-5856-bc42-212385ea7970')).toMatchSnapshot();
  });

  test('should throw error while parsing uuid', () => {
    expect(() => parseUuid('Lorem ipsum')).toThrowError();
  });
});

describe('integration', () => {
  test('Hello world!', () => {
    expect(generateUuid('Hello world!')).toBe('d3486ae9-136e-5856-bc42-212385ea7970');
  });

  test('should throw error because of the wrong value', () => {
    expect(() => generateUuid()).toThrowError();
  });

  test('should throw error because of the wrong version', () => {
    expect(() => generateUuid('Hello', 1)).toThrowError();
    expect(() => generateUuid('Hello', 'd3486ae9-136e-5856-bc42-212385ea7970', 1)).toThrowError();
  });

  test('should generate uuid v3 from string', () => {
    for (let i = 0; i < stringSamples.length; i++) {
      const uuid = generateUuid(stringSamples[i], 3);

      expect(uuid).toMatchSnapshot();
    }
  });

  test('should generate uuid v3 from string with namespace', () => {
    for (let i = 0; i < stringSamples.length; i++) {
      const uuid = generateUuid(stringSamples[i], 'd3486ae9-136e-5856-bc42-212385ea7970');

      expect(uuid).toMatchSnapshot();
    }
  });

  test('should generate uuid v5 from string', () => {
    for (let i = 0; i < stringSamples.length; i++) {
      const uuid = generateUuid(stringSamples[i]);

      expect(uuid).toMatchSnapshot();
    }
  });

  test('should generate uuid v5 from string with namespace', () => {
    for (let i = 0; i < stringSamples.length; i++) {
      const uuid = generateUuid(stringSamples[i], 'd3486ae9-136e-5856-bc42-212385ea7970', 5);

      expect(uuid).toMatchSnapshot();
    }
  });

  test('should generate platform compatible uuid', () => {
    expect(generateUuid('9239107d-259f-4cf8-b62d-0964b680ab08', 3)).toBe('12f01aa4-5090-3f83-b823-7e7cb43246e7');
  });
});

describe('checker of speed of one generation', () => {
  it('100k chars', () => {
    generateUuid(longText);
  });
});
