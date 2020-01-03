import { logging, storage, math } from "near-runtime-ts";
// available class: near, context, storage, logging, base58, base64,
// PersistentMap, PersistentVector, PersistentDeque, PersistentTopN, ContractPromise, math
import { TextMessage } from "./model";
import { TemplateModel } from "./model";

const NAME = ". Welcome to NEAR Protocol chain";
export function welcome(name: string): TextMessage {
    logging.log("simple welcome test");
    let message = new TextMessage();
    const s = printString(NAME);
    message.text = "Welcome, " + name + s;
    logging.log(message.text);
    return message;
}

export function hello(): string {
    return "Hello, World!";
}

function printString(s: string): string {
    return s;
}

/**
 * 创建模板
 * @param accountId 模板创建者
 * @param templateInfo 模板信息
 */
export function createTemplate(
    accountId: string,
    templateInfo: string
): boolean {
    if (isBlank(accountId) || isBlank(templateInfo)) {
        return false;
    }

    storage.setString(accountId + "_templates", templateInfo);
    return true;
}

/**
 * 查询创建的模板
 * @param accountId 目标用户
 */
export function listTemplate(accountId: string): string {
    return storage.getString(accountId + "_templates")!;
}

/**
 * 判断字符串是否为空
 * @param str 目标字符串
 */
function isBlank(str: string): boolean {
    if (str == null || str.trim().length < 1) {
        return true;
    }

    return false;
}
