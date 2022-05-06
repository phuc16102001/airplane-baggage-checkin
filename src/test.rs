#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{MockedBlockchain};
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "decentrablog.npmrunstart.testnet".to_string(),
            signer_account_id: "alice_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "alice_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 1000000000000000000000000,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn create_post() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Blog::default();
        contract.create_post("This is the title".to_string(), "Lets go Brandon!".to_string());

        //log id
        env::log(format!("Debug here {}", contract.get_post(0).unwrap().get_post_id()).as_bytes());
        
        assert_eq!(
            "This is the title".to_string(),
            contract.get_post(0).unwrap().get_title()
        );
        assert_eq!(
            "Lets go Brandon!".to_string(),
            contract.get_post(0).unwrap().get_body()
        );
        assert_eq!(1, contract.get_total_posts());
        assert_eq!(0, contract.get_post(0).unwrap().get_post_id());

        //test get_user_posts
        let user_posts = contract.get_user_posts("alice_near".to_string());
        assert_eq!(1, user_posts.len());
        assert_eq!(0, user_posts[0].get_post_id());
    }

    #[test]
    fn delete_a_post_then_add_two_posts() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Blog::default();
        contract.create_post("This is the title".to_string(), "Lets go Brandon!".to_string());
        contract.delete_post(0);
        
        assert_eq!(0, contract.get_total_posts(), "Total posts should be 0");

        // add a post
        contract.create_post("This is the title".to_string(), "Lets go Brandon!".to_string());
        contract.create_post("This is the title".to_string(), "Lets go Brandon!".to_string());
        assert_eq!(2, contract.get_total_posts());

        //next post id
        assert_eq!(3, contract.get_next_post_id());
    }

    #[test]
    fn return_owner_account_id() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = Blog::default();
        assert_eq!(
            "alice_near".to_string(),
            contract.get_owner()
        );
    }

    #[test]
    fn create_comment() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Blog::default();

        // Create the first post
        contract.create_post("This is the title".to_string(), "Lets go Brandon!".to_string());
        contract.create_comment(0, "This is the comment".to_string());

        assert_eq!(
            "This is the comment".to_string(),
            contract.get_comment(0).get_body()
        );
        assert_eq!(0, contract.get_comment(0).get_comment_id());

        contract.create_comment(0, "This is comment 2, id 1".to_string());
        contract.create_comment(0, "This is comment 3, id 2".to_string());

        // Check if the comments is there
        assert_eq!(
            "This is comment 2, id 1".to_string(),
            contract.get_comment(1).get_body()
        );
        assert_eq!(
            "This is comment 3, id 2".to_string(),
            contract.get_comment(2).get_body()
        );

        let comments = contract.get_post(0).unwrap().get_comments();

        //assert size of comments
        assert_eq!(3, comments.len(), "Comments size is not 3");
        assert_eq!(3, contract.get_post_total_comments(0), "get_post_total_comments is not 3");

        //Check comment string
        assert_eq!(
            "This is comment 2, id 1".to_string(),
            contract.get_comment(1).get_body()
        );
        assert_eq!(
            "This is comment 3, id 2".to_string(),
            contract.get_comment(2).get_body()
        );
    }

    #[test]
    fn upvote_test() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Blog::default();

        // Create the first post
        contract.create_post("This is the title".to_string(), "Lets go Brandon!".to_string());

        // Upvote the post
        contract.upvote(0);

        // Check if the upvote is there
        assert_eq!(
            1,
            contract.get_post(0).unwrap().get_upvotes().len()
        );

        // upvote 10 times 
        for _ in 0..10 {
            contract.upvote(0);
        }

        // downvote 5 times
        for _ in 0..5 {
            contract.downvote(0);
        }

        // check statistic, 10 times of upvote is 1 in an account
        // 5 times of downvote is the same
        assert_eq!(
            (0, 1),
            contract.get_votes_statistics(0)
        );
        
    }

    #[test]
    fn test_paging_post() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Blog::default();

        // Loop 100 post and create them
        for i in 0..45 {
            contract.create_post(format!("This is the title {}", i), format!("Lets go Brandon! {}", i));
        }

        assert_eq!(45, contract.get_total_posts(), "Total post is not 45");
        
        // Check paging post call
        // let posts = contract.get_paging_posts(1, 10);
        // assert_eq!(10, posts.len(), "Paging post size is not 10");
        // assert_eq!(0, posts[0].get_post_id(), "Paging post id is not 10");
        // assert_eq!(9, posts[9].get_post_id(), "Paging post id is not 20");

        // Check paging post call
        let posts = contract.get_paging_posts(5, 10);

        assert_eq!(5, posts.len(), "Paging post size is not 5");
        assert_eq!(40, posts[0].get_post_id(), "Paging post id is not 40");
        assert_eq!(44, posts[4].get_post_id(), "Paging post id is not 44");
    }

    #[test]
    fn test_donation() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Blog::default();

        // Create the first post
        contract.create_post("This is the title".to_string(), "Lets go Brandon!".to_string());

        // Donate
        contract.donate(0, 1000000, "Support Trump for the USA".to_string());

        // Check if the donation is there
        assert_eq!(
            1,
            contract.get_post(0).unwrap().get_total_donation()
        );
    }
}