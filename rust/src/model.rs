/// # About This File
/// There are three concepts for each user in SayHi:
/// ### Template:
/// User can use templates to pre-define some kinds of cards.
/// ### Card:
/// The detail of user's card are defined here.
/// ### Contact: 
/// Descibe user's contacts.
/// 
use borsh::{BorshDeserialize, BorshSerialize};

pub type TemplateID = String;
pub type CardID = String;
pub type AccountID = String;

// 模板model, 一期中仅定义模版结构, 不实现模版相关业务逻辑
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Template {
    pub id: TemplateID,         // 模板唯一编号
    pub name: String,       // 模板名称, 也是模版生成卡片的默认名称
    pub content: String,    // 模版内容, 也是模版生成卡片的默认公共消息内容
    pub owner: AccountID,      // 创建者
    pub current_block: u64, // 模板创建时块高
    pub duration: u64,      // 模板超时块数
}

impl Template {
    pub fn new(id: &str, new_name: &str, content: &str, owner: &str, new_current_block: u64, new_duration: u64) -> Self {
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
pub struct MyCard {
    pub id: String,              // 卡片唯一编号
    pub template_id: String,     // 所属模版编号, 没有模版则为空串
    
    // 卡片信息
    pub name: String,            // 卡片名称
    pub public_message: String,  // 公开消息
    pub private_message: String, // 私密消息
    
    // 收发人相关
    pub creator: String,        // 卡片创建人
    pub card_type: u8,           // 卡片类型0为不定向多人，1为联系人卡片
    pub specify_account: String, // 指定接收人，当card_type为1时必填
    pub count: u64,              // 卡片数量, 联系人卡片时该值视为1
    pub remaining_count: u64,    // 剩余卡片数量, 即还能被接收的次数
    // 红包相关
    pub is_avg: bool,            // true代表平均红包, false代表随机红包
    pub total: u64,              // 红包总金额
    pub remaining_total: u64,    // 剩余未领金额
    // 链属性
    pub current_block: u64,      // 卡片创建时块高
    pub duration: u64,           // 卡片超时块数
}

impl MyCard {
    pub fn new(
        id: &str, template_id: &str,
        name: &str, public_message: &str, private_message: &str,
        creator: &str, card_type: u8, specify_account: &str, count: u64,
        is_avg: bool, total: u64,
        current_block: u64, duration: u64
    ) -> Self {
        MyCard {
            id: String::from(id),
            template_id: String::from(template_id),
            name: String::from(name),
            public_message: String::from(public_message),
            private_message: String::from(private_message),
            creator: String::from(creator),
            card_type: card_type,
            specify_account: String::from(specify_account),
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

// 名片model
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Card {
    pub id: String,              // 名片唯一编号
    pub template_id: String,     // 模板唯一编号
    pub card_type: u8,           // 卡片类型0为不定向多人，1为指定某人
    pub public_message: String,  // 公开消息
    pub private_message: String, // 私密消息
    pub name: String,            // 名片名称
    pub count: u64,              // 名片数量
    pub remaining_count: u64,    // 剩余名片数量
    pub is_avg: bool,            // 是否均分
    pub total: u64,              // 总红包
    pub remaining_total: u64,    // 剩余总红包
    pub current_block: u64,      // 名片创建时块高
    pub duration: u64,           // 名片超时块数
    pub specify_account: String, // 指定接收人，当card_type为1时必填
}

impl Card {
    pub fn new(
        id: String,
        template_id: String,
        card_type: u8,
        public_message: String,
        private_message: String,
        new_name: String,
        new_count: u64,
        is_avg: bool,
        new_total: u64,
        new_current_block: u64,
        new_duration: u64,
        new_specify_account: String,
    ) -> Self {
        Card {
            id,
            template_id,
            card_type: card_type,
            public_message: public_message,
            private_message: private_message,
            name: new_name,
            count: new_count,
            remaining_count: new_count,
            is_avg: is_avg,
            total: new_total,
            remaining_total: new_total,
            current_block: new_current_block,
            duration: new_duration,
            specify_account: new_specify_account,
        }
    }
}

// 联系人(收到的卡片以及被接收者查看后创建的联系人)model
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct ContactPerson {
    pub id: String,             // 联系人唯一编号
    pub contact_person: String, //联系人姓名
    pub card_count: u64,        // 收到的卡片数量
    pub duration: u64,          // 名片超时块数
}

impl ContactPerson {
    pub fn new(id: String, new_contact_person: String, new_card_count: u64, duration: u64) -> Self {
        ContactPerson {
            id: id,
            contact_person: new_contact_person,
            card_count: new_card_count,
            duration: duration,
        }
    }
}