//  Didn't get to test this because I'm waiting on my new computer.

struct SelectorAttributes {
    imei: String,
    ipv4: (u8, u8, u8, u8),
    ipv6: String,
    // etc
}

struct Operation {
    team_name: String,
    selectors_concerned_with: Option<SelectorAttributes>,
    mission_capes: Option<Vec<String>>,
}

enum CyberDefenseTeam {
    Israel(Operation),
    Gaza(Operation),
}

struct Hit {
    timestamp: i32,
    count: i32,
}

impl Hit {
    // update, hits, get
    fn new(timestamp: i32) -> Result<Self, String> {
        if timestamp < 0 {
            return Err(format!("{} isnt a valid hit... unless its though a quantum space bridge... hmmm...", timestamp));
        }

        Ok(Self {
            timestamp,
            count: 1,
        })
    }

    fn update(&mut self) {
        self.count += 1;
    }

    fn hits(&self) -> i32 {
        self.count
    }

    fn get(&self) -> i32 {
        self.timestamp
    }
}

struct HitCounter {
    hits: Vec<Hit>,
}


/** 
 * Client can implement their mission areas with the cyber defense team enum... etc...
 * Funny story, I got the CISSP and applied for a CDT with the Navy in 2020... Apparently, time worked in my favor. Hope to share my knowledge with newbie programmers/cyber ops professionals in the future. 
 * Also, I hope to develop cyber physical systems with good Rust design principles in place. Look out for the future!
**/
impl HitCounter {

    fn new() -> Self {
        Self {
            hits: Vec::new(),
        }
    }
    
    /*
        - Vec::last_mut() returns an Option<&mut T>
        - if let combines a match statement with a pattern, it returns the value inside Some else the block is skipped
        - if let Some(last_hit) = self.hits.last_mut() -> gives you a mutable reference if it exists in the last element
    */
    fn hit(&mut self, timestamp: i32) {
        if let Some(last_hit) = self.hits.last_mut() {
            if last_hit.get() == timestamp {
                last_hit.update();
                return;
            }
        }

        self.hits.push(Hit::new(timestamp).unwrap());
    }
    
    fn get_hits(&self, timestamp: i32) -> i32 {
        let mut hits = 0;

        // Option here: for idx in (0..self.length).rev() {
        for item in self.hits.iter().rev() {
            let diff = timestamp - item.get();

            if diff < 300 && diff >= 0 {
                hits += item.hits();
            } else {
                break;
            }
        }

        hits
    }
}

// fn main() later when computer arrives!!
