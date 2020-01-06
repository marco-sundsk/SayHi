import { logging, storage, context } from "near-runtime-ts";
// available class: near, context, storage, logging, base58, base64,
// PersistentMap, PersistentVector, PersistentDeque, PersistentTopN, ContractPromise, math
import { TextMessage } from "./model";
import { isBlank } from "./util";

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

const CARD_SUFFIX = "_cards";
const RECI_CARD_SUFFIX = "_reci_cards";
const TEMPLATE_SUFFIX = "_templates";
const CONTRACT_SUFFIX = "_contracts";

/**
 * 创建（更新）模板
 * @param templateInfo 模板信息
 */
export function createTemplate(templateInfo: string): boolean {
    if (!context.sender) {
        return false;
    }

    storage.setString(context.sender + TEMPLATE_SUFFIX, templateInfo);
    return true;
}

/**
 * 查询创建的模板
 */
export function listTemplate(): string {
    return storage.getString(context.sender + TEMPLATE_SUFFIX)!;
}

/**
 * 创建（更新）新卡
 * cardInfo 推荐使用json，且包含以下信息
 * cardName: string,
 * publicInfo: string,
 * privateInfo: string,
 * count: i32=1,
 * total: f32=0.0,
 * isAvg: boolean=true
 * @param cardInfo 需创建的卡片信息
 */
export function createCard(cardInfo: string): boolean {

    let result = true;

    let sender = context.sender;

    if (!sender) {
        logging.log("需先登录获取用户信息");
        return false;
    } else if (isBlank(cardInfo)) {
        logging.log("cardInfo 为空");
        return false;
    }

    storage.setString(sender + CARD_SUFFIX, cardInfo);

    return result;
}

/**
 * 查询卡片列表
 */
export function listCard(): string {
    logging.log(context.sender)
    return storage.getString(context.sender + CARD_SUFFIX)!;
}

/**
 * 创建接收的卡片信息
 * @param cardInfo 接收到的卡片信息
 */
export function createReciCard(cardInfo: string): boolean {
    let result = true;

    let sender = context.sender;

    if (!sender) {
        logging.log("需先登录获取用户信息");
        return false;
    } else if (isBlank(cardInfo)) {
        logging.log("cardInfo 为空");
        return false;
    }

    storage.setString(sender + RECI_CARD_SUFFIX, cardInfo);

    return result;
}

/**
 * 创建（更新）联系人
 * @param contractInfo 联系人信息
 */
export function createContract(contractInfo: string, newContract: string, newContractInfo: string): boolean {
    let result = true;

    let sender = context.sender;

    if (!sender) {
        logging.log("需先登录获取用户信息");
        return false;
    } else if (isBlank(contractInfo)) {
        logging.log("contractInfo 为空");
        return false;
    } else if (isBlank(newContract)) {
        logging.log("newContract 为空");
        return false;
    } else if (isBlank(newContractInfo)) {
        logging.log("newContractInfo 为空");
        return false;
    }

    storage.setString(sender + CONTRACT_SUFFIX, contractInfo);
    storage.setString(newContract + CONTRACT_SUFFIX, newContractInfo);

    return result;
}

/**
 * 查询联系人
 */
export function listContract(contract: string): string {

    if (isBlank(contract)) {
        return storage.getString(context.sender + CONTRACT_SUFFIX)!;
    } else {
        return storage.getString(contract + CONTRACT_SUFFIX)!;
    }
}