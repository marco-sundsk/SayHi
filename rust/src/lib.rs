use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::collections::{Map, Set};
use near_bindgen::{env, near_bindgen};
use std::collections::HashMap;
pub mod model;
use model::{TemplateID, CardID, AccountID};

// 1、创建模板
// 2、创建卡片 包含卡片标题、私密信息、公开信息、红包等
//      不定向发卡（接收到二维码的人扫描，仅能扫描一次）
//      指定人发卡
// 3、扫描卡片并创建联系人

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type Template = model::Template;
type SayHiCard = model::SayHiCard;

type Card = model::Card;
type ContactPerson = model::ContactPerson;

// 用于提供访问服务
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct BLCardService {
    // templates storage, each user has his own templates list
    templates: Map<TemplateID, Template>,
    user_templates: Map<AccountID, Vec<TemplateID>>, 

    // cards storage, each user has his own create-cards list and recv-cards list
    cards: Map<CardID, SayHiCard>,
    card_created: Map<AccountID, Vec<CardID>>, 
    card_recv: Map<AccountID, Set<CardID>>,

    // contracts storage, each user has his own contracts list
    user_contracts: Map<AccountID, Set<AccountID>>,



    card_record: Map<String, Map<String, Card>>, // 外层 key 为账号信息，value 为名片信息，内层 key 为cardId，value 为card
    card_account_relation: Map<String, String>, // key 为名片唯一编号，value 为账号信息，用于反向查找
    card_scan_result: Map<String, Card>,        // key 为  名片_扫描人 唯一编号，value 为是否

    contract_person: Map<String, Vec<ContactPerson>>, // key 为账号信息，value 为联系人列表
}

#[near_bindgen]
impl BLCardService {

    // TODO: 用户内置卡相关操作
    // 每个用户默认都有一张内置卡，记录发给他的私密信息的加密公钥
    // 前端获取用户的发卡列表时，检查内置卡的公钥是否与本地私钥匹配，如不匹配，主动发起更新内置卡操作

    // 创建模板
    pub fn create_template(&mut self, name: &str, content: &str, duration: u64) -> bool {
        // 获取调用人身份
        let account_id = env::signer_account_id();

        // 创建模版对象
        let current_block_index = env::block_index();
        let random_seed = env::random_seed();
        let id_str = random_seed
            .iter()
            .map(|&c| format!("{:x?}", c))
            .collect::<String>();

        let new_template = Template::new(
            &id_str,
            name,
            content,
            &account_id,
            current_block_index,
            duration,
        );
        self.templates.insert(&id_str, &new_template);

        // 关联到用户
        if let Some(mut list) = self.user_templates.get(&account_id) {
            // 用户已存在
            // templates = list.to_vec();
            list.push(id_str);
        } else {
            self.user_templates.insert(&account_id, &vec![id_str]);
        }
        true
    }

    // 列出指定账号的模板信息
    pub fn list_template(&self, account_id: &str) -> Option<Vec<HashMap<String, String>>> {
        let mut rslt: Vec<HashMap<String, String>> = Vec::new();

        self.user_templates.get(&String::from(account_id)).map(|records| {    
            for tid in records.iter() {
                if let Some(item) = self.templates.get(&tid) {
                    let mut temp_map: HashMap<String, String> = HashMap::new();
                    temp_map.insert(String::from("id"), item.id.to_string());
                    temp_map.insert(String::from("name"), item.name.to_string());
                    temp_map.insert(String::from("content"), item.content.to_string());
                    temp_map.insert(String::from("duration"), format!("{}", item.duration));
                    rslt.push(temp_map);
                }
            }
            rslt
        })
    }

    // 创建名片
    pub fn create_card(
        &mut self,
        template_id: &str,
        card_type: u8,
        public_message: &str,
        private_message: &str,
        name: &str,
        count: u64,
        total: u64,
        duration: u64,
        specify_account: &str,
    ) -> String {
        // 入参检查, 卡类型在存储时已归入target, 这里需要逻辑转换
        if card_type != 0 && card_type != 1 {
            return "".to_string();
        }
        // 创建卡
        let account_id = env::signer_account_id();
        let current_block_index = env::block_index();
        let random_seed = env::random_seed();
        let id_str = random_seed
            .iter()
            .map(|&c| format!("{:x?}", c))
            .collect::<String>();
        let rslt = id_str.to_string();
        
        let new_card = SayHiCard::new(
            &id_str,
            None,  // 模版功能暂未提供
            name,
            public_message,
            private_message,
            &account_id,
            {
                if card_type == 0 {
                    None
                } else {
                    Some(specify_account.to_string())
                }
            },
            count,
            true,  // random 红包功能暂未提供
            total,
            current_block_index,
            duration,
        );
        self.cards.insert(&id_str, &new_card);
        env::log("create new entry".as_bytes());

        if let Some(mut sends) = self.card_created.get(&account_id) {
            sends.push(id_str);
        } else {
            self.card_created.insert(&account_id, &vec![id_str]);
        }
        rslt
    }

    // 列出指定账号的创建名片信息
    pub fn list_card(&self, account_id: &str) -> Option<Vec<HashMap<String, String>>> {
        let mut rslt: Vec<HashMap<String, String>> = Vec::new();

        self.card_created.get(&String::from(account_id)).map(|records| {    
            for cid in records.iter() {
                if let Some(item) = self.cards.get(&cid) {
                    let mut temp_map: HashMap<String, String> = HashMap::new();
                    temp_map.insert(String::from("id"), item.id.to_string());
                    temp_map.insert(
                        String::from("template_id"), 
                        item.tid.unwrap_or(String::from("").to_string()),
                    );
                    temp_map.insert(
                        String::from("public_message"),
                        item.public_message.to_string(),
                    );
                    temp_map.insert(
                        String::from("private_message"),
                        item.private_message.to_string(),
                    );
                    temp_map.insert(String::from("name"), item.name.to_string());
                    temp_map.insert(String::from("count"), format!("{}", item.count));
                    temp_map.insert(String::from("total"), format!("{}", item.total));
                    temp_map.insert(String::from("duration"), format!("{}", item.duration));
                    rslt.push(temp_map);
                }
            }
            rslt
        })
    }

    // 收卡人扫卡
    pub fn scan_card(&mut self, card_id: &str) -> Option<HashMap<String, String>> {
        let account_id = env::signer_account_id();
        // 1. 找到卡
        if let Some(card) = self.cards.get(&String::from(card_id)) {
            // 
            // 2. 卡片是否可收取
            if let Some(target) = card.target {
                // 定向卡片，判断是否给我
                if target != account_id {
                    env::log("定向卡片只能由特定人收取".as_bytes());
                    return None;
                }
            } else {
                // 不定向卡片，判断个数是否已满
                if card.remaining_count == 0 {
                    env::log("卡片收取数已满".as_bytes());
                    return None;
                }
            }
            // 3. 收取卡片
            if let None = self.card_recv.get(&account_id) {
                self.card_recv.insert(&account_id, &Set::default());
            } 
            self.card_recv.get(&account_id).unwrap().insert(&String::from(card_id));
            // 4. 互加联系人
            if let None = self.user_contracts.get(&account_id) {
                self.user_contracts.insert(&account_id, &Set::default());
            } 
            self.user_contracts.get(&account_id).unwrap().insert(&card.creator);
            if let None = self.user_contracts.get(&card.creator) {
                self.user_contracts.insert(&card.creator, &Set::default());
            } 
            self.user_contracts.get(&card.creator).unwrap().insert(&account_id);
            // 5. 返回卡信息
            let mut temp_map: HashMap<String, String> = HashMap::new();
            temp_map.insert(String::from("id"), card.id.to_string());
            temp_map.insert(
                String::from("template_id"), 
                card.tid.unwrap_or(String::from("").to_string()),
            );
            temp_map.insert(
                String::from("public_message"),
                card.public_message.to_string(),
            );
            temp_map.insert(
                String::from("private_message"),
                card.private_message.to_string(),
            );
            temp_map.insert(String::from("name"), card.name.to_string());
            temp_map.insert(String::from("count"), format!("{}", card.count));
            temp_map.insert(String::from("total"), format!("{}", card.total));
            temp_map.insert(String::from("duration"), format!("{}", card.duration));
            Some(temp_map)
        } else {
            env::log("卡片不存在".as_bytes());
            None
        }
    }


    // TODO: 获取联系人列表
    pub fn list_contract_person(&self, account_id: String) -> Option<Vec<HashMap<String, String>>> {
        self.contract_person.get(&account_id).map(|record| {
            let mut temp: Vec<HashMap<String, String>> = Vec::new();

            for item in record.iter() {
                let mut temp_map: HashMap<String, String> = HashMap::new();
                temp_map.insert(String::from("id"), item.id.to_string());
                temp_map.insert(
                    String::from("contact_person"),
                    item.contact_person.to_string(),
                );

                let mut need_key_section = String::from("_");
                need_key_section.push_str(&account_id);
                // TODO 此处结构需要修改，此时为临时方式
                let mut card_info = String::from("[{}");

                for scan_item in self.card_scan_result.keys() {
                    if scan_item.contains(&need_key_section.to_string()) {
                        let scan_card_item: Card = self.card_scan_result.get(&scan_item).unwrap();
                        card_info.push_str(",{");

                        card_info.push_str("\"id\":");
                        card_info.push_str("\"");
                        card_info.push_str(&scan_card_item.id);
                        card_info.push_str("\"");
                        card_info.push(',');
                        card_info.push_str("\"name\":");
                        card_info.push_str("\"");
                        card_info.push_str(&scan_card_item.name);
                        card_info.push_str("\"");
                        card_info.push(',');
                        card_info.push_str("\"total\":");
                        card_info.push_str("\"");
                        card_info.push_str("10"); // TODO 需要获取红包总数
                        card_info.push_str("\"");
                        card_info.push(',');

                        card_info.push_str("\"template_id\":");
                        card_info.push_str("\"");
                        card_info.push_str(&scan_card_item.template_id);
                        card_info.push_str("\"");
                        card_info.push(',');

                        card_info.push_str("\"public_message\":");
                        card_info.push_str("\"");
                        card_info.push_str(&scan_card_item.public_message);
                        card_info.push_str("\"");
                        card_info.push(',');

                        card_info.push_str("\"private_message\":");
                        card_info.push_str("\"");
                        card_info.push_str(&scan_card_item.private_message);
                        card_info.push_str("\"");

                        card_info.push('}');
                    }
                }

                card_info.push(']');

                temp_map.insert(String::from("card_count"), format!("{}", item.card_count));
                temp_map.insert(String::from("card_list"), card_info);
                temp_map.insert(String::from("duration"), format!("{}", item.duration));
                temp.push(temp_map)
            }

            temp
        })
    }

    pub fn t(&self) -> String {
        let random_seed = env::random_seed();
        let id_str = random_seed
            .iter()
            .map(|&c| format!("{:x?}", c))
            .collect::<String>();

        id_str
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_bindgen::MockedBlockchain;
    use near_bindgen::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(9),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn test_template_method() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let _template_name = String::from("new template 1");
        let _content = String::from("This is template's content.");

        let mut bl_card_service = BLCardService::default();
        let create_result = bl_card_service.create_template(&_template_name, &_content, 100);
        assert_eq!(create_result, true);

        let _templates = bl_card_service.list_template("bob_near");

        match _templates {
            None => assert_eq!(1, 2),
            Some(_temp) => {
                    assert_eq!(_temp.len(), 1);
                    println!("Templates count: {}", _temp.len());
                    println!("{:#?}", _temp[0]);
                },
        }
    }

    #[test]
    fn test_card_method() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let _card_name = String::from("new card 1");

        let mut bl_card_service = BLCardService::default();
        let create_result = bl_card_service.create_card(
            "template_id",  // tid
            0,  // card_type
            "This is public msg", // public
            "This is private msg", // private
            "This is name",  // name
            1,  // count
            1,  // amount
            100,  // duration
            "Receiver",
        );
        assert_ne!(create_result, "");
        println!("Create card return: {}", create_result);
        let _cards = bl_card_service.list_card("bob_near");

        match _cards {
            None => assert_eq!(1, 2),
            Some(_temp) => {
                assert_eq!(_temp.len(), 1);
                println!("{:#?}", _temp[0]);
            },
        }
    }

    #[test]
    fn test_contract_person() {
        // let context = get_context(vec![], false);
        // testing_env!(context);
        // let mut bl_card_service = BLCardService::default();
        // let result = bl_card_service.t();
        // let result_str = result
        //     .iter()
        //     .map(|&c| {
        //         // let temp = c as char;
        //         format!("{:x?}", c)
        //     })
        //     .collect::<String>();
        // assert_eq!(
        //     result_str,
        //     "ae4b3280e56e2faf83f414a6e3dabe9d5fbe18976544c05fed121accb85b53fc"
        // );
    }
}
