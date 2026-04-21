type NativeModule = {
  lexical_score?: (text: string, query: string) => number;
  lexicalScore?: (text: string, query: string) => number;
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
