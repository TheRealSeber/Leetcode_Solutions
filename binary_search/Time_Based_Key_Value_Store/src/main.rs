use std::collections::HashMap;

struct TimeMap {
    timelapse: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self {
            timelapse: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.timelapse
            .entry(key)
            .and_modify(|e| e.push((timestamp, value.clone())))
            .or_insert(vec![(timestamp, value)]);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(val) = self.timelapse.get(&key) {
            if timestamp < val[0].0 {
                return "".to_string();
            }
            let mut lp = 0_i32;
            let mut rp = (val.len() - 1) as i32;
            while lp <= rp {
                let mp = (lp + rp) / 2;
                if val[mp as usize].0 == timestamp {
                    return val[mp as usize].1.clone()
                }
                if val[mp as usize].0 < timestamp {
                    lp = mp + 1;
                } else {
                    rp = mp - 1;
                }
            }
            return val[0.max(lp - 1) as usize].1.clone()
        }
        "".to_string()
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
fn main() {
    let mut obj = TimeMap::new();
    obj.set("key".to_string(), "value".to_string(), 1);
    let ret_2: String = obj.get("key".to_string(), 1);
    println!("Hello, world! The result is {}!", ret_2);
}
