
/**
 * 是否为空
 * @param str 目标字符串
 */
export function isBlank(str: string): boolean {
    if (str == null || str.trim().length < 1) {
        return true;
    }

    return false;
}