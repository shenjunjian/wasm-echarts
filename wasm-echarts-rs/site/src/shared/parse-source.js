/**
 * 从源码提取 option：支持纯 JSON 或 option = {...} 赋值语句
 */
export function parseEchartsSource(source) {
  const trimmed = source.trim();
  if (!trimmed) {
    throw new Error('源码为空');
  }

  let jsonText = trimmed;
  const assignMatch = trimmed.match(/(?:const|let|var)\s+option\s*=\s*([\s\S]+);?\s*$/);
  if (assignMatch) {
    jsonText = assignMatch[1].trim().replace(/;\s*$/, '');
  }

  let option;
  try {
    option = JSON.parse(jsonText);
  } catch (err) {
    throw new Error(`无法解析 option JSON: ${err.message}`);
  }

  if (typeof option !== 'object' || option === null) {
    throw new Error('option 须为对象');
  }

  return option;
}

export function formatOption(option) {
  return JSON.stringify(option, null, 2);
}

/**
 * 去掉 // 行注释后再解析 echarts option
 */
export function parseEchartsSourceStripComments(source) {
  const code = source.split('\n//')[0].trim();
  return parseEchartsSource(code);
}
