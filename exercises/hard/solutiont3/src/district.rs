use std::{
    collections::{HashMap, HashSet},
    fmt,
    fs::File,
};

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};

pub fn count_provinces() -> String {
    let file_path = "district.json";
    let file = File::open(file_path).unwrap();

    let data: Batch = serde_json::from_reader(file).unwrap();
    dbg!(&data);
    let mut res = vec![];
    let mut keys: Vec<String> = data.batches.keys().cloned().collect();
    keys.sort();
    keys.into_iter().for_each(|key| {
        let batch = data.batches.get(&key).unwrap();

        let cnt = count_components(&batch);
        res.push(cnt.to_string());
    });

    dbg!(&res);
    res.join(",")
}

#[derive(Debug, Deserialize)]
struct Batch {
    #[serde(flatten)]
    batches: HashMap<String, CityData>,
}

type CityLinks = HashMap<String, HashSet<String>>;

#[derive(Debug)]
struct CityData {
    cities: CityLinks,
}

impl<'de> Deserialize<'de> for CityData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Tell serde to use our custom Visitor to deserialize a map.
        deserializer.deserialize_map(CityDataVisitor)
    }
}

struct CityDataVisitor;

impl<'de> Visitor<'de> for CityDataVisitor {
    type Value = CityData;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
            "a map from city names to list of linked cities (with possible duplicate keys)",
        )
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut cities: HashMap<String, HashSet<String>> = HashMap::new();

        // read every repeated keys and extend every its links.
        while let Some((city, links)) = map.next_entry::<String, Vec<String>>()? {
            cities
                .entry(city)
                .and_modify(|set: &mut HashSet<String>| {
                    set.extend(links.clone());
                })
                .or_insert_with(|| links.into_iter().collect());
        }

        let mut all_cities = HashSet::new();
        for links in cities.values() {
            for city in links.iter() {
                all_cities.insert(city.clone());
            }
        }
        for city in all_cities.drain() {
            cities.entry(city).or_insert_with(HashSet::new);
        }
        for (city, links) in &mut cities {
            links.remove(city);
        }
        Ok(CityData { cities })
    }
}

// fn deserialize_city_data<'de, D>(de: D) -> Result<CityLinks, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let raw_map: serde_json::Map<String, serde_json::Value> = Deserialize::deserialize(de)?;
//     let mut merged = HashMap::new();

//     for (city, links_value) in raw_map {
//         let links: Vec<String> =
//             serde_json::from_value(links_value).map_err(serde::de::Error::custom)?;
//         dbg!(&links);
//         merged
//             .entry(city)
//             .or_insert_with(HashSet::new)
//             .extend(links);
//     }

//     Ok(merged)
// }

fn count_components(cites: &CityData) -> usize {
    let cities = &cites.cities;
    let keys = cities.keys().cloned().collect();
    if cities.is_empty() {
        return 0;
    }
    let mut uf = UnionFind::new(keys);
    for (city, neighbors) in cities {
        for neighbor in neighbors {
            uf.union(city, neighbor);
        }
    }
    let roots: HashSet<String> = cities.keys().map(|city| uf.find(city)).collect();
    roots.len()
}

#[derive(Debug)]
struct UnionFind {
    parent: HashMap<String, String>,
    rank: HashMap<String, usize>,
}

impl UnionFind {
    pub fn new(elms: Vec<String>) -> Self {
        let mut parent = HashMap::new();
        let mut rank = HashMap::new();
        for e in elms {
            parent.insert(e.clone(), e.clone());
            rank.insert(e, 1);
        }
        UnionFind { parent, rank }
    }

    fn find(&mut self, x: &str) -> String {
        if self.parent[x] != x {
            let parent = self.parent[x].clone();
            let root = self.find(&parent);
            self.parent.insert(x.to_string(), root.clone());
            return root;
        }
        x.to_string()
    }

    fn union(&mut self, x: &str, y: &str) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root {
            return;
        }

        if self.rank[&x_root] < self.rank[&y_root] {
            self.parent.insert(x_root.clone(), y_root.clone());
            *self.rank.get_mut(&y_root).unwrap() += self.rank[&x_root];
        } else {
            self.parent.insert(y_root.clone(), x_root.clone());
            *self.rank.get_mut(&x_root).unwrap() += self.rank[&y_root];
        }
    }
}
