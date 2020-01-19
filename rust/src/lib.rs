use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::collections::Map;
use near_bindgen::{env, near_bindgen, Promise};
use std::collections::{HashMap, HashSet};
pub mod model;
use model::{TemplateID, CardID, AccountID, CertificateID};

// 1、创建模板
// 2、创建卡片 包含卡片标题、私密信息、公开信息、红包等
//      不定向发卡（接收到二维码的人扫描，仅能扫描一次）
//      指定人发卡
// 3、扫描卡片并创建联系人

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type Template = model::Template;
type SayHiCard = model::SayHiCard;
type Certificate = model::Certificate;

// const SCHOLARSHIP_AMOUNT: u128 = 1 * NEAR_BASE;
const NEAR_BASE: u128 = 1_000_000_000_000_000_000_000_000;

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
    card_recv: Map<AccountID, HashSet<CardID>>,
    card_scan_result: Map<CardID, HashMap<AccountID, u64>>,

    // Cert describe user's nature attributes
    certificates: Map<CertificateID, Certificate>,
    user_certificates: Map<AccountID, Vec<CertificateID>>,

    // contracts storage, each user has his own contracts list
    user_contacts: Map<AccountID, HashSet<AccountID>>,
}

#[near_bindgen]
impl BLCardService {

    // TODO: 用户内置卡相关操作
    // 每个用户默认都有一张内置卡，记录发给他的私密信息的加密公钥
    // 前端获取用户的发卡列表时，检查内置卡的公钥是否与本地私钥匹配，如不匹配，主动发起更新内置卡操作

    // 创建模板
    pub fn create_template(&mut self, name: &String, content: &String, duration: u64) -> bool {
        // 获取调用人身份
        let account_id = env::signer_account_id();

        // 创建模版对象
        let current_block_index = env::block_index();
        let id_str = self.gen_id();

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
    pub fn list_template(&self) -> Option<Vec<HashMap<String, String>>> {
        let mut rslt: Vec<HashMap<String, String>> = Vec::new();
        let account_id = env::signer_account_id();
        self.user_templates.get(&account_id).map(|records| {    
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
        template_id: &String,
        card_type: u8,
        public_message: &String,
        private_message: &String,
        name: &String,
        count: u64,
        total: u64,
        duration: u64,
        is_rand: bool,
        specify_account: &String,
    ) -> String {
        // 入参检查, 卡类型在存储时已归入target, 这里需要逻辑转换
        if card_type != 0 && card_type != 1 {
            return "".to_string();
        }
        // 创建卡
        let account_id = env::signer_account_id();
        let current_block_index = env::block_index();
        let id_str = self.gen_id();
        let rslt = id_str.to_string();
        
        let new_card = SayHiCard::new(
            &id_str,
            Some(String::from(template_id)),  // 模版功能暂未提供
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
            !is_rand,  // random 红包功能暂未提供
            total,
            current_block_index,
            duration,
        );
        self.cards.insert(&id_str, &new_card);
        env::log("create new entry".as_bytes());

        if let Some(mut sends) = self.card_created.get(&account_id) {
            sends.push(id_str);
            self.card_created.insert(&account_id, &sends);
        } else {
            self.card_created.insert(&account_id, &vec![id_str]);
        }
        rslt
    }

    // 列出自己创建的名片信息
    pub fn list_card(&self) -> Option<Vec<HashMap<String, String>>> {
        let mut rslt: Vec<HashMap<String, String>> = Vec::new();
        let account_id = env::signer_account_id();
        // env::log(format!("signer_account_id: {}", env::signer_account_id()).as_bytes());
        // env::log(format!("current_account_id: {}", env::current_account_id()).as_bytes());
        self.card_created.get(&account_id).map(|records| {    
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
    pub fn scan_card(&mut self, card_id: &String) -> Option<HashMap<String, String>> {
        let account_id = env::signer_account_id();
        // 1. 找到卡
        if let Some(mut card) = self.cards.get(card_id) {
            // 
            // 2. 卡片是否可收取
            if let Some(target) = &card.target {
                // 定向卡片，判断是否给我
                if target.to_string() != account_id {
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

            if account_id == card.creator {
                // 定向卡片，判断是否自己创建
                env::log("卡片不能自收".as_bytes());
                return None;
            }

            // 3. 收取卡片
            if let None = self.card_recv.get(&account_id) {
                self.card_recv.insert(&account_id, &HashSet::new());
            } 

            self.card_recv.get(&account_id).unwrap().insert(String::from(card_id));

            if let Some(mut item) = self.card_recv.get(&account_id) {
                item.insert(String::from(card_id));
                self.card_recv.insert(&account_id, &item);
            }
            // 4. 互加联系人
            if let None = self.user_contacts.get(&account_id) {
                self.user_contacts.insert(&account_id, &HashSet::new());
                env::log(format!("{} gen Set to store contact.", account_id).as_bytes());
            } 

            // self.user_contacts.get(&account_id).unwrap().insert(String::from(&card.creator));
            if let Some(mut item) = self.user_contacts.get(&account_id) {
                item.insert(String::from(&card.creator));
                self.user_contacts.insert(&account_id, &item);
            }
 
            env::log(format!("{} add {} to his contact.", account_id, card.creator).as_bytes());

            if let None = self.user_contacts.get(&card.creator) {
                self.user_contacts.insert(&card.creator, &HashSet::new());
                env::log(format!("{} gen Set to store contact.", card.creator).as_bytes());
            } 

            // self.user_contacts.get(&card.creator).unwrap().insert(String::from(&account_id));
            if let Some(mut item) = self.user_contacts.get(&card.creator) {
                item.insert(String::from(&account_id));
                self.user_contacts.insert(&card.creator, &item);
            }

            env::log(format!("{} add {} to his contact.", card.creator, account_id).as_bytes());

            // 5. 卡信息变动后进行更新
            if let None = self.card_scan_result.get(card_id) {
                self.card_scan_result.insert(&card_id, &HashMap::new());
            }

            let mut recv_total = 0u64;
            if let Some(mut item) = self.card_scan_result.get(card_id) {
                env::log(format!("transfer to {}.", account_id).as_bytes());

                if card.is_avg {
                    recv_total = card.total / card.count;
                } else {
                    if card.remaining_count - 1 > 0 {
                        recv_total = self.random_amount(card.total);
                    } else {
                        recv_total = card.total - card.remaining_total;
                    }
                }
                
                item.insert(String::from(&account_id), recv_total as u64);
                self.transfer(account_id, recv_total as u128 * NEAR_BASE);
                self.card_scan_result.insert(&card_id, &item);
            }
            
            card.remaining_count = card.remaining_count - 1;
            card.remaining_total = card.remaining_total - recv_total as u64;
        
            self.cards.insert(card_id, &card);

            // 6. 返回卡信息
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

    // 获取自己的联系人
    pub fn list_contacts(&self) -> Option<Vec<String>> {
        let account_id = env::signer_account_id();
        if let Some(contact_sets) = self.user_contacts.get(&account_id) {
            env::log(format!("{} has {} contacts.", account_id, contact_sets.len()).as_bytes());
            Some(contact_sets.iter().map(|item| String::from(item)).collect::<Vec<String>>())
        } else {
            env::log("您还没有任何联系人".as_bytes());
            None
        }
    }

    // 获取收到的来自某个联系人的卡片
    pub fn list_recvcard_by_contact(&self, contact: &String) -> Option<Vec<HashMap<String, String>>> {
        let mut rslt: Vec<HashMap<String, String>> = Vec::new();
        let account_id = env::signer_account_id();
        // 遍历收到的卡片，过滤出creator等于contact的
        if let Some(recvcards_set) = self.card_recv.get(&account_id) {
            for card_id in recvcards_set.iter() {
                if let Some(card) = self.cards.get(&card_id) {
                    if &card.creator == contact {
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
                        rslt.push(temp_map);
                    }
                } else {
                    env::log("Error!!! Storage State Inconsistency!".as_bytes());
                    continue;
                }
            }
            Some(rslt)
        } else {
            env::log("您还没有收到任何卡片".as_bytes());
            None
        }
    }

}

impl BLCardService {

    // 生成随机
    fn gen_id(&self) -> String {
        let random_seed = env::random_seed();
        let id_str = random_seed
            .iter()
            .map(|&c| format!("{:x?}", c))
            .collect::<String>();
        id_str
    }

    // 交易
    fn transfer(&self, target_account: String, amount: u128) -> bool {
        if 0 < amount {
            Promise::new(target_account).transfer(amount);
            return true;
        }
        
        return false;
    }

    // 计算红包随机比例
    fn random_amount(&self, total_amount: u64) -> u64 {
        let u8_max_value: u64 = u8::max_value().into();
        let block_length = total_amount / u8_max_value;

        let random_seed = env::random_seed();

        // 计算总 seed 值
        let mut block_index = 0_u8;

        for item in random_seed {
            block_index = block_index.wrapping_add(item);
        }

        // TODO 有待检查
        if block_index < 1 {
            block_index += 1;
        } else if block_index > 253 {
            block_index -= 1;
        }

        block_length.wrapping_mul(block_index.into())
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
            random_seed: vec![0, 1, 2, 3, 4, 5, 6],
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

        let _templates = bl_card_service.list_template();

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
            &String::from("template_id"),  // tid
            0,  // card_type
            &String::from("This is public msg"), // public
            &String::from("This is private msg"), // private
            &String::from("This is name"),  // name
            1,  // count
            1,  // amount
            100,  // duration
            true,
            &String::from("Receiver"),
        );
        assert_ne!(create_result, "");
        println!("Create card return: {}", create_result);
        let _cards = bl_card_service.list_card();

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

    }

    #[test]
    fn test_random() {
        let context = get_context(vec![], false);
        testing_env!(context);

        // let random_seed = env::random_seed();
        let random_seed = vec![1u8, 11u8, 34u8, 44u8, 100u8, 145u8, 223u8];
        
        let mut total = 0_u8;

        for item in random_seed {
            total = total.wrapping_add(item);
        }

        let u8_max_value: u8 = u8::max_value().into();
        assert_eq!(46, total);
        total = total.wrapping_mul(100);
        // total = total.wrapping_div(u8_max_value);
        assert_eq!(248, total);

    }
}
