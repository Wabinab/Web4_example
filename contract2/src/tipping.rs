use crate::*;


const MIN_TIP_AMOUNT: u128 = 1000000000000000000000;


#[near_bindgen]
impl Contract {
    /// Tipping Module
    /// We keep 5% of the total tip.
    /// No checks for tipping yourself: if you want to send me money, welcome to tip yourself! 
    #[payable]
    pub fn tip(
        &mut self,
        article_id: String,
        owner_id: AccountId
    ) {
        require!(
            env::attached_deposit() >= MIN_TIP_AMOUNT,
            format!(
                "Minimum tip amount: {} N",
                yoctonear_to_near(MIN_TIP_AMOUNT)
            )
        );

        let article_ids = expect_lightweight(
            self.wlog_by_owner.get(&owner_id),
            "This article is not owned by this owner."
        );

        let article_id: u64 = article_id.as_str().parse().unwrap();

        require!(
            article_ids.contains(&article_id),
            "This article is not owned by this owner."
        );

        let mut total_tip = expect_lightweight(
            self.total_tip_by_article.get(article_id), 
            "This article id might not exist. Contact admin."
        );

        let tip_amount_in_yocto = env::attached_deposit();
        let current_tip = yoctonear_to_tip_language(tip_amount_in_yocto);

        total_tip += current_tip;

        self.total_tip_by_article.replace(article_id, &total_tip);

        let tip_to_owner = tip_amount_in_yocto * 95 / 100;

        // Promise transfer money: we keep 5% of the total tip. 
        Promise::new(owner_id).transfer(tip_to_owner);
    }


    pub fn get_tip(&self, article_id: String) -> f64 {
        let article_id: u64 = article_id.as_str().parse().unwrap();

        let total_tip = expect_lightweight(
            self.total_tip_by_article.get(article_id),
            "Cannot find this article."
        );

        tip_language_to_near(total_tip)
    }
}