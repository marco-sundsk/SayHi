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
 * @param templateName 模板名称
 * @param templateContent 模板内容
 * @param duration
 */
export function createTemplate(
    accountId: string,
    templateName: string,
    templateContent: string,
    duration: string = "0"
): boolean {
	let template = new TemplateModel();
	template.id = math.hash32(math.randomSeed()); // TODO 此处应该有随机生成编号
    template.name = templateName;
    template.content = templateContent;
    template.duration = duration;
    storage.setBytes(accountId, template.encode());
	
    return true;
}

export function updateTemplate(accountId: string): string {
	return "";
	// return TemplateModel.decode(storage.getBytes(accountId));
}