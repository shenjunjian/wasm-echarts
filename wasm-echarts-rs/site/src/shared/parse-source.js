/**
 * 解析 zrender 实例页源码：JSON { scene, width?, height? }
 */
export function parseZrenderSource(source) {
  const trimmed = source.trim();
  if (!trimmed) {
    throw new Error('源码为空');
  }
  let config;
  try {
    config = JSON.parse(trimmed);
  } catch {
    throw new Error('源码须为 JSON：{ "scene": "shapes", "width": 480, "height": 360 }');
  }
  const scene = config.scene;
  if (!scene || typeof scene !== 'string') {
    throw new Error('缺少 scene 字段（shapes | text | sector | hit | state）');
  }
  const allowed = ['shapes', 'text', 'sector', 'hit', 'state'];
  if (!allowed.includes(scene)) {
    throw new Error(`未知 scene: ${scene}，可选: ${allowed.join(', ')}`);
  }
  return {
    scene,
    width: config.width ?? 480,
    height: config.height ?? 360,
  };
}

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
