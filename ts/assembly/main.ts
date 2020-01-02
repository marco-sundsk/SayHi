import { logging, context } from "near-runtime-ts";
// available class: near, context, storage, logging, base58, base64, 
// PersistentMap, PersistentVector, PersistentDeque, PersistentTopN, ContractPromise, math
import { TextMessage } from "./model";

const NAME = ". Welcome to NEAR Protocol chain"
export function welcome(name: string): TextMessage {
  logging.log("simple welcome test");
  let message = new TextMessage()
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