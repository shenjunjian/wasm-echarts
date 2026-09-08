/**
 * 最小 SVG 解析：能解析 g / path / 基础图形。
 * 完整 SVG 语义（defs 渐变、text、clip）后置。
 */
import { Group } from '../group.js';
import { Circle, Ellipse, Line, Polygon, Polyline, Rect } from '../shapes/index.js';
import { createFromString } from './path.js';
import { isString, trim } from './util.js';

class XmlNode {
  constructor(nodeName, attrs, parent) {
    this.nodeName = nodeName;
    this.nodeType = 1;
    this.attributes = attrs || {};
    this.children = [];
    this.parent = parent || null;
    this.textContent = '';
  }

  getAttribute(name) {
    if (name == null) {
      return null;
    }
    const attrs = this.attributes;
    if (Object.prototype.hasOwnProperty.call(attrs, name)) {
      return attrs[name];
    }
    const lower = String(name).toLowerCase();
    for (const key of Object.keys(attrs)) {
      if (key.toLowerCase() === lower) {
        return attrs[key];
      }
    }
    return null;
  }

  get firstChild() {
    return this.children[0] || null;
  }

  get nextSibling() {
    if (!this.parent) {
      return null;
    }
    const siblings = this.parent.children;
    const idx = siblings.indexOf(this);
    return idx >= 0 ? siblings[idx + 1] || null : null;
  }
}

function parseAttrs(raw) {
  const attrs = {};
  const re = /([^\s=]+)(?:\s*=\s*(?:"([^"]*)"|'([^']*)'|(\S+)))?/g;
  let m;
  while ((m = re.exec(raw))) {
    attrs[m[1]] = m[2] != null ? m[2] : m[3] != null ? m[3] : m[4] != null ? m[4] : '';
  }
  return attrs;
}

function parseSvgString(str) {
  const cleaned = String(str)
    .replace(/<\?xml[\s\S]*?\?>/i, '')
    .replace(/<!DOCTYPE[\s\S]*?>/i, '')
    .replace(/<!--[\s\S]*?-->/g, '');
  const rootWrap = new XmlNode('#document', {});
  rootWrap.nodeType = 9;
  const stack = [rootWrap];
  const tokenRe = /<(\/)?([A-Za-z][\w:.-]*)([^>]*?)(\/)?>|([^<]+)/g;
  let m;
  while ((m = tokenRe.exec(cleaned))) {
    if (m[5] != null) {
      const text = m[5];
      if (trim(text)) {
        const parent = stack[stack.length - 1];
        const node = new XmlNode('#text', {}, parent);
        node.nodeType = 3;
        node.textContent = text;
        parent.children.push(node);
      }
      continue;
    }
    const closing = !!m[1];
    const name = m[2];
    const selfClose = !!m[4] || /\/\s*$/.test(m[3] || '');
    if (closing) {
      for (let i = stack.length - 1; i > 0; i--) {
        if (stack[i].nodeName.toLowerCase() === name.toLowerCase()) {
          stack.length = i;
          break;
        }
      }
      continue;
    }
    const parent = stack[stack.length - 1];
    const node = new XmlNode(name, parseAttrs(m[3] || ''), parent);
    parent.children.push(node);
    if (!selfClose) {
      stack.push(node);
    }
  }
  return rootWrap;
}

function findSvg(node) {
  if (!node) {
    return null;
  }
  if (node.nodeType === 1 && String(node.nodeName).toLowerCase() === 'svg') {
    return node;
  }
  if (node.nodeType === 9 || node.documentElement) {
    const docEl = node.documentElement;
    if (docEl && String(docEl.nodeName).toLowerCase() === 'svg') {
      return wrapDom(docEl);
    }
  }
  let child = node.firstChild;
  while (child) {
    const found = findSvg(child);
    if (found) {
      return found;
    }
    child = child.nextSibling;
  }
  return null;
}

function wrapDom(el) {
  if (!el || el instanceof XmlNode) {
    return el;
  }
  const node = new XmlNode(el.nodeName, {}, null);
  node.nodeType = el.nodeType;
  if (el.attributes) {
    for (let i = 0; i < el.attributes.length; i++) {
      const attr = el.attributes[i];
      node.attributes[attr.name] = attr.value;
    }
  }
  node.textContent = el.textContent || '';
  let child = el.firstChild;
  while (child) {
    if (child.nodeType === 1 || child.nodeType === 3) {
      const wrapped = wrapDom(child);
      wrapped.parent = node;
      node.children.push(wrapped);
    }
    child = child.nextSibling;
  }
  return node;
}

export function parseXML(svg) {
  if (isString(svg)) {
    if (typeof DOMParser !== 'undefined') {
      const doc = new DOMParser().parseFromString(svg, 'text/xml');
      const found = findSvg(doc);
      if (found) {
        return found;
      }
    }
    const tree = parseSvgString(svg);
    const found = findSvg(tree);
    if (!found) {
      throw new Error('Illegal svg');
    }
    return found;
  }
  const found = findSvg(svg) || wrapDom(svg);
  if (!found) {
    throw new Error('Illegal svg');
  }
  return found;
}

function parseFloatAttr(node, name, fallback) {
  const raw = node.getAttribute(name);
  if (raw == null || raw === '') {
    return fallback;
  }
  const n = parseFloat(raw);
  return Number.isFinite(n) ? n : fallback;
}

function parsePoints(str) {
  const nums = String(str)
    .trim()
    .split(/[\s,]+/)
    .map(Number)
    .filter((n) => Number.isFinite(n));
  const points = [];
  for (let i = 0; i + 1 < nums.length; i += 2) {
    points.push([nums[i], nums[i + 1]]);
  }
  return points;
}

function applyCommonStyle(el, node) {
  const style = {};
  const fill = node.getAttribute('fill');
  const stroke = node.getAttribute('stroke');
  const lineWidth = node.getAttribute('stroke-width');
  const opacity = node.getAttribute('opacity');
  const fillOpacity = node.getAttribute('fill-opacity');
  const strokeOpacity = node.getAttribute('stroke-opacity');
  if (fill != null) style.fill = fill;
  if (stroke != null) style.stroke = stroke;
  if (lineWidth != null) style.lineWidth = parseFloat(lineWidth);
  if (opacity != null) style.opacity = parseFloat(opacity);
  if (fillOpacity != null) style.fillOpacity = parseFloat(fillOpacity);
  if (strokeOpacity != null) style.strokeOpacity = parseFloat(strokeOpacity);
  if (Object.keys(style).length && typeof el.setStyle === 'function') {
    el.setStyle(style);
  }
  const name = node.getAttribute('name') || node.getAttribute('id');
  if (name) {
    el.name = name;
  }
  el.silent = true;
}

function splitNumberSequence(str) {
  return String(str).trim().split(/[\s,]+/).filter(Boolean);
}

export function makeViewBoxTransform(viewBoxRect, boundingRect) {
  const scaleX = boundingRect.width / viewBoxRect.width;
  const scaleY = boundingRect.height / viewBoxRect.height;
  const scale = Math.min(scaleX, scaleY);
  return {
    scale,
    x: -(viewBoxRect.x + viewBoxRect.width / 2) * scale + (boundingRect.x + boundingRect.width / 2),
    y: -(viewBoxRect.y + viewBoxRect.height / 2) * scale + (boundingRect.y + boundingRect.height / 2),
  };
}

function createElement(tag, node) {
  switch (tag) {
    case 'g':
      return new Group();
    case 'path':
      return createFromString(node.getAttribute('d') || '');
    case 'rect':
      return new Rect({
        shape: {
          x: parseFloatAttr(node, 'x', 0),
          y: parseFloatAttr(node, 'y', 0),
          width: parseFloatAttr(node, 'width', 0),
          height: parseFloatAttr(node, 'height', 0),
        },
      });
    case 'circle':
      return new Circle({
        shape: {
          cx: parseFloatAttr(node, 'cx', 0),
          cy: parseFloatAttr(node, 'cy', 0),
          r: parseFloatAttr(node, 'r', 0),
        },
      });
    case 'ellipse':
      return new Ellipse({
        shape: {
          cx: parseFloatAttr(node, 'cx', 0),
          cy: parseFloatAttr(node, 'cy', 0),
          rx: parseFloatAttr(node, 'rx', 0),
          ry: parseFloatAttr(node, 'ry', 0),
        },
      });
    case 'line':
      return new Line({
        shape: {
          x1: parseFloatAttr(node, 'x1', 0),
          y1: parseFloatAttr(node, 'y1', 0),
          x2: parseFloatAttr(node, 'x2', 0),
          y2: parseFloatAttr(node, 'y2', 0),
        },
      });
    case 'polygon':
      return new Polygon({
        shape: { points: parsePoints(node.getAttribute('points') || '') },
      });
    case 'polyline':
      return new Polyline({
        shape: { points: parsePoints(node.getAttribute('points') || '') },
      });
    default:
      return null;
  }
}

function walk(node, parentGroup, named) {
  if (!node || node.nodeType !== 1) {
    return;
  }
  const tag = String(node.nodeName).toLowerCase();
  if (tag === 'defs' || tag === 'style' || tag === 'title' || tag === 'desc') {
    return;
  }

  let el = null;
  if (tag === 'svg' || tag === 'switch') {
    el = parentGroup;
  } else {
    el = createElement(tag, node);
    if (!el) {
      return;
    }
    applyCommonStyle(el, node);
    const nameAttr = node.getAttribute('name');
    if (nameAttr) {
      named.push({
        name: nameAttr,
        namedFrom: null,
        svgNodeTagLower: tag,
        el,
      });
    }
    parentGroup.add(el);
  }

  if (el && el.isGroup) {
    let child = node.firstChild;
    while (child) {
      walk(child, el, named);
      child = child.nextSibling;
    }
  }
}

export function parseSVG(xml, opt) {
  opt = opt || {};
  const svg = parseXML(xml);
  const named = [];
  let root = new Group();

  const viewBox = svg.getAttribute('viewBox') || '';
  let width = parseFloat(svg.getAttribute('width') || opt.width);
  let height = parseFloat(svg.getAttribute('height') || opt.height);
  if (!Number.isFinite(width)) width = null;
  if (!Number.isFinite(height)) height = null;

  applyCommonStyle(root, svg);

  let child = svg.firstChild;
  while (child) {
    walk(child, root, named);
    child = child.nextSibling;
  }

  let viewBoxRect;
  let viewBoxTransform;
  if (viewBox) {
    const arr = splitNumberSequence(viewBox);
    if (arr.length >= 4) {
      viewBoxRect = {
        x: parseFloat(arr[0] || 0),
        y: parseFloat(arr[1] || 0),
        width: parseFloat(arr[2]),
        height: parseFloat(arr[3]),
      };
    }
  }

  if (viewBoxRect && width != null && height != null) {
    viewBoxTransform = makeViewBoxTransform(viewBoxRect, {
      x: 0,
      y: 0,
      width,
      height,
    });
    if (!opt.ignoreViewBox) {
      const inner = root;
      root = new Group();
      root.add(inner);
      inner.scaleX = inner.scaleY = viewBoxTransform.scale;
      inner.x = viewBoxTransform.x;
      inner.y = viewBoxTransform.y;
    }
  }

  if (!opt.ignoreRootClip && width != null && height != null) {
    root.setClipPath(
      new Rect({
        shape: { x: 0, y: 0, width, height },
      }),
    );
  }

  return {
    root,
    width,
    height,
    viewBoxRect,
    viewBoxTransform,
    named,
  };
}
