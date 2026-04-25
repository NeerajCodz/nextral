type NativeModule = {
  lexical_score?: (text: string, query: string) => number;
  lexicalScore?: (text: string, query: string) => number;
  validate_config?: (configJson: string) => string;
  validateConfig?: (configJson: string) => string;
};

let nativeModule: NativeModule | null = null;

export function lexicalScore(text: string, query: string): number {
  if (!nativeModule) {
    nativeModule = require("./index.node") as NativeModule;
  }

  const scorer = nativeModule.lexicalScore ?? nativeModule.lexical_score;
  if (!scorer) {
    throw new Error("Nextral native module is missing lexical score export.");
  }
  return scorer(text, query);
}

export function validateConfig(config: unknown): { status: string } {
  if (!nativeModule) {
    nativeModule = require("./index.node") as NativeModule;
  }

  const validator = nativeModule.validateConfig ?? nativeModule.validate_config;
  if (!validator) {
    throw new Error("Nextral native module is missing config validation export.");
  }
  return JSON.parse(validator(JSON.stringify(config))) as { status: string };
}
