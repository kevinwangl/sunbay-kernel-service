/* tslint:disable */
/* eslint-disable */

export class WasmEmvProcessor {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Generate AC (Application Cryptogram)
   */
  generateAc(ac_type: number, cdol_hex: string): any;
  /**
   * Read record from card
   */
  readRecord(sfi: number, record: number): any;
  /**
   * Select PPSE (Payment System Environment)
   */
  selectPpse(): any;
  /**
   * Parse card data from TLV response
   */
  parseCardData(tlv_hex: string, aid: string): any;
  /**
   * Select application by AID
   */
  selectApplication(aid_hex: string): any;
  /**
   * Get processing options
   */
  getProcessingOptions(pdol_hex: string): any;
  /**
   * Create a new EMV processor
   */
  constructor(country_code: string, currency_code: string);
}

/**
 * Get the version of the kernel
 */
export function getVersion(): string;

/**
 * Initialize panic hook for better error messages
 */
export function init(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmemvprocessor_free: (a: number, b: number) => void;
  readonly getVersion: () => [number, number];
  readonly init: () => void;
  readonly wasmemvprocessor_generateAc: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly wasmemvprocessor_getProcessingOptions: (a: number, b: number, c: number) => [number, number, number];
  readonly wasmemvprocessor_new: (a: number, b: number, c: number, d: number) => number;
  readonly wasmemvprocessor_parseCardData: (a: number, b: number, c: number, d: number, e: number) => [number, number, number];
  readonly wasmemvprocessor_readRecord: (a: number, b: number, c: number) => any;
  readonly wasmemvprocessor_selectApplication: (a: number, b: number, c: number) => [number, number, number];
  readonly wasmemvprocessor_selectPpse: (a: number) => any;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_table_dealloc: (a: number) => void;
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
