import initWasm, { init, registerFont } from '@wasm-echarts';

const width = 560;
const height = 400;

/**
 * WASM 不读系统字体，必须把字体文件 fetch 进来再 registerFont。
 *
 * 本示例字体从 Windows 复制到 site/public/fonts/（微软授权字体请勿提交到 git）：
 *   C:\Windows\Fonts\msyh.ttc   → /fonts/msyh.ttc     微软雅黑
 *   C:\Windows\Fonts\simsun.ttc → /fonts/simsun.ttc   宋体
 *   C:\Windows\Fonts\simkai.ttf → /fonts/simkai.ttf   楷体
 *
 * 已有的 Noto Sans SC 继续作为 CSS 通用族 sans-serif 的默认回退。
 */
const FONTS = [
  {
    url: '/fonts/NotoSansSC-Regular.ttf',
    familyName: 'Noto Sans SC',
    sansSerif: ['Noto Sans SC'],
  },
  {
    url: '/fonts/msyh.ttc',
    familyName: 'Microsoft YaHei',
  },
  {
    url: '/fonts/simsun.ttc',
    familyName: 'SimSun',
  },
  {
    url: '/fonts/simkai.ttf',
    familyName: 'KaiTi',
  },
];

async function registerFontFromUrl({ url, familyName, sansSerif }) {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`字体加载失败: ${url} (${response.status})`);
  }
  const bytes = new Uint8Array(await response.arrayBuffer());
  const opts = { familyName };
  if (sansSerif) {
    opts.sansSerif = sansSerif;
  }
  registerFont(bytes, opts);
}

async function loadFonts() {
  for (const font of FONTS) {
    await registerFontFromUrl(font);
  }
}

async function main() {
  await initWasm();
  await loadFonts();

  const canvas = document.getElementById('canvas');
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const chart = init(canvas);
  chart.setOption({
    title: {
      text: '季度销量（雅黑标题）',
      subtext: '宋体副标题 · 演示 registerFont 与多 fontFamily',
      left: 'center',
      top: 8,
      textStyle: {
        fontFamily: 'Microsoft YaHei',
        fontSize: 18,
        color: '#1f1f1f',
      },
      subtextStyle: {
        fontFamily: 'SimSun',
        fontSize: 12,
        color: '#666',
      },
    },
    legend: {
      top: 56,
      left: 'center',
      textStyle: {
        fontFamily: 'KaiTi',
        fontSize: 13,
        color: '#333',
      },
    },
    grid: { top: 96, left: 72, right: 56, bottom: 48 },
    xAxis: {
      type: 'category',
      data: ['Q1', 'Q2', 'Q3', 'Q4'],
      name: '季度',
      nameTextStyle: {
        fontFamily: 'SimSun',
        fontSize: 12,
        color: '#666',
      },
      axisLabel: {
        fontFamily: 'Microsoft YaHei',
        fontSize: 11,
      },
    },
    yAxis: {
      type: 'value',
      name: '单位：万件',
      nameTextStyle: {
        fontFamily: 'KaiTi',
        fontSize: 12,
        color: '#666',
      },
      axisLabel: {
        fontFamily: 'SimSun',
        fontSize: 11,
        formatter: '{value} 万',
      },
    },
    series: [
      {
        type: 'line',
        name: '华东',
        data: [120, 200, 150, 180],
        symbol: 'emptyCircle',
        symbolSize: 8,
      },
      {
        type: 'line',
        name: '华北',
        data: [90, 130, 170, 140],
        symbol: 'emptyCircle',
        symbolSize: 8,
      },
    ],
  });
}

main();
