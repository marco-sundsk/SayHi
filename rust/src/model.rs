use borsh::{BorshDeserialize, BorshSerialize};

// 模板model
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Template {
    pub id: String,         // 模板唯一编号
    pub name: String,       // 模板名称
    pub current_block: u64, // 模板创建时块高
    pub duration: u64,      // 模板超时块数
}

impl Template {
    pub fn new(id: String, new_name: String, new_current_block: u64, new_duration: u64) -> Self {
        Template {
            id: id,
            name: new_name,
            current_block: new_current_block,
            duration: new_duration,
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