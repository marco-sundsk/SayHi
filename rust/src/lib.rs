use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::collections::Map;
use near_bindgen::{env, near_bindgen};
use std::collections::HashMap;
pub mod model;

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
    template_record: Map<String, Vec<Template>>, // key 为账号信息，value 为模板列表
    template_account_relation: Map<String, String>, // key 为模板唯一编号，value 为账号信息，用于反向查找

    card_record: Map<String, Map<String, Card>>, // 外层 key 为账号信息，value 为名片信息，内层 key 为cardId，value 为card
    card_account_relation: Map<String, String>, // key 为名片唯一编号，value 为账号信息，用于反向查找
    card_scan_result: Map<String, bool>,        // key 为  名片_扫描人 唯一编号，value 为是否

    contract_person: Map<String, Vec<ContactPerson>>, // key 为账号信息，value 为联系人列表
}

#[near_bindgen]
impl BLCardService {
    // 创建模板
    pub fn create_template(&mut self, name: String, duration: u64) -> bool {
        let account_id = env::signer_account_id();
        let mut templates: Vec<Template> = Vec::new();

        if let Some(list) = self.template_record.get(&account_id) {
            templates = list.to_vec();
        }

        let current_block_index = env::block_index();
        let new_template = Template::new(
            name.to_string(),
            name.to_string(),
            current_block_index,
            duration,
        ); // TODO 第一个参数应该为template id
        templates.push(new_template);
        self.template_record.insert(&account_id, &templates);
        self.template_account_relation
            .insert(&account_id, &account_id); // TODO 第二个参数应该为template id

        return true;
    }

    // 列出指定账号的模板信息
    pub fn list_template(&self, account_id: String) -> Option<Vec<HashMap<String, String>>> {
        self.template_record.get(&account_id).map(|record| {
            let mut temp: Vec<HashMap<String, String>> = Vec::new();

            for item in record.iter() {
                let mut temp_map: HashMap<String, String> = HashMap::new();
                temp_map.insert(String::from("id"), item.id.to_string());
                temp_map.insert(String::from("name"), item.name.to_string());
                temp_map.insert(String::from("duration"), format!("{}", item.duration));
                temp.push(temp_map);
            }

            temp
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
    ) -> bool {
        if card_type != 0 && card_type != 1 {
            return false;
        }
        // 根据card_type判断为指定给某人的card或不定向
        // 给指定人发card，是否需要知道扫没扫描
        // 给指定人发card前，发送人联系人列表里有接收人，接收人列表里有发送人嘛，还是一定要等接收人扫了才会有
        let account_id = env::signer_account_id();
        let mut cards: Map<String, Card> = Map::default();

        if let Some(map) = self.card_record.get(&account_id) {
            cards = map;
        }

        let current_block_index = env::block_index();
        // 创建卡片
        let new_card = Card::new(
            name.to_string(),
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
        ); // TODO 第一个参数应该为card id

        cards.insert(&name, &new_card); // TODO 第一个参数应该为card id
        self.card_record.insert(&account_id, &cards);
        self.card_account_relation.insert(&name, &account_id); // TODO 第二个参数应该为card id
        return true;
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

    // 通过 template 查询创建人
    pub fn find_account_by_template(&self, template_id: String) -> Option<String> {
        self.template_account_relation.get(&template_id)
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
        duration: u64,) -> bool {
        // 1、判断卡是否存在；2、判断是否是不定向卡片
        let car_obj = self.card_account_relation.get(&card_id); // car_obj 值为创建人

        if let None = car_obj {
            env::log("卡片不存在".as_bytes());
            return false;
        }

        // card_vec 为 car_obj 创建的所有卡片列表，在此卡片列表中查找卡片
        let card_map = self.card_record.get(&car_obj.unwrap()).unwrap();
        let item = card_map.get(&card_id).unwrap();
        if item.id == card_id {
            if item.card_type == 1 && item.remaining_count == 0 {
                // 不定向
                return false;
            }
        }

        let mut scan_result_key = String::from(&card_id);
        scan_result_key.push_str("_");
        scan_result_key.push_str(&contact_person);

        // 此处用于判断是否已经扫描过
        if let Some(_) = self.card_scan_result.get(&scan_result_key) {
            return false;
        }

        let account_id = env::signer_account_id();
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
        contact_person_vec.remove(temp_count);
        let new_contract_person = ContactPerson::new(
            contact_person.to_string(),
            contact_person.to_string(),
            old_card_count as u64 + 1,
            duration,
        ); // TODO 第一个参数应该生成；第三个参数应该去查询已经收到的数量

        contact_person_vec.push(new_contract_person);
        self.contract_person
            .insert(&account_id, &contact_person_vec);
        self.card_scan_result.insert(&scan_result_key, &true);

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
        contact_person_vec.remove(temp_count);
        let new_contract_person = ContactPerson::new(
            account_id.to_string(),
            account_id.to_string(),
            old_card_count as u64,
            duration,
        ); // TODO 第一个参数应该生成；第三个参数应该去查询已经收到的数量
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
                temp_map.insert(String::from("card_count"), format!("{}", item.card_count));
                temp_map.insert(String::from("duration"), format!("{}", item.duration));
                temp.push(temp_map)
            }

            temp
        })
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

        let mut bl_card_service = BLCardService::default();
        let create_result = bl_card_service.create_template(_template_name.to_string(), 100);
        assert_eq!(create_result, true);
        let _templates = bl_card_service.list_template("bob_near".to_string());

        match _templates {
            None => assert_eq!(1, 1),
            Some(_temp) => assert_eq!(_temp.len(), 1),
        }
    }

    #[test]
    fn test_card_method() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let _card_name = String::from("new card 1");

        // let mut bl_card_service = BLCardService::default();
        // let create_result = bl_card_service.create_card(
        //     "".to_string(),
        //     "".to_string(),
        //     "".to_string(),
        //     _card_name.to_string(),
        //     100,
        // );
        // assert_eq!(create_result, true);
        // let _templates = bl_card_service.list_card("bob_near".to_string());

        // match _templates {
        //     None => assert_eq!(1, 1),
        //     Some(_temp) => assert_eq!(_temp.len(), 1),
        // }
    }
}
