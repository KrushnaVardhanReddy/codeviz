import { init, supported_languages, parse, parse_to_json } from './index';

describe('CodeViz WASM Node Wrapper', () => {
    beforeAll(async () => {
        await init();
    });

    it('should support the defined languages', () => {
        const langs = supported_languages();
        expect(langs).toContain('python');
        expect(langs).toContain('typescript');
        expect(langs).toContain('java');
    });

    it('should parse simple source code into a JSON string', () => {
        const source = 'def my_function():\n    pass';
        const jsonStr = parse_to_json(source, 'python');

        expect(typeof jsonStr).toBe('string');
        const jsonObj = JSON.parse(jsonStr);
        expect(jsonObj.nodes).toBeDefined();
        expect(jsonObj.meta.language).toBe('python');
    });

    it('should parse source code and generate a mermaid diagram', () => {
        const source = 'import os\n\ndef my_func(): pass';
        const diagram = parse(source, 'python', 'module');

        expect(typeof diagram).toBe('string');
        expect(diagram.startsWith('graph TD')).toBe(true);
    });

    it('should throw for unknown languages', () => {
        expect(() => {
            parse_to_json('x', 'cobol');
        }).toThrow('Unsupported language: cobol');
    });

    it('should throw for invalid diagram kind', () => {
        expect(() => {
            parse('x', 'python', 'invalid');
        }).toThrow('Unknown diagram kind: invalid');
    });
});
