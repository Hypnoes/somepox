#![allow(unused)]

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    sync::Arc,
};

use anyhow::{anyhow, Result};

use crate::{
    connection::{Connection, Net},
    issue::{Issue, IssueType},
    logbackend::{LogBackend, Queryable, Writable},
    mailbox::{Mail, MailBox},
};

type Address = String;
type AddressBook = HashMap<String, Vec<String>>;

/// Master 负责三个角色
///
/// # 议长：
/// 1. 从 *提议者(Proposer)* 接受 *提案(Proposal)*
/// 2. 将 *提案(Proposal)* 交由所有 *议员(Senator)* *表决(Vote)*
/// 3. 将 *表决(Vote)* 结果收回，*唱票(Counting)*
/// 4. 将投票结果交由 *书记(Secretary)* 记录在案形成最终 *决议(Resolution)*
///
/// # 提案者：
/// 提交 *议案(Proposal)* 至 *议长(President)* ，由 *议长* 添加进待议列表
///
/// # 书记：
/// 将议题 *决定(Resolution)* 写入记录中
///
///
pub struct Master {
    address: Address,
    address_book: AddressBook,
    mail_box: MailBox<String, Issue>,
    vote_table: RefCell<HashMap<Issue, u8>>,
    counter: Cell<u64>,
    logbackend: Box<dyn LogBackend>,
}

impl Master {
    pub fn new(
        address: Address,
        address_book: AddressBook,
        log_backend: Box<dyn LogBackend>,
    ) -> Result<Self> {
        Ok(Self {
            address: address.clone(),
            address_book: address_book,
            mail_box: MailBox::new(Box::new(Net::new(address)?)),
            vote_table: RefCell::new(HashMap::new()),
            counter: Cell::new(0),
            logbackend: log_backend,
        })
    }

    /// 获取当前在线的议员的数量
    ///
    /// **TODO**: 这里获取的是被配置的worker数量，而不是实时在线的数量
    fn senators(&self) -> usize {
        self.address_book
            .get("worker")
            .map(|v| v.len())
            .unwrap_or(0)
    }

    /// 提议新的议题
    pub fn emmit_new_proposal(&self, msg_content: String) -> Result<()> {
        // 为新的议题生成编号
        let issue_id = self.counter.get() + 1;

        // 生成议题
        let issue = Issue::new(msg_content, issue_id, IssueType::Proposal);

        // 在表决表中记录该议题
        self.vote_table.borrow_mut().insert(issue.clone(), 0);

        // 将议题准备下发至议员投票
        // TODO: 错误处理，这里发生错误会被丢弃
        self.address_book.get("worker").map(|addr| {
            let worker_address = addr.join(",");
            let mail = Mail::new(self.address.clone(), vec![worker_address], issue);
            self.mail_box.put_mail(mail);
        });

        // 更新议题编号
        self.counter.set(issue_id);
        Ok(())
    }

    /// 当收到投票时，为对应议案进行计票，如果票数过半，就生成议案交由书记记录
    ///
    /// 当投票未过半，返回Error("not enough votes")
    pub fn process_vote(&self) -> Result<()> {
        let half_of_voters = (self.senators() / 2) as u8;

        if let Ok(issue) = self.mail_box.get_mail().map(|mail| mail.body()) {
            match self.vote_table.borrow().get(&issue) {
                // 此决议正在表决中
                Some(cnt) => {
                    // 表决通过了
                    if cnt + 1 > half_of_voters {
                        self.vote_table.borrow_mut().remove(&issue);
                        self.logbackend
                            .write(issue.id().into(), issue.content().into())
                    }
                    // 表决进行中
                    else {
                        self.vote_table.borrow_mut().insert(issue, cnt + 1);
                        Err(anyhow!("not enough votes"))
                    }
                }
                // 此决议已完成表决，或未有此决议的提案
                None => Err(anyhow!("this proposal is either not emitted or finished")),
            }
        } else {
            Err(anyhow!("Error"))
        }
    }

    pub fn get_log(&self, id: u64) -> Result<String> {
        let query_result = self.logbackend.query(id)?;
        Ok(String::from_utf8(query_result.into())?)
    }
}

/// 议员：
/// 1. 对 *议长(President)* 下发的 *议题(Proposal)* 进行投票
/// 2. 如果当前议题的 *编号(id)* 大于已处理的 *编号(id)* ，同意该提案；否则拒绝
/// 3. 回复投票结果至 *议长(President)*
pub struct Worker {
    address: Address,
    address_book: AddressBook,
    mail_box: MailBox<String, Issue>,
    last_proposal_id: u64,
}

impl Worker {
    pub fn new(address: Address, address_book: AddressBook) -> Result<Self> {
        Ok(Self {
            address: address.clone(),
            address_book: address_book,
            mail_box: MailBox::new(Box::new(Net::new(address)?)),
            last_proposal_id: 0,
        })
    }

    pub fn vote(&self) -> Result<()> {
        if let Ok(issue) = self.mail_box.get_mail().map(|mail| mail.body()) {
            if issue.id() > self.last_proposal_id {
                self.address_book
                    .get("master")
                    .map(|addr| {
                        let worker_address = addr.join(",");
                        let mail = Mail::new(
                            self.address.clone(),
                            vec![worker_address],
                            Issue::new(issue.content(), issue.id(), IssueType::Vote),
                        );
                        self.mail_box.put_mail(mail).ok()
                    })
                    .flatten()
                    .ok_or(anyhow!(
                        "Can not Send Vote to Master. Check address_book setting."
                    ))
            } else {
                Err(anyhow!(
                    "received expire issue {}, last issue is {}, drop.",
                    issue.id(),
                    self.last_proposal_id
                ))
            }
        } else {
            Err(anyhow!("Error"))
        }
    }
}
