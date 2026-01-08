import { ref } from 'vue';
import { setTheme, setColorScheme } from 'mdui';

// 全局状态
const currentTheme = ref(localStorage.getItem('theme') || 'auto');
const currentColor = ref(localStorage.getItem('themeColor') || '#6750A4');

// 预定义颜色（参考 Material Design）
export const themeColors = [
    { name: 'Deep Purple', color: '#6750A4' },
    { name: 'Green', color: '#4CAF50' },
    { name: 'Blue', color: '#2196F3' },
    { name: 'Red', color: '#F44336' },
    { name: 'Orange', color: '#FF9800' },
    { name: 'Teal', color: '#009688' },
];

export function useTheme() {
    // 助手函数：Hex 转 RGB
    const hexToRgb = (hex: string) => {
        const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
        return result ? {
            r: parseInt(result[1], 16),
            g: parseInt(result[2], 16),
            b: parseInt(result[3], 16)
        } : { r: 103, g: 80, b: 164 }; // 默认值
    };

    // 助手函数：混合颜色
    const mixColors = (color1: any, color2: any, weight: number) => {
        return {
            r: Math.round(color1.r * weight + color2.r * (1 - weight)),
            g: Math.round(color1.g * weight + color2.g * (1 - weight)),
            b: Math.round(color1.b * weight + color2.b * (1 - weight))
        };
    };

    // 助手函数：RGB 转 Hex
    const rgbToHex = (rgb: any) => {
        return '#' + [rgb.r, rgb.g, rgb.b].map(x => x.toString(16).padStart(2, '0')).join('');
    };

    const applyAllMonetVariables = (primaryColor: string, isDark: boolean) => {
        const html = document.documentElement;
        setColorScheme(primaryColor);

        const primary = hexToRgb(primaryColor);

        // 设置主色变量
        html.style.setProperty('--monet-primary', primaryColor);
        html.style.setProperty('--monet-primary-container', primaryColor + '30');
        html.style.setProperty('--monet-on-primary', '#ffffff');
        html.style.setProperty('--monet-on-primary-container', primaryColor);
        html.style.setProperty('--monet-secondary-container', primaryColor + '30');
        html.style.setProperty('--monet-on-secondary-container', primaryColor);

        if (isDark) {
            // 深色模式表面色生成
            const darkBase = { r: 20, g: 18, b: 24 };
            const darkMid = { r: 33, g: 31, b: 38 };
            const darkHigh = { r: 43, g: 41, b: 48 };
            const darkHighest = { r: 54, g: 52, b: 59 };

            const surface = mixColors(primary, darkBase, 0.05);
            const surfaceContainer = mixColors(primary, darkMid, 0.05);
            const surfaceContainerLow = mixColors(primary, { r: 29, g: 27, b: 32 }, 0.05);
            const surfaceContainerHigh = mixColors(primary, darkHigh, 0.05);
            const surfaceContainerHighest = mixColors(primary, darkHighest, 0.05);

            html.style.setProperty('--monet-surface', rgbToHex(surface));
            html.style.setProperty('--monet-on-surface', '#e6e0e9');
            html.style.setProperty('--monet-surface-variant', '#49454f');
            html.style.setProperty('--monet-on-surface-variant', '#cac4d0');
            html.style.setProperty('--monet-surface-container', rgbToHex(surfaceContainer));
            html.style.setProperty('--monet-surface-container-low', rgbToHex(surfaceContainerLow));
            html.style.setProperty('--monet-surface-container-high', rgbToHex(surfaceContainerHigh));
            html.style.setProperty('--monet-surface-container-highest', rgbToHex(surfaceContainerHighest));
            html.style.setProperty('--monet-background', rgbToHex(surface));
            html.style.setProperty('--monet-on-background', '#e6e0e9');
            html.style.setProperty('--monet-outline', '#938f99');
            html.style.setProperty('--monet-outline-variant', '#49454f');
            html.style.setProperty('--monet-secondary', '#ccc2dc');
            html.style.setProperty('--monet-on-secondary', '#332d41');

            // 深色模式滚动条
            html.style.setProperty('--scrollbar-thumb', 'rgba(255, 255, 255, 0.3)');
            html.style.setProperty('--scrollbar-thumb-hover', 'rgba(255, 255, 255, 0.5)');
        } else {
            // 浅色模式表面色生成
            const white = { r: 255, g: 255, b: 255 };

            const surface = mixColors(primary, white, 0.03);
            const surfaceContainer = mixColors(primary, white, 0.06);
            const surfaceContainerLow = mixColors(primary, white, 0.04);
            const surfaceContainerHigh = mixColors(primary, white, 0.08);
            const surfaceContainerHighest = mixColors(primary, white, 0.10);

            html.style.setProperty('--monet-surface', rgbToHex(surface));
            html.style.setProperty('--monet-on-surface', '#1d1b20');
            html.style.setProperty('--monet-surface-variant', rgbToHex(mixColors(primary, { r: 231, g: 224, b: 236 }, 0.15)));
            html.style.setProperty('--monet-on-surface-variant', '#49454f');
            html.style.setProperty('--monet-surface-container', rgbToHex(surfaceContainer));
            html.style.setProperty('--monet-surface-container-low', rgbToHex(surfaceContainerLow));
            html.style.setProperty('--monet-surface-container-high', rgbToHex(surfaceContainerHigh));
            html.style.setProperty('--monet-surface-container-highest', rgbToHex(surfaceContainerHighest));
            html.style.setProperty('--monet-background', rgbToHex(surface));
            html.style.setProperty('--monet-on-background', '#1d1b20');
            html.style.setProperty('--monet-outline', '#79747e');
            html.style.setProperty('--monet-outline-variant', rgbToHex(mixColors(primary, { r: 202, g: 196, b: 208 }, 0.10)));
            html.style.setProperty('--monet-secondary', rgbToHex(mixColors(primary, { r: 98, g: 91, b: 113 }, 0.20)));
            html.style.setProperty('--monet-on-secondary', '#ffffff');

            // 浅色模式滚动条
            html.style.setProperty('--scrollbar-thumb', 'rgba(128, 128, 128, 0.4)');
            html.style.setProperty('--scrollbar-thumb-hover', 'rgba(128, 128, 128, 0.6)');
        }
    };

    const applyTheme = (theme: string) => {
        const html = document.documentElement;
        const isDark = theme === 'dark' || (theme === 'auto' && window.matchMedia('(prefers-color-scheme: dark)').matches);

        html.classList.remove('mdui-theme-light', 'mdui-theme-dark', 'mdui-theme-auto');
        html.classList.add(`mdui-theme-${theme}`);
        setTheme(theme as any);

        // 应用颜色逻辑
        applyAllMonetVariables(currentColor.value, isDark);
    };

    const setThemeMode = (mode: string) => {
        currentTheme.value = mode;
        localStorage.setItem('theme', mode);
        applyTheme(mode);
    };

    const setThemeColor = (color: string) => {
        currentColor.value = color;
        localStorage.setItem('themeColor', color);
        applyTheme(currentTheme.value);
    };

    const toggleTheme = () => {
        const themes = ['light', 'dark', 'auto'];
        const currentIndex = themes.indexOf(currentTheme.value);
        const nextTheme = themes[(currentIndex + 1) % themes.length];
        setThemeMode(nextTheme);
    };

    const initTheme = () => {
        applyTheme(currentTheme.value);

        // 如果在自动模式，监听系统主题变化
        window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
            if (currentTheme.value === 'auto') {
                applyTheme('auto');
            }
        });
    };

    return {
        currentTheme,
        currentColor,
        themeColors,
        setThemeMode,
        setThemeColor,
        toggleTheme,
        initTheme
    };
}
