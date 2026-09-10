/** 跨 facade 共享的注册表（theme / map / connect）。 */

export const themes = new Map();
export const maps = new Map();
/** @type {Map<string, boolean>} */
export const connectedGroups = new Map();
