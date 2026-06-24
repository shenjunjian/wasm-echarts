import { createHighlighter } from 'shiki';

/** @type {Promise<import('shiki').Highlighter> | null} */
let highlighterPromise = null;

const THEME = 'github-dark';
const LANG = 'javascript';

function getOrCreateHighlighter() {
  if (!highlighterPromise) {
    highlighterPromise = createHighlighter({
      themes: [THEME],
      langs: [LANG],
    });
  }
  return highlighterPromise;
}

/**
 * @param {string} code
 * @param {string} [lang]
 */
export async function highlightSource(code, lang = LANG) {
  const highlighter = await getOrCreateHighlighter();
  return highlighter.codeToHtml(code, { lang, theme: THEME });
}
