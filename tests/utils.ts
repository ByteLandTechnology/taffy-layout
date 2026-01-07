import { loadTaffy } from "../src/index";

let initialized = false;

export async function setupTaffy() {
  if (!initialized) {
    await loadTaffy();
    initialized = true;
  }
}
