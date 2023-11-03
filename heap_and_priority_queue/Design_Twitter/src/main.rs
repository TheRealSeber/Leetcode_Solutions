use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

#[derive(Default)]
struct Twitter {
    users: BTreeMap<i32, BTreeSet<i32>>,
    tweets: VecDeque<(i32, i32)>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Self::default()
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.check_and_create_user_if_not_exist(user_id);
        self.tweets.push_front((user_id, tweet_id));
    }

    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        self.check_and_create_user_if_not_exist(user_id);
        let mut res = Vec::new();
        let following = self.users.get(&user_id).unwrap();
        for (usr_id, tweet_id) in &self.tweets {
            if *usr_id == user_id || following.contains(&usr_id) {
                res.push(*tweet_id);
                if res.len() == 10 {
                    break
                }
            }
        }
        res
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.check_and_create_user_if_not_exist(follower_id);
        self.users.get_mut(&follower_id).unwrap().insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.check_and_create_user_if_not_exist(follower_id);
        self.users.get_mut(&follower_id).unwrap().remove(&followee_id);
    }

    fn check_and_create_user_if_not_exist(&mut self, follower_id: i32) {
        if let None = self.users.get(&follower_id) {
            self.users.insert(follower_id, BTreeSet::new());
        }
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */

fn main() {
    println!("Hello, world!");
}
