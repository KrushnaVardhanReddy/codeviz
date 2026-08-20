/* tslint:disable */
/* eslint-disable */

/**
 * A simplified AST to CodeGraph generator using JSON AST created by web-tree-sitter.
 */
export function parse_and_build_graph(language: string, file_path: string, ast_json: string): string;

/**
 * Render a pre-parsed CodeGraph JSON into a Mermaid diagram string.
 * @param graph_json   - Full JSON-serialized CodeGraph
 * @param diagram_kind - "module" | "call" | "class"
 * @returns Mermaid diagram string
 * @throws string error message on failure
 */
export function render_graph(graph_json: string, diagram_kind: string): string;

/**
 * Return a list of supported language identifiers.
 */
export function supported_languages(): Array<any>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly parse_and_build_graph: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number, number, number];
    readonly render_graph: (a: number, b: number, c: number, d: number) => [number, number, number, number];
    readonly supported_languages: () => any;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
