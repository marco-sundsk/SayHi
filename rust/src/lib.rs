use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::collections::Map;
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

// 模板model
type Template = model::Template;
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
    cards: Map<CardID, Card>,
    card_created: Map<AccountID, Vec<CardID>>, 
    card_recv: Map<AccountID, Vec<CardID>>,

    

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
        template_id: String,
        card_type: u8,
        public_message: String,
        private_message: String,
        name: String,
        count: u64,
        total: u64,
        duration: u64,
        specify_account: String,
    ) -> String {
        if card_type != 0 && card_type != 1 {
            return "".to_string();
        }

        let account_id = env::signer_account_id();

        if let None = self.card_record.get(&account_id) {
            self.card_record.insert(&account_id, &Map::default());
            env::log("create new entry".as_bytes());
        }

        // let mut cards: Map<String, Card> = Map::default();

        if let Some(mut map) = self.card_record.get(&account_id) {
            let current_block_index = env::block_index();
            let random_seed = env::random_seed();
            let id_str = random_seed
                .iter()
                .map(|&c| format!("{:x?}", c))
                .collect::<String>();

            // 创建卡片
            let new_card = Card::new(
                id_str.to_string(),
                template_id,
                card_type,
                public_message,
                private_message,
                name.to_string(),
                count,
                true,
                total,
                current_block_index,
                duration,
                specify_account,
            );

            map.insert(&id_str, &new_card);
            env::log("insert new value".as_bytes());
            self.card_account_relation.insert(&id_str, &account_id);
            return id_str;
            // self.card_record.insert(&account_id)
            // self.card_record.insert(&account_id, &cards);
        }
        
        
        return "".to_string();
    }

    // 列出指定账号的名片信息
    pub fn list_card(&self, account_id: String) -> Option<Vec<HashMap<String, String>>> {
        self.card_record.get(&account_id).map(|record| {
            let mut temp: Vec<HashMap<String, String>> = Vec::new();

            for key in record.keys() {
                let item = record.get(&key).unwrap();
                let mut temp_map: HashMap<String, String> = HashMap::new();
                temp_map.insert(String::from("id"), item.id.to_string());
                temp_map.insert(String::from("template_id"), item.template_id.to_string());
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
                temp.push(temp_map);
            }

            temp
        })
    }

    // 通过 card 查询创建人
    pub fn find_account_by_card(&self, card_id: String) -> Option<String> {
        self.card_account_relation.get(&card_id)
    }

    // 扫卡人创建联系人
    pub fn create_contract_person(
        &mut self,
        contact_person: String,
        card_id: String,
        duration: u64,
    ) -> bool {
        // 1、判断卡是否存在；2、判断是否是不定向卡片
        let car_obj = self.card_account_relation.get(&card_id); // car_obj 值为创建人

        if let None = car_obj {
            env::log("卡片不存在".as_bytes());
            return false;
        }

        // card_vec 为 car_obj 创建的所有卡片列表，在此卡片列表中查找卡片
        let card_map = self.card_record.get(&car_obj.unwrap()).unwrap();
        let card_item = card_map.get(&card_id).unwrap();
        if card_item.id == card_id {
            if card_item.card_type == 1 && card_item.remaining_count == 0 {
                // 不定向
                return false;
            }
        }

        let account_id = env::signer_account_id();
        let mut scan_result_key = String::from(&card_id);
        scan_result_key.push_str("_");
        scan_result_key.push_str(&account_id);

        // 此处用于判断是否已经扫描过
        if let Some(_) = self.card_scan_result.get(&scan_result_key) {
            return false;
        }

        let mut contact_person_vec: Vec<ContactPerson> = Vec::new();

        if let Some(list) = self.contract_person.get(&account_id) {
            contact_person_vec = list.to_vec();
        }

        // 当联系人存在时删除后进行添加
        let mut temp_count: usize = 0;
        let mut old_card_count = 0;

        for item in &contact_person_vec {
            if item.contact_person == contact_person {
                old_card_count = item.card_count;
                break;
            }
            temp_count = temp_count + 1;
        }

        if old_card_count != 0 {
            contact_person_vec.remove(temp_count);
        }

        let random_seed = env::random_seed();
        let id_str = random_seed
            .iter()
            .map(|&c| format!("{:x?}", c))
            .collect::<String>();
        let new_contract_person = ContactPerson::new(
            id_str.to_string(),
            contact_person.to_string(),
            old_card_count as u64 + 1,
            duration,
        );

        contact_person_vec.push(new_contract_person);
        self.contract_person
            .insert(&account_id, &contact_person_vec);
        self.card_scan_result.insert(&scan_result_key, &card_item);

        return true;
    }

    // 扫描人为发卡人创建联系人
    pub fn create_contract_person_for_sender(&mut self, sender: String, duration: u64) -> bool {
        let account_id = env::signer_account_id();
        let mut contact_person_vec: Vec<ContactPerson> = Vec::new();

        if let Some(list) = self.contract_person.get(&sender) {
            contact_person_vec = list.to_vec();
        }

        // 当联系人存在时删除后进行添加
        let mut temp_count: usize = 0;
        let mut old_card_count = 0;

        for item in &contact_person_vec {
            if item.contact_person == account_id {
                old_card_count = item.card_count;
                break;
            }
            temp_count = temp_count + 1;
        }

        if contact_person_vec.len() > temp_count {
            contact_person_vec.remove(temp_count);
        }

        let random_seed = env::random_seed();
        let id_str = random_seed
            .iter()
            .map(|&c| format!("{:x?}", c))
            .collect::<String>();
        let new_contract_person = ContactPerson::new(
            id_str.to_string(),
            account_id.to_string(),
            old_card_count as u64,
            duration,
        );
        contact_person_vec.push(new_contract_person);
        self.contract_person.insert(&sender, &contact_person_vec);
        return true;
    }

    // 获取联系人列表
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
                    println!("{}", _temp.len());
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
        // let create_result = bl_card_service.create_card(
        //     "template_id".to_string(),
        //     0,
        //     "".to_string(),
        //     "".to_string(),
        //     "".to_string(),
        //     1,
        //     1,
        //     100,
        //     "".to_string(),
        // );
        // assert_eq!(create_result, "true");
        // let _templates = bl_card_service.list_card("bob_near".to_string());

        // match _templates {
        //     None => assert_eq!(1, 1),
        //     Some(_temp) => assert_eq!(_temp.len(), 1),
        // }
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
