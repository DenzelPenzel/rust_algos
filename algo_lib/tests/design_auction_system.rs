/*
You are asked to design an auction system that manages bids from multiple users in real time.

Each bid is associated with a userId, an itemId, and a bidAmount.

Implement the AuctionSystem class:​​​​​​​

AuctionSystem(): Initializes the AuctionSystem object.
void addBid(int userId, int itemId, int bidAmount): Adds a new bid for itemId by userId with bidAmount. 
If the same userId already has a bid on itemId, replace it with the new bidAmount.
void updateBid(int userId, int itemId, int newAmount): Updates the existing bid of userId for itemId to newAmount. 
It is guaranteed that this bid exists.
void removeBid(int userId, int itemId): Removes the bid of userId for itemId. It is guaranteed that this bid exists.
int getHighestBidder(int itemId): Returns the userId of the highest bidder for itemId. 
If multiple users have the same highest bidAmount, return the user with the highest userId. If no bids exist for the item, return -1.
 
Example 1:
    Input:
        ["AuctionSystem", "addBid", "addBid", "getHighestBidder", "updateBid", "getHighestBidder", "removeBid", "getHighestBidder", "getHighestBidder"]
        [[], [1, 7, 5], [2, 7, 6], [7], [1, 7, 8], [7], [2, 7], [7], [3]]
    Output:
        [null, null, null, 2, null, 1, null, 1, -1]

    Explanation
        AuctionSystem auctionSystem = new AuctionSystem(); // Initialize the Auction system
        auctionSystem.addBid(1, 7, 5); // User 1 bids 5 on item 7
        auctionSystem.addBid(2, 7, 6); // User 2 bids 6 on item 7
        auctionSystem.getHighestBidder(7); // return 2 as User 2 has the highest bid
        auctionSystem.updateBid(1, 7, 8); // User 1 updates bid to 8 on item 7
        auctionSystem.getHighestBidder(7); // return 1 as User 1 now has the highest bid
        auctionSystem.removeBid(2, 7); // Remove User 2's bid on item 7
        auctionSystem.getHighestBidder(7); // return 1 as User 1 is the current highest bidder
        auctionSystem.getHighestBidder(3); // return -1 as no bids exist for item 3
    
Constraints:
    1 <= userId, itemId <= 5 * 104
    1 <= bidAmount, newAmount <= 109
    At most 5 * 104 total calls to addBid, updateBid, removeBid, and getHighestBidder.
    The input is generated such that for updateBid and removeBid, the bid from the given userId for the given itemId will be valid.
*/

use std::collections::{BTreeMap, BTreeSet, HashMap};

type UserId = i32;
type ItemId = i32;
type Price = i32;

struct AuctionWrapper {
    bids: BTreeMap<Price, BTreeSet<UserId>>,
    user_map: HashMap<UserId, Price>,   
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AuctionWrapper {

    fn new() -> Self {
        Self { bids: BTreeMap::new(), user_map: HashMap::new() }
    }
    
    fn add(&mut self, user_id: UserId, price: Price) {
        if self.user_map.contains_key(&user_id) {
            self.remove(&user_id);
        }

        self.user_map.insert(user_id, price);

        self.bids.entry(price).or_insert_with(BTreeSet::new).insert(user_id);
    }
    
    fn remove(&mut self, user_id: &UserId) {
        if let Some(price) = self.user_map.remove(user_id) {
            if let Some(node) = self.bids.get_mut(&price) {
                node.remove(&user_id);

                if node.is_empty() {
                    self.bids.remove(&price);
                }
            }
        }
    }

    fn get_highest_bidder(&self) -> i32 {
        if let Some((_, users)) = self.bids.last_key_value() {
            if let Some(&max_user) = users.last() {
                return max_user;
            }
        }
        -1
    }
}

struct AuctionSystem {
    auctions: HashMap<ItemId, AuctionWrapper>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AuctionSystem {

    fn new() -> Self {
        Self {
            auctions: HashMap::new(),
        }
    }
    
    fn add_bid(&mut self, user_id: i32, item_id: i32, bid_amount: i32) {
        self.auctions.entry(item_id).or_insert_with(AuctionWrapper::new).add(user_id, bid_amount);
    }
    
    fn update_bid(&mut self, user_id: i32, item_id: i32, new_amount: i32) {
        if let Some(auction) = self.auctions.get_mut(&item_id) {
            auction.add(user_id, new_amount);
        }
    }
    
    fn remove_bid(&mut self, user_id: i32, item_id: i32) {
        if let Some(auction) = self.auctions.get_mut(&item_id) {
            auction.remove(&user_id);
        }
    }
    
    fn get_highest_bidder(&self, item_id: i32) -> i32 {
        match self.auctions.get(&item_id) {
            Some(auction) => auction.get_highest_bidder(),
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auction_flow() {
        let mut system = AuctionSystem::new();
        system.add_bid(1, 7 ,5);
        system.add_bid(2, 7, 6);

        assert_eq!(system.get_highest_bidder(7), 2);

        system.add_bid(3, 7, 6);
        assert_eq!(system.get_highest_bidder(7), 3);

        system.remove_bid(3, 7);
        assert_eq!(system.get_highest_bidder(7), 2);

        system.update_bid(1, 7, 10);
        assert_eq!(system.get_highest_bidder(7), 1);

        assert_eq!(system.get_highest_bidder(99), -1);
    }
}