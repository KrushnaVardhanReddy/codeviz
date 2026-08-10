export declare function init(input?: string | URL | Request | BufferSource): Promise<void>;
export declare function supported_languages(): string[];
export declare function parse_to_json(source: string, language: string): string;
export declare function parse(source: string, language: string, diagram_kind: string): string;
export default init;
