/*
You are given an initial list of events, 
where each event has a unique eventId and a priority.

Implement the EventManager class:

EventManager(int[][] events) Initializes the manager with the given events, where events[i] = [eventIdi, priority​​​​​​​i].
void updatePriority(int eventId, int newPriority) Updates the priority of the active event with id eventId to newPriority.
int pollHighest() Removes and returns the eventId of the active event with the highest priority. If multiple active events have the same priority, return the smallest eventId among them. If there are no active events, return -1.
An event is called active if it has not been removed by pollHighest().

Example 1:
    Input:
        ["EventManager", "pollHighest", "updatePriority", "pollHighest", "pollHighest"]
        [[[[5, 7], [2, 7], [9, 4]]], [], [9, 7], [], []]
    Output:
        [null, 2, null, 5, 9]
    Explanation
        EventManager eventManager = new EventManager([[5,7], [2,7], [9,4]]); // Initializes the manager with three events
        eventManager.pollHighest(); // both events 5 and 2 have priority 7, so return the smaller id 2
        eventManager.updatePriority(9, 7); // event 9 now has priority 7
        eventManager.pollHighest(); // remaining highest priority events are 5 and 9, return 5
        eventManager.pollHighest(); // return 9

Example 2:
    Input:
        ["EventManager", "pollHighest", "pollHighest", "pollHighest"]
        [[[[4, 1], [7, 2]]], [], [], []]
    Output:
        [null, 7, 4, -1]
    Explanation
        EventManager eventManager = new EventManager([[4,1], [7,2]]); // Initializes the manager with two events
        eventManager.pollHighest(); // return 7
        eventManager.pollHighest(); // return 4
        eventManager.pollHighest(); // no events remain, return -1
    
Constraints:
    1 <= events.length <= 105
    events[i] = [eventId, priority]
    1 <= eventId <= 109
    1 <= priority <= 109
    All the values of eventId in events are unique.
    1 <= newPriority <= 109
    For every call to updatePriority, eventId refers to an active event.
    At most 105 calls in total will be made to updatePriority and pollHighest.
*/


use std::collections::{BTreeMap, BTreeSet, HashMap};

struct EventManager {
    events: BTreeMap<i32, BTreeSet<i32>>,
    event_map: HashMap<i32, i32> // event -> score
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl EventManager {

    fn new(events: Vec<Vec<i32>>) -> Self {
        let mut obj = Self {
            events: BTreeMap::new(),
            event_map: HashMap::new()
        };

        for e in events {
            let id = e[0];
            let priority = e[1];

            obj.event_map.insert(id, priority);
            obj.events.entry(priority).or_insert_with(BTreeSet::new).insert(id); 
        }

        obj
    }
    
    fn update_priority(&mut self, event_id: i32, new_priority: i32) {
        if let Some(&old_priority) = self.event_map.get(&event_id) {
            if old_priority == new_priority {
                return;
            }
            
            self.event_map.insert(event_id, new_priority);
            
            if let Some(node) = self.events.get_mut(&old_priority) {
                node.remove(&event_id);
                if node.is_empty() {
                    self.events.remove(&old_priority);
                }
            }

            self.events.entry(new_priority)
                .or_insert_with(BTreeSet::new)
                .insert(event_id);
        }
    }
    
    fn poll_highest(&mut self) -> i32 {
        let highest_priority = match self.events.keys().last() {
            Some(&p) => p,
            None => -1,
        };

        let min_event_id = self.events.get_mut(&highest_priority).and_then(|set| set.pop_first()).unwrap_or(-1);

        if min_event_id == -1 {
            self.events.remove(&highest_priority);
            return -1;
        }

        if self.events.get(&highest_priority).unwrap().is_empty() {
            self.events.remove(&highest_priority);
        }

        self.event_map.remove(&min_event_id);

        min_event_id
    }
}
