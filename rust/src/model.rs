/// # About This File
/// There are three concepts for each user in SayHi:
/// ### Template:
/// User can use templates to pre-define some kinds of cards.
/// ### Card:
/// The detail of user's card are defined here.
/// ### Contact:
/// Contract info are analysed from cards info.
///
use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashSet;

pub type TemplateID = String;
pub type CardID = String;
pub type AccountID = String;
pub type CertificateID = String;

// 模板model, 一期中仅定义模版结构, 不实现模版相关业务逻辑
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Template {
    pub id: TemplateID,     // 模板唯一编号
    pub name: String,       // 模板名称, 也是模版生成卡片的默认名称
    pub content: String,    // 模版内容, 也是模版生成卡片的默认公共消息内容
    pub owner: AccountID,   // 创建者
    pub current_block: u64, // 模板创建时块高
    pub duration: u64,      // 模板超时块数
}

impl Template {
    pub fn new(
        id: &str,
        new_name: &str,
        content: &str,
        owner: &str,
        new_current_block: u64,
        new_duration: u64,
    ) -> Self {
        Template {
            id: String::from(id),
            name: String::from(new_name),
            content: String::from(content),
            owner: String::from(owner),
            current_block: new_current_block,
            duration: new_duration,
        }
    }
}

// 卡片model
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct SayHiCard {
    pub id: CardID,              // 卡片唯一编号
    pub tid: Option<TemplateID>, // 所属模版编号
    // 卡片信息
    pub name: String,            // 卡片名称
    pub public_message: String,  // 公开消息
    pub private_message: String, // 私密消息
    // 收发人相关
    pub creator: AccountID,        // 卡片创建人
    pub target: Option<AccountID>, // 卡片目标人, 为自身时代表内置卡片, None代表不定向卡片

    pub count: u64,           // 卡片数量, 联系人卡片时该值视为1
    pub remaining_count: u64, // 剩余卡片数量, 即还能被接收的次数
    // 红包相关
    pub is_avg: bool,         // true代表平均红包, false代表随机红包
    pub total: u64,           // 红包总金额
    pub remaining_total: u64, // 剩余未领金额
    // 链属性
    pub current_block: u64, // 卡片创建时块高
    pub duration: u64,      // 卡片超时块数
}

impl SayHiCard {
    pub fn new(
        id: &str,
        tid: Option<TemplateID>,
        name: &str,
        public_message: &str,
        private_message: &str,
        creator: &str,
        target: Option<AccountID>,
        count: u64,
        is_avg: bool,
        total: u64,
        current_block: u64,
        duration: u64,
    ) -> Self {
        SayHiCard {
            id: String::from(id),
            tid: tid,
            name: String::from(name),
            public_message: String::from(public_message),
            private_message: String::from(private_message),
            creator: String::from(creator),
            target: target,
            count: count,
            remaining_count: count,
            is_avg: is_avg,
            total: total,
            remaining_total: total,
            current_block: current_block,
            duration: duration,
        }
    }
}

// 证书
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Certificate {
    pub id: CertificateID,
    pub public_key: String,
    pub contacts: HashSet<String>,
    pub other_attrs: String,
}

impl Certificate {
    pub fn new(id: &str, public_key: &str, contacts: HashSet<String>, other_attrs: &str) -> Self {
        Certificate {
            id: String::from(id),
            public_key: String::from(public_key),
            contacts: contacts,
            other_attrs: String::from(other_attrs),
        }
    }
}
